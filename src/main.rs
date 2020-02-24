use std::error::Error;
use clap::{App, load_yaml};
use log::{error};

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_yaml!("../config/conf.yaml");
    let app = App::from_yaml(config).get_matches();

    match app.occurrences_of("verbose") {
        0 => logger_init(log::LevelFilter::Warn)?,
        1 => logger_init(log::LevelFilter::Info)?,
        2 => logger_init(log::LevelFilter::Debug)?,
        3|_ => logger_init(log::LevelFilter::Trace)?,
    }

    match run(app) {
        Err(e) => {
            error!("{:?}", e);
            return Err(e);
        },
        _ => ()
    };

    Ok(())
}

fn run(app: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn logger_init(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })
    .level(level)
    .chain(std::io::stdout())
    .apply()?;

    Ok(())
}
