use std::sync::atomic::{AtomicBool, Ordering};

use crate::lexer::Token;

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub enum RuntimeError {
    Syntax(SyntaxError),
    IO(std::io::Error),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub token: Token,
    pub message: String,
}

impl From<std::io::Error> for RuntimeError {
    fn from(error: std::io::Error) -> Self {
        RuntimeError::IO(error)
    }
}

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

pub fn runtime_error(error: SyntaxError) {
    eprintln!("{}\n[line {}]", error.token.line, error.message);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}
