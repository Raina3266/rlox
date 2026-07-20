use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn reset_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: usize, error: String, message: String) {
    eprintln!("[line {line}] Error {error}: {message}");
    HAD_ERROR.store(true, Ordering::Relaxed);
}