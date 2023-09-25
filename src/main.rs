#[allow(warnings, dead_code)]
use kengen::{
    logger::{LogLevel, Logger},
    Game,
};
use std::env;

fn main() {
    handle_readline_args(env::args().collect::<Vec<String>>());

    let mut game = Game::new().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    game.run();
    game.destroy();
}

fn handle_readline_args(args: Vec<String>) {
    let mut iter = args.iter();
    let mut log_level: Option<&str> = None;
    let mut log_output: Option<&str> = None;

    while let Some(arg) = iter.next() {
        if arg == "-l" || arg == "--loglevel" {
            log_level = iter.next().map(|s| s.as_str());
        } else if arg == "-o" || arg == "--logoutput" {
            log_output = iter.next().map(|s| s.as_str());
        }
    }

    let log_level = match log_level {
        Some("debug") => Some(LogLevel::Debug),
        Some("info") => Some(LogLevel::Info),
        Some("warning") => Some(LogLevel::Warning),
        Some("error") => Some(LogLevel::Error),
        Some("critical") => Some(LogLevel::Critical),
        None => None,
        _ => {
            println!("Improperly formed loglevel option");
            None
        }
    };

    Logger::new(log_level, log_output);
    Logger::dbg("debug");
    Logger::info("debug");
    Logger::warn("debug");
    Logger::err("debug");
    Logger::crit("debug");
}
