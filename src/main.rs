use clap::{Arg, Command};

use commands::generate::generate;
use commands::get::get;
use commands::set::set;

fn main() {
    let command = Command::new("netplancli")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("get")
                .about("Get all the things \\o/")
                .arg(Arg::new("root dir").long("root-dir").default_value("/"))
                .arg(Arg::new("key").default_value("all")),
        )
        .subcommand(
            Command::new("set")
                .about("Set all the things \\o/")
                .arg(Arg::new("root-dir").long("root-dir").default_value("/"))
                .arg(
                    Arg::new("origin-hint")
                        .long("origin-hint")
                        .default_value("70-netplan-set"),
                )
                .arg(Arg::new("key_value").required(true)),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate all the things \\o/")
                .arg(Arg::new("root dir").long("root-dir").default_value("/"))
                .arg(Arg::new("mapping").long("mapping")),
        );

    match command.get_matches().subcommand() {
        Some(("get", args)) => {
            let key = args.get_one::<String>("key").unwrap();
            let root_dir = args.get_one::<String>("root dir").unwrap();
            get(key, root_dir);
        }
        Some(("set", args)) => {
            let key = args.get_one::<String>("key_value").unwrap();
            let root_dir = args.get_one::<String>("root-dir");
            let origin_hint = args.get_one::<String>("origin-hint");
            set(key, origin_hint, root_dir);
        }
        Some(("generate", args)) => {
            let root_dir = args.get_one::<String>("root dir").unwrap();
            let mapping = args.get_one::<String>("mapping");
            generate(root_dir, mapping);
        }

        _ => {}
    }
}
