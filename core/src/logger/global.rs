use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use super::message::{LogLevel, LogMessage};
use super::worker::LoggerWorker;

static LOGGER_TX: LazyLock<UnboundedSender<LogMessage>> = LazyLock::new(|| {
    let (tx, rx) = unbounded_channel();

    // Spawn logging worker exactly once
    tokio::spawn(async move {
        let worker = LoggerWorker::new(rx);
        worker.run().await;
    });

    tx
});

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn send(level: LogLevel, msg: &str) {
    let _ = LOGGER_TX.send(LogMessage {
        level,
        message: msg.to_string(),
        timestamp: now_ms(),
    });
}

/* ===== Public API ===== */

pub fn info(msg: &str) {
    send(LogLevel::Info, msg);
}

pub fn warn(msg: &str) {
    send(LogLevel::Warn, msg);
}

pub fn error(msg: &str) {
    send(LogLevel::Error, msg);
}

pub fn success(msg: &str) {
    send(LogLevel::Success, msg);
}
