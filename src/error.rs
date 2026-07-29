use std::sync::atomic::{AtomicBool, Ordering};

use crate::interpreter::RuntimeError;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn reset_error() {
    HAD_ERROR.store(false, Ordering::Relaxed);
}

pub fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

pub fn reset_runtime_error() {
    HAD_RUNTIME_ERROR.store(false, Ordering::Relaxed);
}

pub fn had_runtime_error() -> bool {
    HAD_RUNTIME_ERROR.load(Ordering::Relaxed)
}

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: usize, error: String, message: String) {
    eprintln!("[line {line}] Error {error}: {message}");
    HAD_ERROR.store(true, Ordering::Relaxed);
}

pub fn runtime_error(error: RuntimeError) {
    eprintln!("{}\n[line {}]", error.token.line, error.message);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}
