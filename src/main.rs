extern crate clap;
use clap::{App, Arg, SubCommand};

use std::fs::{File, OpenOptions};

extern crate serde;
extern crate serde_json;

mod config;
mod commands;

#[macro_use]
extern crate serde_derive;

fn main() {
    let matches = App::new("v2ray manager cli")
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
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("controls log")
                .version("1.0")
                .author("coderfox <i@xfox.me>")
                .args(&[
                    Arg::with_name("key")
                        .short("k")
                        .long("key")
                        .help("canbe `access`, `error` or `loglevel`")
                        .takes_value(true)
                        .required(true),
                    Arg::with_name("set")
                        .short("s")
                        .long("set")
                        .help("set a specified value of config")
                        .takes_value(true),
                    Arg::with_name("get")
                        .short("g")
                        .long("get")
                        .help("get a specified value of config"),
                ]),
        )
        .get_matches();
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
