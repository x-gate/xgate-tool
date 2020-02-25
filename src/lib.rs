use fern::colors::{Color, ColoredLevelConfig};

pub mod data_structure;
pub mod resource;

pub fn logger_init(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::default().info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                color.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}