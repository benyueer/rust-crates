use clap::{Parser, Subcommand};
use std::path::PathBuf;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    }
}

pub fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value of config_path: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is on"),
        2 => println!("Debug mode is on on"),
        _ => println!("Don't be crazy"),
    }

    match cli.command {
        Some(Commands::Test { list }) => {
            if list {
                println!("Printing testing listing ...")
            } else {
                println!("no printing testing listing ...")
            }
        },
        None => {}
    }
}