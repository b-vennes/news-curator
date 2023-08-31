mod config;
mod program;
mod result_ops;
mod site;
mod state;
mod types;

use std::path::Path;
use tera::Tera;

use types::config::Config;
use types::program::Program;

fn main() -> () {
    let feed = Path::new("./news_config.yml");

    let config = Config::read_yaml(feed).unwrap();

    let tera = Tera::new("templates/**/*").unwrap();
    let client = reqwest::blocking::Client::new();

    let program = program::ProgramSync { tera, client };

    let state = program.get_state(config).unwrap();
    let site = program.make_site(state).unwrap();
    program.write_site(site).unwrap();
}
