mod a001_top25;
mod a002_immutable_tokens;
mod a003_tokens_no_more_than_10_percent;

pub use a001_top25::*;
pub use a002_immutable_tokens::*;
pub use a003_tokens_no_more_than_10_percent::*;

use eyre::*;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct AuditRule {
    pub id: i32,
    pub name: &'static str,
    pub description: &'static str,
}

pub fn get_audit_rules() -> &'static [AuditRule] {
    &[
        AUDIT_TOP25_TOKENS,
        AUDIT_IMMUTABLE_TOKENS,
        AUDIT_TOKENS_NO_MORE_THAN_10_PERCENT,
    ]
}
#[derive(Clone)]
pub struct AuditLogger {
    appender: Arc<Mutex<tracing_appender::rolling::RollingFileAppender>>,
}
impl AuditLogger {
    pub fn new(prefix: &str) -> Result<Self> {
        std::fs::create_dir_all("log")?;
        let appender = tracing_appender::rolling::hourly("log", format!("{}.log", prefix));
        Ok(Self {
            appender: Arc::new(Mutex::new(appender)),
        })
    }
    pub fn log(&mut self, rule: AuditRule, text: &str) -> Result<()> {
        let time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S");
        let rule = rule.name;
        self.appender
            .lock()
            .unwrap()
            .write_fmt(format_args!("[AUDIT] [{time}] [{rule}] {text}"))?;
        Ok(())
    }
}
