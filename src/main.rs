use clap::{Arg, Command};

use commands::get::get;

fn main() {
    let command = Command::new("netplancli")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("get")
                .about("Get all the things \\o/")
                .arg(Arg::new("root dir").long("root-dir").default_value("/"))
                .arg(Arg::new("key").default_value("all")),
        );

    match command.get_matches().subcommand() {
        Some(("get", args)) => {
            let key = args.get_one::<String>("key").unwrap();
            let root_dir = args.get_one::<String>("root dir").unwrap();
            get(key, root_dir);
        }
        _ => {}
    }
}
