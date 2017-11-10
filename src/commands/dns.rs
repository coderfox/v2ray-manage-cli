use clap::{App, Arg, ArgMatches, SubCommand};
use config;

pub fn apply_subcommand<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("dns")
            .about("controls dns")
            .version("1.0")
            .author("coderfox <i@xfox.me>"),
    )
}
pub fn dns(config: &mut config::Config, matches: &ArgMatches) {}
