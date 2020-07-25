extern crate clap;

mod base32;
mod hash;

use clap::{App, Arg, crate_version, crate_description, crate_name, crate_authors};
use hash::Hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(Arg::with_name("encoding")
            .long("encoding")
            .value_name("ENCODING")
            .help("Sets specific encoding: BASE16|BASE32|BASE64|PBASE16|PBASE32|PBASE64|SRI")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let encoding = matches.value_of("encoding");
    let hash_arg = matches.value_of("INPUT").unwrap();
    println!("{}", Hash::print_hash(encoding, hash_arg));
    Ok(())
}
