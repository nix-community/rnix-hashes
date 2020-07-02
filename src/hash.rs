extern crate base16;
extern crate base64;

use crate::base32;
use std::result::Result;

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

#[derive(Debug, Clone, Copy)]
pub(crate) enum HashType {
    MD5,    // 16 bytes
    Sha1,   // 20 bytes
    Sha256, // 32 bytes
    Sha512, // 64 bytes
}

impl HashType {
    // TODO: Write test
    // pub(crate) fn from_type<'a>(self) -> &'a str {
    //     match self {
    //         HashType::MD5 => "md5",
    //         HashType::Sha1 => "sha1",
    //         HashType::Sha256 => "sha256",
    //         HashType::Sha512 => "sha512",
    //     }
    // }

    // TODO: Write test
    pub(crate) fn into_type(text: &str) -> Option<HashType> {
        match text {
            "md5" => Some(HashType::MD5),
            "sha1" => Some(HashType::Sha1),
            "sha256" => Some(HashType::Sha256),
            "Sha512" => Some(HashType::Sha512),
            _ => None,
        }
    }

    // FIXME: Brute force is bad!
    pub(super) fn find_hashing(hdata: &str) -> Self {
        match hdata.len() {
            x if x == base16_len(MD5SIZE) | base32_len(MD5SIZE) | base64_len(MD5SIZE) => {
                return HashType::MD5
            }
            x if x == base16_len(SHA1SIZE) | base32_len(SHA1SIZE) | base64_len(SHA1SIZE) => {
                return HashType::MD5
            }
            x if x == base16_len(SHA256SIZE) | base32_len(SHA256SIZE) | base64_len(SHA256SIZE) => {
                return HashType::MD5
            }
            x if x == base16_len(SHA512SIZE) | base32_len(SHA512SIZE) | base64_len(SHA512SIZE) => {
                return HashType::MD5
            }
            _ => HashType::MD5,
        }
    }
}

// function of encoder and decoder of BASE16, BASE32, and BASE64
#[derive(Debug, Clone, Copy)]
pub(crate) enum HashEncoding {
    BASE16,
    BASE32,
    BASE64,

    // Prefix encoding
    // format <type>:base<n>
    PBASE16,
    PBASE32,
    PBASE64,

    // format <type> - base64
    SRI,
}

impl HashEncoding {
    // TODO: write the implementation and test
    pub(crate) fn into_encoding(
        htype: Option<&str>,
        hdata: &str,
        is_sri: bool,
    ) -> Option<HashEncoding> {
        let is_prefix = htype.is_some();
        let hash_type = match htype.and_then(|x| HashType::into_type(x)) {
            Some(ht) => ht,
            None => HashType::find_hashing(hdata),
        };
        match hash_type {
            HashType::MD5 => HashEncoding::find_encoding(hdata, MD5SIZE, is_sri, is_prefix),
            HashType::Sha1 => HashEncoding::find_encoding(hdata, SHA1SIZE, is_sri, is_prefix),
            HashType::Sha256 => HashEncoding::find_encoding(hdata, SHA256SIZE, is_sri, is_prefix),
            HashType::Sha512 => HashEncoding::find_encoding(hdata, SHA512SIZE, is_sri, is_prefix),
        }
    }

    pub(super) fn find_encoding(
        hdata: &str,
        hash_size: usize,
        is_sri: bool,
        is_prefix: bool,
    ) -> Option<HashEncoding> {
        if is_sri {
            return Some(HashEncoding::SRI);
        }
        if !is_sri && is_prefix {
            if hdata.len() == base16_len(hash_size) {
                return Some(HashEncoding::PBASE16);
            }
            if hdata.len() == base32_len(hash_size) {
                return Some(HashEncoding::PBASE32);
            }
            if hdata.len() == base64_len(hash_size) {
                return Some(HashEncoding::PBASE32);
            }
        }
        if !is_sri && !is_prefix {
            if hdata.len() == base16_len(hash_size) {
                return Some(HashEncoding::BASE16);
            }
            if hdata.len() == base32_len(hash_size) {
                return Some(HashEncoding::BASE32);
            }
            if hdata.len() == base64_len(hash_size) {
                return Some(HashEncoding::BASE32);
            }
        }
        return None;
    }

    pub(crate) fn decode_data<'a>(
        hash_encoding: HashEncoding,
        hdata: &str,
    ) -> Option<std::vec::Vec<u8>> {
        match hash_encoding {
            HashEncoding::BASE16 | HashEncoding::PBASE16 => base16::decode(hdata).ok(),
            HashEncoding::BASE32 | HashEncoding::PBASE32 => base32::decode(hdata).ok(),
            HashEncoding::BASE64 | HashEncoding::PBASE64 => base64::decode(hdata).ok(),
            HashEncoding::SRI => base64::decode(hdata).ok(),
        }
    }

    // pub(crate) fn from_encoding<'a>(text: HashEncoding) -> &'a str {
    //     match text {
    //         BASE16 =>"",
    //         BASE32 =>"",
    //         BASE64 =>"",

    //         // Prefix encoding
    //         // format <type>:base<n>
    //         PBASE16 =>,
    //         PBASE32 => ,
    //         PBASE64 => ,

    //         // format <type> - base64
    //         SRI =>,
    //     }
    // }
}

pub(crate) fn base16_len(hash_size: usize) -> usize {
    hash_size * 2
}

pub(crate) fn base32_len(hash_size: usize) -> usize {
    (hash_size * 8 - 1) / 5 + 1
}

pub(crate) fn base64_len(hash_size: usize) -> usize {
    ((4 * hash_size / 3) + 3) & !3
}

#[derive(Clone, Debug)]
pub(crate) struct Hash {
    hash_type: HashType,
    hash_encoding: HashEncoding,
    data: Option<std::vec::Vec<u8>>,
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
                println!(
                    "Prefx:\n hash_type = {:?},\n is_sri = {:?},\n hash_data = {:?}",
                    first, hash_is_sri, second
                );
                (first, second)
            } else {
                return (None, hash);
            }
        }
    }
    // The input will be <hash_type>-<hash_data> or <hash_type>:<hash_data>
    pub(crate) fn parse_hash(hash: &str) -> Result<Hash, Box<dyn std::error::Error>> {
        // let err_par: Box<dyn std::error::Error> = From::from("parsing hash failed");
        let err_enc: Box<dyn std::error::Error> =
            From::from("decoding failed: unable to find the hash type");
        let hash_is_sri = hash.contains('-');
        let (htype, hdata) = Hash::split_hash(hash, hash_is_sri);
        let hash_type = {
            match htype.and_then(|x| HashType::into_type(x)) {
                Some(ht) => ht,
                None => HashType::Sha256,
            }
        };

        println!("hash type = {:?}", hash_type);
        let hash_encoding = match HashEncoding::into_encoding(htype, hdata, hash_is_sri) {
            Some(a) => a,
            None => return Err(err_enc),
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

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct TestCase {
        name: Option<String>,
        before: String,
        after: String,
    }

    impl TestCase {
        fn run(&self) -> Result<(), String> {
            Ok(())
        }
    }

    fn run(tests: &[TestCase]) {
        let mut n_failed = 0;
        for test in tests {
            if let Err(msg) = test.run() {
                n_failed += 1;
                eprintln!("{}", msg)
            }
        }
        if n_failed > 0 {
            panic!(
                "{} failed test cases out of {} total",
                n_failed,
                tests.len()
            )
        }
    }
}
