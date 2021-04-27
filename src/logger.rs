use colored::*;
use log::{Level, LevelFilter, Log, Metadata, Record};

pub struct Logger {
    default_level: LevelFilter,
    target_levels: Vec<(String, LevelFilter)>,
}

impl Logger {
    pub fn new(default_level: LevelFilter) -> Self {
        let logger = Logger {
            default_level,
            target_levels: vec![],
        };
        return logger;
    }
    pub fn with_target_level(mut self, target: &str, level: LevelFilter) -> Logger {
        self.target_levels.push((target.to_string(), level));
        return self;
    }
    pub fn init(self) {
        log::set_max_level(LevelFilter::Trace);
        log::set_boxed_logger(Box::new(self)).unwrap();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter()
            <= self
                .target_levels
                .iter()
                /* At this point the Vec is already sorted so that we can simply take
                 * the first match
                 */
                .find(|(name, _level)| metadata.target().starts_with(name))
                .map(|(_name, level)| level)
                .unwrap_or(&self.default_level)
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{}", record.args());
            println!(
                "{}",
                match record.level() {
                    Level::Trace => message.bright_black(),
                    Level::Debug => message.dimmed(),
                    Level::Info => message.normal(),
                    Level::Warn => message.yellow(),
                    Level::Error => message.red(),
                },
            );
        }
    }
    fn flush(&self) {
        todo!()
    }
}
