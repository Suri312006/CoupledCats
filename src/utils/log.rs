use std::time::SystemTime;

pub fn setup() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
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
