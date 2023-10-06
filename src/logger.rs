use lazy_static::lazy_static;
use std::sync::Mutex;

// extern crate chrono;

use chrono::prelude::*;
// use std::fs::OpenOptions;
// use std::io::prelude::*;

#[derive(Clone, Copy)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

enum LoggerTarget {
    File(String),
    Console,
}

pub struct Logger {
    target: LoggerTarget,
}

lazy_static! {
    static ref GLOBAL_LOG_LEVEL: Mutex<LogLevel> = Mutex::new(LogLevel::Debug);
}

impl Logger {
    pub fn new(level: LogLevel, target: Option<&str>) -> Self {
        // In this pattern, you're effectively using the Logger::new as a way to
        // optionally configure the logging behavior. If it's never called, the
        // system simply proceeds with defaults.
        // see lazy_static!

        *GLOBAL_LOG_LEVEL.lock().unwrap() = level;
        let target = match target {
            Some(path) => LoggerTarget::File(path.to_string()),
            None => LoggerTarget::Console,
        };
        Self { target }
    }

    fn log(level: LogLevel, msg: &str) {
        let global_level = GLOBAL_LOG_LEVEL.lock().unwrap();

        let formatted_msg = Self::format_msg(level, &get_time_date_string(), msg);

        if *global_level as u8 <= level as u8 {
            println!("{}", formatted_msg);

            // match level {
            //     LogLevel::Debug => println!("{}", formatted_msg),
            //     LogLevel::Info => println!("{}", formatted_msg),
            //     LogLevel::Warning => println!("{}", formatted_msg),
            //     LogLevel::Error => println!("{}}", formatted_msg),
            //     LogLevel::Critical => println!("{}", formatted_msg),
            // }
        }
    }

    pub fn dbg(msg: &str) {
        Self::log(LogLevel::Debug, msg);
    }

    pub fn info(msg: &str) {
        Self::log(LogLevel::Info, msg);
    }

    pub fn warn(msg: &str) {
        Self::log(LogLevel::Warning, msg);
    }

    pub fn err(msg: &str) {
        Self::log(LogLevel::Error, msg);
    }

    pub fn crit(msg: &str) {
        Self::log(LogLevel::Critical, msg);
    }

    fn format_msg(level: LogLevel, time: &str, msg: &str) -> String {
        let level_str = match level {
            LogLevel::Debug => "DBG",
            LogLevel::Info => "INF",
            LogLevel::Warning => "WRN",
            LogLevel::Error => "ERR",
            LogLevel::Critical => "CRT",
        };
        format!("{} [{}] {}", level_str, time, msg)
    }
}

pub fn get_time_date_string() -> String {
    let now = Local::now();
    now.format("%y/%m/%d %H:%M:%S").to_string()
}
