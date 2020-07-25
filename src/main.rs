mod base32;
mod hash;

use clap::{
    Arg,
    app_from_crate,
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
};
use hash::Hash;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = app_from_crate!()
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
