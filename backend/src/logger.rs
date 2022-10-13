use log::{Level, Metadata, Record};
struct CommandCenterLogger {}

impl log::Log for CommandCenterLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {}
    fn flush(&self) {}
    fn log(&self, record: &Record) {}
}
