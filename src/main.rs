extern crate clap;

mod base32;
mod hash;

use clap::{App, Arg, SubCommand};
use hash::Hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Nixihash")
        .version("0.1")
        .author("NumTide Engineering")
        .about("Nix Has Converter")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
    // 1. we parse the hash
    // 2. print the hash in multiple encoding
    //
    let hash_arg = matches.value_of("INPUT").unwrap();
    let parsed_hash = Hash::parse_hash(hash_arg)?;
    Hash::print_hash(parsed_hash);
    Ok(())
}
