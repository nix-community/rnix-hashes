extern crate base16;
extern crate base64;

mod encoding;
mod types;

use crate::hash::{
    encoding::{HashEncoding},
    types::HashType,
};
use std::result::Result;
use core::fmt;

// Hash size
const MD5SIZE: usize = 16;
const SHA1SIZE: usize = 20;
const SHA256SIZE: usize = 32;
const SHA512SIZE: usize = 64;

// Base encoding characters
// static B64_CHARS: &'static [u8; 64] =
//     &b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
// static B16_CHARS: &'static [u8; 16] = &b"0123456789abcdef";
// static B32_CHARS: &'static [u8; 32] = &b"0123456789abcdfghijklmnpqrsvwxyz";

#[derive(Clone, Debug)]
pub(crate) struct Hash {
    hash_type: HashType,
    hash_encoding: HashEncoding,
    data: Option<std::vec::Vec<u8>>,
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
        "
          type: {:?}\n
          encoding: {:?}\n
          data: {:?}
        ",
        self.hash_type, self.hash_encoding, self.data
        )

    }
}

impl<'a> Hash {
    pub(super) fn split_hash(hash: &str, hash_is_sri: bool) -> (Option<&str>, &str) {
        if hash_is_sri {
            let sp = hash.split('-').into_iter();
            let first = sp.clone().min_by(|x, y| x.len().cmp(&y.len()));
            let second = sp.last().unwrap_or("");
            println!(
                "SRI:\n hash_type = {:?},\n is_sri = {:?},\n hash_data = {:?}",
                first, hash_is_sri, second
            );
            (first, second)
        } else {
            if hash.contains(':') {
                let sp = hash.split(':');
                let first = sp.clone().min_by(|x, y| x.len().cmp(&y.len()));
                let second = sp.last().unwrap_or("");
                (first, second)
            } else {
                return (None, hash);
            }
        }
    }
    // The input will be <hash_type>-<hash_data> or <hash_type>:<hash_data>
    pub(crate) fn parse_hash(hash: &str) -> Result<Hash, Box<dyn std::error::Error>> {
        let hash_is_sri = hash.contains('-');
        let (htype, hdata) = Hash::split_hash(hash, hash_is_sri);
        let hash_type = {
            match htype.and_then(|x| HashType::into_type(x)) {
                Some(ht) => ht,
                None => HashType::Sha256,
            }
        };

        let hash_encoding = match HashEncoding::into_encoding(htype, hdata, hash_is_sri) {
            Ok(a) => a,
            Err(_) => return Err(From::from("Invalid hash type")), // TODO: Fine the best way to implement error from HashError
        };

        let decoded_data = HashEncoding::decode_data(hash_encoding, hdata);

        return Ok(Hash {
            hash_type: hash_type,
            hash_encoding: hash_encoding,
            data: decoded_data,
        });
    }

    pub(crate) fn print_hash(hash: Hash) {
        // TODO
        //
        println!(
            "
          type: {:?}\n
          encoding: {:?}\n
          data: {:?}
        ",
            hash.hash_type, hash.hash_encoding, hash.data
        )
    }
}

// Basic length representation of Base16
pub(crate) fn base16_len(hash_size: usize) -> usize {
    hash_size * 2
}

// Basic length representation of Base32
pub(crate) fn base32_len(hash_size: usize) -> usize {
    (hash_size * 8 - 1) / 5 + 1
}

// Basic length representation of Base64
pub(crate) fn base64_len(hash_size: usize) -> usize {
    ((4 * hash_size / 3) + 3) & !3
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct TestCase {
        name: Option<String>,
        before: String,
        after: String,
    }

    // impl TestCase {
    //     fn run(&self) -> Result<(), String> {
    //         Ok(())
    //     }
    // }

    // fn run(tests: &[TestCase]) {
    //     let mut n_failed = 0;
    //     for test in tests {
    //         if let Err(msg) = test.run() {
    //             n_failed += 1;
    //             eprintln!("{}", msg)
    //         }
    //     }
    //     if n_failed > 0 {
    //         panic!(
    //             "{} failed test cases out of {} total",
    //             n_failed,
    //             tests.len()
    //         )
    //     }
    // }
}
