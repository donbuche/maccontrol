//! Turns raw trackpad touch frames into gestures. Pure logic, no system calls.
//! Positions are normalised 0..1, with y growing upwards (as the trackpad reports it).

use crate::mechanics as m;
use crate::model::{Corner, Dir, Gesture, Side};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Touch {
    pub id: i32,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
struct Finger {
    start: (f32, f32),
    pos: (f32, f32),
    since: f64,
}

#[derive(Debug)]
struct Sequence {
    start_t: f64,
    max_n: usize,
    fingers: HashMap<i32, Finger>,
    moved: bool,
    swiped: bool,
    consumed: bool,
    /// Centroid when the current finger count was reached.
    base: Option<(usize, (f32, f32), f64)>,
    first_pos: (f32, f32),
    /// (resting finger, tapping finger, time it landed)
    tiptap: Option<(i32, i32, f64)>,
}

pub fn tap_time(n: usize) -> f64 {
    match n {
        0..=2 => m::TAP_TIME_1_2,
        3 => m::TAP_TIME_3,
        4 => m::TAP_TIME_4,
        _ => m::TAP_TIME_5,
    }
}

pub fn swipe_distance(n: usize) -> f32 {
    match n {
        0..=3 => m::SWIPE_DISTANCE_3,
        4 => m::SWIPE_DISTANCE_4,
        _ => m::SWIPE_DISTANCE_5,
    }
}

#[derive(Default)]
pub struct Recognizer {
    seq: Option<Sequence>,
}

impl Recognizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// A physical click happened during this touch (it was handled as an N-finger click),
    /// so the touch must not also be reported as a tap.
    pub fn mark_consumed(&mut self) {
        if let Some(s) = &mut self.seq {
            s.consumed = true;
        }
    }

    pub fn feed(&mut self, touches: &[Touch], t: f64) -> Vec<Gesture> {
        let mut out = vec![];
        let n = touches.len();
        if self.seq.is_none() {
            if n == 0 {
                return out;
            }
            let c = centroid(touches);
            self.seq = Some(Sequence {
                start_t: t,
                max_n: 0,
                fingers: HashMap::new(),
                moved: false,
                swiped: false,
                consumed: false,
                base: None,
                first_pos: c,
                tiptap: None,
            });
        }
        let s = self.seq.as_mut().unwrap();

        // Fingers that lifted.
        let present: Vec<i32> = touches.iter().map(|t| t.id).collect();
        let lifted: Vec<i32> = s.fingers.keys().copied().filter(|id| !present.contains(id)).collect();
        for id in lifted {
            // A tip-tap finishes when the tapping finger lifts while the resting one stays.
            if let Some((rest, tap, landed)) = s.tiptap {
                if id == tap && present.contains(&rest) && !s.swiped && !s.consumed {
                    if t - landed <= m::TAP_TIME_1_2 {
                        let f = s.fingers[&tap];
                        let r = s.fingers[&rest];
                        let moved = d(f.start, f.pos) > m::TAP_MAX_MOVE;
                        if !moved {
                            out.push(Gesture::TipTap(if f.pos.0 < r.pos.0 { Side::Left } else { Side::Right }));
                            s.consumed = true;
                        }
                    }
                    s.tiptap = None;
                }
            }
            s.fingers.remove(&id);
        }
        // Fingers that landed or moved.
        for tc in touches {
            let e = s.fingers.entry(tc.id).or_insert(Finger { start: (tc.x, tc.y), pos: (tc.x, tc.y), since: t });
            e.pos = (tc.x, tc.y);
            if d(e.start, e.pos) > m::TAP_MAX_MOVE {
                s.moved = true;
            }
        }
        // A second finger landing next to one that has been resting still: tip-tap candidate.
        if n == 2 && s.tiptap.is_none() && s.max_n <= 1 {
            let mut ids: Vec<i32> = s.fingers.keys().copied().collect();
            ids.sort_by(|a, b| s.fingers[a].since.partial_cmp(&s.fingers[b].since).unwrap());
            let (rest, tap) = (ids[0], ids[1]);
            let r = s.fingers[&rest];
            let f = s.fingers[&tap];
            let spread = d(r.pos, f.pos);
            if f.since - r.since >= m::TIPTAP_REST_TIME
                && d(r.start, r.pos) <= m::TAP_MAX_MOVE
                && (m::TIPTAP_MIN_SPREAD..=m::TIPTAP_MAX_DISTANCE).contains(&spread)
            {
                s.tiptap = Some((rest, tap, f.since));
            }
        }
        if n > s.max_n {
            s.max_n = n;
        }

        // Swipes with 3+ fingers: measure from where the current finger count was reached.
        if n >= 3 && !s.swiped && !s.consumed {
            let c = centroid(touches);
            match s.base {
                Some((bn, _, _)) if bn == n => {}
                _ => s.base = Some((n, c, t)),
            }
            let (_, b, _) = s.base.unwrap();
            let dx = c.0 - b.0;
            let dy = c.1 - b.1;
            if dx.abs().max(dy.abs()) >= swipe_distance(n) {
                let dir = if dx.abs() > dy.abs() {
                    if dx > 0.0 { Dir::Right } else { Dir::Left }
                } else if dy > 0.0 {
                    Dir::Up
                } else {
                    Dir::Down
                };
                out.push(Gesture::Swipe(n as u8, dir));
                s.swiped = true;
            }
        } else if n < 3 {
            s.base = None;
        }

        // One finger sliding in from an edge.
        if n == 1 && s.max_n == 1 && !s.swiped && !s.consumed {
            let (sx, sy) = s.first_pos;
            let (x, y) = (touches[0].x, touches[0].y);
            let edge = if sx <= m::EDGE_LEFT_START && x >= m::EDGE_LEFT_FIRE && y <= m::EDGE_CROSS_LIMIT {
                Some(Dir::Left)
            } else if sx >= m::EDGE_RIGHT_START && x <= m::EDGE_RIGHT_FIRE && y <= m::EDGE_CROSS_LIMIT {
                Some(Dir::Right)
            } else if sy <= m::EDGE_BOTTOM_START && y >= m::EDGE_BOTTOM_FIRE {
                Some(Dir::Down)
            } else if sy >= m::EDGE_TOP_START && y <= m::EDGE_TOP_FIRE {
                Some(Dir::Up)
            } else {
                None
            };
            if let Some(d) = edge {
                out.push(Gesture::EdgeSwipe(d));
                s.swiped = true;
            }
        }

        // Everything lifted: maybe it was a tap.
        if n == 0 {
            let seq = self.seq.take().unwrap();
            let dur = t - seq.start_t;
            if !seq.moved && !seq.swiped && !seq.consumed {
                if seq.max_n == 1 {
                    if dur <= m::TAP_TIME_1_2 {
                        if let Some(c) = corner_of(seq.first_pos) {
                            out.push(Gesture::CornerTap(c));
                        }
                    }
                } else if seq.max_n >= 2 && dur <= tap_time(seq.max_n) {
                    out.push(Gesture::Tap(seq.max_n.min(5) as u8));
                }
            }
        }
        out
    }
}

