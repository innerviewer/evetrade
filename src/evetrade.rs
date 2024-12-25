use chrono::Local;
use env_logger::Builder;
use log::{debug, error, info, trace, warn, Level, LevelFilter};
use std::io::Write;

use crate::urls;

pub struct Evetrade {
    is_initialized: bool,
}

impl Evetrade {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
        }
    }

    pub fn init(&mut self) {
        Builder::new()
            .format(|buf, record| {
                let now = Local::now();
                let time = now.format("%H:%M:%S"); // HOUR:MINUTE:SECOND

                let millis_full = now.format("%.3f").to_string();
                let millis = millis_full.trim_start_matches('.');

                let mut time_style = buf.style();
                time_style
                    .set_color(env_logger::fmt::Color::Blue)
                    .set_bold(true);

                let mut level_style = buf.style();
                match record.level() {
                    Level::Error => level_style
                        .set_color(env_logger::fmt::Color::Red)
                        .set_bold(true)
                        .set_bg(env_logger::fmt::Color::Black)
                        .set_bold(true),
                    Level::Warn => level_style
                        .set_color(env_logger::fmt::Color::Yellow)
                        .set_bold(true),
                    Level::Info => level_style
                        .set_color(env_logger::fmt::Color::Green)
                        .set_bold(true),
                    Level::Debug => level_style.set_color(env_logger::fmt::Color::Cyan),
                    Level::Trace => level_style.set_color(env_logger::fmt::Color::Magenta),
                };

                let mut target_style = buf.style();
                target_style
                    .set_color(env_logger::fmt::Color::White)
                    .set_bold(true);

                writeln!(
                    buf,
                    "[{}.{} {:<5} {:<20}] {}",
                    time_style.value(time),
                    millis,
                    level_style.value(record.level()),
                    target_style.value(record.target()),
                    record.args()
                )
            })
            .filter_level(LevelFilter::Trace)
            .init();

        info!("Logger initialized successfully!");

        urls::get_market_browser_url(123);

        self.is_initialized = true;
    }
}
