use clap::{App, load_yaml};

fn main() {
    let _app = App::from_yaml(load_yaml!("../config/conf.yaml")).get_matches();
}
