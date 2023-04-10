extern crate clap;

use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
pub fn main() {
    let matches = command!()
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("dose test things")
                .arg(arg!(-l --list "list test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value of config: {}", config_path.display());
    }

    match matches
    .get_one::<u8>("debug")
    .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is on"),
        2 => println!("Debug mode is on on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.get_flag("list") {
            println!("Printing testing lists...")
        } else {
            println!("no printing testing lists...")
        }
    }
}
