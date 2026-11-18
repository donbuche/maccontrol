//! A small diagnostic log of what MacControl did, for tracking down rules that don't work in a
//! particular app: ~/Library/Logs/MacControl.log. It never records ordinary typing, only the
//! triggers that matched a rule and what happened to the window they acted on.

use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// The log starts again once it passes this size.
const MAX_BYTES: u64 = 256 * 1024;

static LOCK: Mutex<()> = Mutex::new(());

pub fn path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("Library/Logs/MacControl.log")
}

pub fn write(line: impl AsRef<str>) {
    let _guard = LOCK.lock();
    let p = path();
    if std::fs::metadata(&p).map(|m| m.len() > MAX_BYTES).unwrap_or(false) {
        let _ = std::fs::remove_file(&p);
    }
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "[{secs:.3}] {}", line.as_ref());
    }
}
