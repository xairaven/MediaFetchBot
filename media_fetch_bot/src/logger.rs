use chrono::Local;
use log::{LevelFilter, SetLoggerError};

pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        // Issue #1, https://github.com/xairaven/MediaFetchBot/issues/2
        .level_for("teloxide::update_listeners::polling", LevelFilter::Debug)
        // Issue #2, https://github.com/xairaven/MediaFetchBot/issues/1
        .level_for("teloxide::error_handlers", LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
}
