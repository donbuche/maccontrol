//! The short click "bip" played when clicking inside MacControl's window.
//! Sound: "audio-volume-change" from the freedesktop.org sound theme (CC-BY-SA,
//! Lucas McCallister), see assets/sounds/CREDITS.txt. Played with rodio.

use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const CLICK: &[u8] = include_bytes!("../assets/sounds/click.wav");

static TX: Mutex<Option<Sender<()>>> = Mutex::new(None);

/// Open the sound output once, on its own thread.
pub fn init() {
    let (tx, rx) = channel::<()>();
    *TX.lock().unwrap() = Some(tx);
    std::thread::Builder::new()
        .name("sound".into())
        .spawn(move || {
            let sink = match rodio::DeviceSinkBuilder::open_default_sink() {
                Ok(mut s) => {
                    s.log_on_drop(false);
                    s
                }
                Err(_) => {
                    while rx.recv().is_ok() {}
                    return;
                }
            };
            let mut last = Instant::now() - Duration::from_secs(1);
            while rx.recv().is_ok() {
                if last.elapsed() < Duration::from_millis(45) {
                    continue;
                }
                last = Instant::now();
                if let Ok(src) = rodio::Decoder::new(Cursor::new(CLICK)) {
                    use rodio::Source;
                    sink.mixer().add(src.amplify(0.55));
                }
            }
        })
        .ok();
}

pub fn click() {
    if let Ok(tx) = TX.lock() {
        if let Some(tx) = tx.as_ref() {
            let _ = tx.send(());
        }
    }
}
