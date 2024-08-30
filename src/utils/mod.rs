pub mod date_utils;

/// 对 Rust 自带的 LOGGER 接口的简易实现
pub mod simple_log {
    use colored::Colorize;
    use log::{Level, Metadata, Record};
    use log::{LevelFilter, SetLoggerError};

    static LOGGER: SimpleLogger = SimpleLogger;

    pub struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            return metadata.level() <= Level::Info && metadata.target().starts_with("log");
        }
        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let args = record.args();
                match record.level() {
                    Level::Error => {
                        eprintln!("{}{} {}", "error".red().bold(), ":".bold(), args);
                    }
                    Level::Warn => {
                        println!("{}{} {}", "warning".yellow().bold(), ":".bold(), args);
                    }
                    Level::Info => {
                        println!("{}{} {}", "info".green().bold(), ":".bold(), args);
                    }
                    _ => {}
                }
            }
        }
        fn flush(&self) {}
    }

    /// 初始化日志系统
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
    }
}