fn corner_of(p: (f32, f32)) -> Option<Corner> {
    let r = m::CORNER_TAP_REGION;
    let left = p.0 <= r;
    let right = p.0 >= 1.0 - r;
    let top = p.1 >= 1.0 - r;
    let bottom = p.1 <= r;
    match (left, right, top, bottom) {
        (true, _, true, _) => Some(Corner::TopLeft),
        (_, true, true, _) => Some(Corner::TopRight),
        (true, _, _, true) => Some(Corner::BottomLeft),
        (_, true, _, true) => Some(Corner::BottomRight),
        _ => None,
    }
}

fn centroid(t: &[Touch]) -> (f32, f32) {
    let n = t.len().max(1) as f32;
    (t.iter().map(|t| t.x).sum::<f32>() / n, t.iter().map(|t| t.y).sum::<f32>() / n)
}

fn d(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tc(id: i32, x: f32, y: f32) -> Touch {
        Touch { id, x, y }
    }
    #[test]
    fn three_finger_tap() {
        let mut r = Recognizer::new();
        let f = [tc(1, 0.4, 0.5), tc(2, 0.5, 0.5), tc(3, 0.6, 0.5)];
        assert!(r.feed(&f, 0.0).is_empty());
        assert!(r.feed(&f, 0.1).is_empty());
        assert_eq!(r.feed(&[], 0.15), vec![Gesture::Tap(3)]);
    }
    #[test]
    fn slow_touch_is_not_a_tap() {
        let mut r = Recognizer::new();
        let f = [tc(1, 0.4, 0.5), tc(2, 0.5, 0.5), tc(3, 0.6, 0.5)];
        r.feed(&f, 0.0);
        assert!(r.feed(&[], 0.6).is_empty());
    }
    #[test]
    fn four_finger_swipe_up() {
        let mut r = Recognizer::new();
        let mut got = vec![];
        for i in 0..10 {
            let y = 0.2 + i as f32 * 0.06;
            let f = [tc(1, 0.3, y), tc(2, 0.4, y), tc(3, 0.5, y), tc(4, 0.6, y)];
            got.extend(r.feed(&f, i as f64 * 0.016));
        }
        got.extend(r.feed(&[], 0.2));
        assert_eq!(got, vec![Gesture::Swipe(4, Dir::Up)]);
    }
    #[test]
    fn tiptap_right() {
        let mut r = Recognizer::new();
        r.feed(&[tc(1, 0.4, 0.4)], 0.0);
        r.feed(&[tc(1, 0.4, 0.4)], 0.3);
        r.feed(&[tc(1, 0.4, 0.4), tc(2, 0.6, 0.4)], 0.35);
        let g = r.feed(&[tc(1, 0.4, 0.4)], 0.42);
        assert_eq!(g, vec![Gesture::TipTap(Side::Right)]);
        assert!(r.feed(&[], 0.8).is_empty());
    }
    #[test]
    fn swipe_in_from_left_edge() {
        let mut r = Recognizer::new();
        let mut got = vec![];
        for i in 0..8 {
            got.extend(r.feed(&[tc(1, 0.02 + i as f32 * 0.05, 0.5)], i as f64 * 0.016));
        }
        got.extend(r.feed(&[], 0.2));
        assert_eq!(got, vec![Gesture::EdgeSwipe(Dir::Left)]);
    }
    #[test]
    fn corner_tap() {
        let mut r = Recognizer::new();
        r.feed(&[tc(1, 0.95, 0.95)], 0.0);
        assert_eq!(r.feed(&[], 0.1), vec![Gesture::CornerTap(Corner::TopRight)]);
    }
}
