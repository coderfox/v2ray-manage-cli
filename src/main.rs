extern crate clap;
use clap::{App, Arg, SubCommand};

use std::fs::File;

extern crate serde;
extern crate serde_json;

mod config;

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
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => config::Config {
            ..Default::default()
        },
    };
    println!("Config file readed: {:?}", &config);

    if let Some(ref matches) = matches.subcommand_matches("log") {
        println!("Key: {}", matches.value_of("key").unwrap());
        match matches.value_of("set") {
            Some(value) => match matches.value_of("key").unwrap() {
                "access" => {
                    config.log = Some(config::Log {
                        access: Some(String::from(value)),
                        ..config.log.unwrap_or_default()
                    })
                }
                "error" => {
                    config.log = Some(config::Log {
                        error: Some(String::from(value)),
                        ..config.log.unwrap_or_default()
                    })
                }
                "loglevel" => {
                    config.log = Some(config::Log {
                        error: Some(String::from(value)),
                        ..config.log.unwrap_or_default()
                    })
                }
                value => panic!("invalid key {}", value),
            },
            None => if matches.is_present("get") {
                println!(
                    "{} = {}",
                    matches.value_of("key").unwrap(),
                    match (&config).log {
                        Some(log) => match matches.value_of("key").unwrap() {
                            "access" => log.access.unwrap_or(String::from("null")),
                            "error" => log.error.unwrap_or(String::from("null")),
                            "loglevel" => log.loglevel.unwrap_or(String::from("null")),
                            value => panic!("invalid key {}", value),
                        },
                        None => String::from("null"),
                    }
                )
            } else {
                panic!("no operation provided")
            },
        }
    };
    println!("config modified: {:?}", &config);
}
