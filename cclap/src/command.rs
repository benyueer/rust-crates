use clap::{arg, Command};

pub fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("yueer")
        .about("command")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!("two: {:?}", matches.get_one::<String>("two").expect("required"));
    println!("one: {:?}", matches.get_one::<String>("one").expect("required"));
}