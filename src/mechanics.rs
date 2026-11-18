//! Tuning values for gesture recognition and window snapping, in one place so they are easy
//! to adjust. Trackpad positions and distances are fractions of the trackpad (0..1).

// ---- Trackpad taps and swipes ----
/// Longest touch (s) that still counts as a 1- or 2-finger tap.
pub const TAP_TIME_1_2: f64 = 0.18;
/// Longest touch (s) that still counts as a 3-finger tap.
pub const TAP_TIME_3: f64 = 0.33;
/// Longest touch (s) that still counts as a 4-finger tap.
pub const TAP_TIME_4: f64 = 0.33;
/// Longest touch (s) that still counts as a 5-finger tap.
pub const TAP_TIME_5: f64 = 0.33;
/// Window (s) for all the fingers of a gesture to land.
pub const FINGER_LAND_WINDOW: f64 = 0.30;
/// Trackpad distance a 3-finger swipe must travel.
pub const SWIPE_DISTANCE_3: f32 = 0.3;
/// Trackpad distance a 4-finger swipe must travel.
pub const SWIPE_DISTANCE_4: f32 = 0.4;
/// Trackpad distance a 5-finger swipe must travel.
pub const SWIPE_DISTANCE_5: f32 = 0.4;
/// TipTap: furthest the tapping finger may be from the resting finger.
pub const TIPTAP_MAX_DISTANCE: f32 = 0.55;
/// TipTap: closest it may be.
pub const TIPTAP_MIN_SPREAD: f32 = 0.03;
/// Where "left" ends and "right" begins on the trackpad.
pub const LEFT_RIGHT_SEPARATOR: f32 = 0.5;
/// Movement a tap may have before it is a drag.
pub const TAP_MAX_MOVE: f32 = 0.04;
/// Time between the two taps of a double tap if the system double-click interval can't be
/// read (MacControl normally uses that system value; 0.5 s is macOS's own default).
pub const DOUBLE_TAP_FALLBACK: f64 = 0.5;
/// A system double-click interval below this is treated as unreadable.
pub const DOUBLE_TAP_MIN: f64 = 0.01;
/// Time a finger must rest before another finger "tip-taps".
pub const TIPTAP_REST_TIME: f64 = 0.15;
/// Size of the corner regions for 1-finger corner taps.
pub const CORNER_TAP_REGION: f32 = 0.2;

// ---- Window snapping defaults ----
/// Snapping is on.
pub const SNAP_ENABLED: bool = true;
/// Snap to the left, right and top edges.
pub const SNAP_LEFT: bool = true;
pub const SNAP_RIGHT: bool = true;
pub const SNAP_TOP: bool = true;
/// Snap to the corners.
pub const SNAP_CORNERS: bool = true;
/// How far (points) from a corner, along an edge, the corner zone reaches.
pub const CORNER_AREA: f32 = 30.0;
/// Extra width (points) added to the edge strips that start a snap.
pub const RECOGNITION_AREA_EXTRA: f32 = 0.0;
/// Width (points) of the edge strips: narrow with one display, wider with several so the
/// cursor can still cross from one display to the next.
pub const EDGE_SINGLE_DISPLAY: f32 = 2.0;
pub const EDGE_MULTI_DISPLAY: f32 = 10.0;
/// Points a window must move before it counts as dragged.
pub const MOVE_THRESHOLD: f32 = 2.0;
/// Share of the screen a window takes when snapped left or right.
pub const LEFT_WIDTH: f32 = 0.5;
pub const RIGHT_WIDTH: f32 = 0.5;
/// Share of the screen width a window takes when snapped to a corner.
pub const CORNER_WIDTH: f32 = 0.5;
/// Restore the old size when a snapped window is dragged away.
pub const SNAP_BACK: bool = true;
/// Gap (points) around snapped windows.
pub const GAP: f32 = 0.0;
/// Border width of the preview overlay.
pub const PREVIEW_BORDER: f32 = 4.0;
/// The preview overlay has rounded corners…
pub const PREVIEW_ROUNDED: bool = true;
/// …with this radius.
pub const PREVIEW_RADIUS: f32 = 40.0;
/// Duration (s) of the preview's fade and resize.
pub const PREVIEW_ANIMATION: f32 = 0.26;
/// Optional delay (s) before a snap preview appears.
pub const SNAP_DELAY_ENABLED: bool = false;
pub const SNAP_DELAY: f32 = 0.1;

// ---- Placing windows ----
/// A window dragged out of a snap gets its old size back and is placed with x = cursor.x +
/// old_width / -1.5…
pub const SNAP_BACK_X_DIVISOR: f32 = -1.5;
/// …and y = cursor.y - 8, so its title bar stays under the cursor; it is then kept on screen.
pub const SNAP_BACK_Y_OFFSET: f32 = 8.0;
/// A window whose size ends up more than this many points off the requested one (it has a
/// minimum size) is moved again so it still lines up with the side it was sent to.
pub const SIZE_TOLERANCE: f32 = 8.0;
/// The Dock counts as touching a screen edge when it is within this many points of it (used
/// for the optional gap above the Dock).
pub const DOCK_EDGE_TOLERANCE: f32 = 10.0;

// ---- One-finger swipes in from an edge of the trackpad. y grows upwards. ----
/// A swipe arms when the finger lands within 5 % of the left edge…
pub const EDGE_LEFT_START: f32 = 0.05;
/// …or beyond 95 % towards the right edge…
pub const EDGE_RIGHT_START: f32 = 0.95;
/// …or within 10 % of the bottom / beyond 90 % at the top.
pub const EDGE_BOTTOM_START: f32 = 0.10;
pub const EDGE_TOP_START: f32 = 0.90;
/// It fires once the finger has moved in past 0.25 from the left or 0.75 from the right…
pub const EDGE_LEFT_FIRE: f32 = 0.25;
pub const EDGE_RIGHT_FIRE: f32 = 0.75;
/// …or past 0.3 from the bottom / 0.7 from the top.
pub const EDGE_BOTTOM_FIRE: f32 = 0.30;
pub const EDGE_TOP_FIRE: f32 = 0.70;
/// While armed, the finger must stay within 0.8 of the other axis.
pub const EDGE_CROSS_LIMIT: f32 = 0.80;
