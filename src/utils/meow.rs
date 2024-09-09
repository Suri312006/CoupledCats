use std::time::SystemTime;

use color_eyre::owo_colors::OwoColorize;
use fern::colors::{Color, ColoredLevelConfig};

pub fn setup() -> Result<(), fern::InitError> {
    let mut colors = ColoredLevelConfig::new();

    colors.error = Color::Red;
    colors.warn = Color::Yellow;
    colors.info = Color::Green;
    colors.debug = Color::Cyan;
    colors.trace = Color::Cyan;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .level_for("wgpu_core", log::LevelFilter::Off)
        .level_for("calloop", log::LevelFilter::Off)
        .level_for("naga", log::LevelFilter::Off)
        .level_for("wgpu_hal", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}
