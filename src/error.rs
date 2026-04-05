use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn reset_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub fn error(line: i32, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: i32, error: String, message: String) {
    eprintln!("[line {line}] Error {error}: {message}");
    HAD_ERROR.store(true, Ordering::Relaxed);
}