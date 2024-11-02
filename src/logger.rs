use chrono::{Datelike, Local, Timelike};
use log::{LevelFilter, Record, SetLoggerError};
use std::fmt::Arguments;
use teloxide::types::Message;

pub fn init(
    log_level: &LevelFilter, format: String,
) -> Result<(), SetLoggerError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            let formatted = parse_log_format(format.clone(), message, record);
            out.finish(format_args!(
                "{}",
                formatted
            ))
        })
        .level(*log_level)
        // Issue #1, https://github.com/xairaven/MediaFetchBot/issues/1
        .level_for("teloxide::update_listeners::polling", LevelFilter::Debug)
        // Issue #2, https://github.com/xairaven/MediaFetchBot/issues/2
        .level_for("teloxide::error_handlers", LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
}

pub fn parse_log_format(
    format: String, message: &Arguments, record: &Record,
) -> String {
    let mut log = format;

    // Message
    log = log.replace("%MESSAGE", &message.to_string());

    // Level
    log = log.replace("%LEVEL", record.level().as_str());

    // Target
    log = log.replace("%TARGET", record.target());

    // Time
    let time = Local::now();
    log = log.replace("%Y", &format!("{:0>2}", time.year()));
    log = log.replace("%m", &format!("{:0>2}", time.month()));
    log = log.replace("%D", &format!("{:0>2}", time.day()));
    log = log.replace("%H", &format!("{:0>2}", time.hour()));
    log = log.replace("%M", &format!("{:0>2}", time.minute()));
    log = log.replace("%S", &format!("{:0>2}", time.second()));

    log
}

pub fn get_sender_identifier(msg: &Message) -> String {
    let identifier: String;

    if let Some(user) = &msg.from {
        if let Some(username) = &user.username {
            identifier = format!("@{}", username);
        } else {
            identifier = format!("{} ({})", user.full_name(), msg.chat.id);
        }
    } else {
        identifier = format!("{}", msg.chat.id);
    }

    identifier
}
