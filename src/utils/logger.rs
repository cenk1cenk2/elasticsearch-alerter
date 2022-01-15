use anyhow::{Context, Result};
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::LevelFilter;
use std::{io, str::FromStr};

pub fn setup_logger(level: &str) -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Cyan)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y%m%d"),
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(
            LevelFilter::from_str(level)
                .with_context(|| format!("{} is not a valid log level", level))?,
        )
        .chain(io::stdout())
        .apply()?;

    Ok(())
}
