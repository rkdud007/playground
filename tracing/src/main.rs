use once_cell::sync::Lazy;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tracing::{debug, error, info, warn};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::*;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum LoggingTopic {
    EventScraper,
    EventApplier,
    System,
}

impl LoggingTopic {
    fn as_str(&self) -> &'static str {
        match self {
            LoggingTopic::EventScraper => "EVENT_SCRAPER",
            LoggingTopic::EventApplier => "EVENT_APPLIER",
            LoggingTopic::System => "SYSTEM",
        }
    }
}

static LOGGERS: Lazy<Mutex<HashMap<LoggingTopic, Arc<Logger>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct Logger {
    topic: LoggingTopic,
    enabled: bool,
}

impl Logger {
    // Initializes the global logger subscriber
    pub fn init() {
        // let logfile = tracing_appender::rolling::hourly("./logs", "cron-logs");
        // // Log `INFO` and above to stdout.
        // let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

        // tracing_subscriber::fmt()
        //     .with_writer(stdout.and(logfile))
        //     .init();

        registry()
            // first layer for console output, use pretty formatter and level filter
            .with(
                fmt::layer()
                    .pretty()
                    .with_filter(filter::LevelFilter::from(tracing::Level::INFO)),
            )
            // second layer for log file appender, use json formatter, no filter
            .with(
                fmt::layer()
                    .json()
                    .flatten_event(true)
                    .with_writer(tracing_appender::rolling::hourly("./logs", "cron-logs"))
                    .with_filter(filter::LevelFilter::from(tracing::Level::INFO)),
            )
            .init();
    }

    // Creates a new logger instance
    pub fn new(topic: LoggingTopic) -> Arc<Logger> {
        Arc::new(Logger {
            topic,
            enabled: true,
        })
    }

    // Retrieves an existing logger instance or creates a new one
    pub fn get_instance(topic: LoggingTopic) -> Arc<Logger> {
        let mut loggers = LOGGERS.lock().unwrap();
        loggers
            .entry(topic)
            .or_insert_with(|| Logger::new(topic))
            .clone()
    }

    // Logging methods
    pub fn log(&self, message: &str) {
        if self.enabled {
            info!(topic = %self.topic.as_str(), "{}", message);
        }
    }

    fn error(&self, message: &str) {
        if self.enabled {
            error!(topic = %self.topic.as_str(), "{}", message);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.enabled {
            warn!(topic = %self.topic.as_str(), "{}", message);
        }
    }

    fn debug(&self, message: &str) {
        if self.enabled {
            debug!(topic = %self.topic.as_str(), "{}", message);
        }
    }

    // Method to disable the logger
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[tokio::main]
async fn main() {
    Logger::init();
    for element in 0..10 {
        let logger = Logger::get_instance(LoggingTopic::EventScraper);
        logger.log(&format!("{} Hello, world!", element));
    }
    while (true) {
        let logger = Logger::get_instance(LoggingTopic::EventScraper);
        logger.log(&format!(" Hello, world!"));

        thread::sleep(Duration::from_secs(5));
    }
}
