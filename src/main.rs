extern crate clap;
use clap::{App, Arg};

use std::fs::{File, OpenOptions};

extern crate serde;
extern crate serde_json;

mod config;
mod commands;

#[macro_use]
extern crate serde_derive;

fn main() {
    let app = App::new("v2ray manager cli")
        .version("1.0")
        .author("coderfox <i@xfox.me>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("file")
                .help("the config file of v2ray to be modified")
                .default_value("v2ray.json")
                .takes_value(true)
                .short("f")
                .long("file"),
        );
    let app = commands::log::apply_subcommand(app);
    let app = commands::dns::apply_subcommand(app);
    let matches = app.get_matches();
    let mut config: config::Config = match File::open(matches.value_of("file").unwrap()) {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(value) => value,
            Err(_) => config::Config {
                ..Default::default()
            },
        },
        Err(_) => config::Config {
            ..Default::default()
        },
    };
    println!("Config file readed: {:?}", &config);

    if let Some(ref matches) = matches.subcommand_matches("log") {
        commands::log::log(&mut config, matches);
    };
    if let Some(ref matches) = matches.subcommand_matches("dns") {
        commands::dns::dns(&mut config, matches);
    }
    println!("config modified: {:?}", &config);

    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(matches.value_of("file").unwrap())
    {
        Ok(file) => match serde_json::to_writer(file, &config) {
            Ok(_) => println!("config modefied"),
            Err(err) => panic!("serialize failed {:?}", err),
        },
        Err(_) => panic!("write failed"),
    };
}
