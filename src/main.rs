use std::error::Error;
use clap::{App, load_yaml};
use log::{error, info};
use xgate_tool::{
    logger_init,
    features::{
        info::show_info,
        dump::dump_graphics,
    },
    resource::graphic::{
        GraphicInfoResource, GraphicResource, PaletteResource
    }
};

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
        _ => {}
    };

    Ok(())
}

fn run(app: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    info!("Start loading resources");
    let mut resources = (
        GraphicInfoResource::load(app.value_of("GraphicInfo").unwrap())?,
        GraphicResource::load(app.value_of("Graphic").unwrap())?,
        PaletteResource::load(app.value_of("Palette"))?,
    );
    info!("Resources loaded");

    match app.subcommand() {
        ("info", Some(sub_args)) => {
            info!("Parsing informations of <GraphicInfo.bin> and <Graphic.bin>");
            show_info(sub_args, &mut resources)?;
        },
        ("dump", Some(sub_args)) => {
            info!("Dumping the graphic");
            dump_graphics(sub_args, &mut resources)?;
        },
        _ => {}
    }

    Ok(())
}
