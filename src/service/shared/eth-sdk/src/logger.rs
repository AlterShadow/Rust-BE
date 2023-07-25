use eyre::*;
use lib::types::H256;
use once_cell::sync::Lazy;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

pub struct BlockchainLogger {
    enabled: AtomicBool,
    appender: Mutex<tracing_appender::rolling::RollingFileAppender>,
}
impl BlockchainLogger {
    pub fn new() -> Result<Self> {
        std::fs::create_dir_all("log")?;
        let appender = tracing_appender::rolling::hourly("log", "transaction.log");
        Ok(Self {
            enabled: Default::default(),
            appender: Mutex::new(appender),
        })
    }
    pub fn log(&self, text: impl AsRef<str>, transaction_hash: H256) {
        let time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let text = text.as_ref();
        match self.appender.lock().unwrap().write_fmt(format_args!(
            "[TX] [{time}] [{transaction_hash:?}] {text}\n"
        )) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
}
static BLOCKCHAIN_LOGGER: Lazy<BlockchainLogger> = Lazy::new(|| BlockchainLogger::new().unwrap());

pub fn get_blockchain_logger() -> &'static BlockchainLogger {
    &BLOCKCHAIN_LOGGER
}
