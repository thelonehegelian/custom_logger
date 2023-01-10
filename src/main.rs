use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}{}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

fn main() {
    log::set_logger(&ConsoleLogger).unwrap();
    log::set_max_level(LevelFilter::Info);

    log::info!(": RoboHumans");
    log::warn!(": The Humans are dead");
    log::error!(": We poisoned their asses");
}
