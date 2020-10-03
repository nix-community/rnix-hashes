extern crate base16;
extern crate base64;

mod encoding;
mod types;

use crate::{
    base32,
    hash::{
        encoding::{
            HashEncoding,
        },
        types::HashType,
    }
};
use std::result::Result;
use core::fmt;
use unindent::unindent;

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

        let hash_encoding = match HashEncoding::get_encoding(htype, hdata, hash_is_sri) {
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

    // pub(crate) fn reformat_hash(enc: Option<&str>, hash: Hash) -> {

    // }

    // By default, print_hash will show all possible encoding for the hash
    // It can be changed by providing argument --encoding into the program
    pub(crate) fn print_hash(enc: Option<&'a str>, hash: &'a str) -> String {
        let parsed_hash = Hash::parse_hash(hash).unwrap();
        let hash_type = parsed_hash.hash_type.from_type();
        let hash_data = match parsed_hash.data {
            Some(a) => a,
            None => Vec::new()
        };
        let htype: String = hash_type.into();
        let enc16: String = base16::encode_lower(&hash_data);
        let enc32: String = base32::encode(&hash_data);
        let enc64: String = base64::encode(&hash_data);

        if let Some(encoding) = enc {
            match HashEncoding::into_encoding(encoding){
                Some(HashEncoding::BASE16) => {
                    return format!("base16\t{}", enc16)
                },
                Some(HashEncoding::BASE32) => {
                    return format!("base32\t{}", enc32)
                },
                Some(HashEncoding::BASE64) => {
                    return format!("base64\t{}", enc64)
                },
                Some(HashEncoding::PBASE16) => {
                    return format!("base16\t{}", enc16)
                },
                Some(HashEncoding::PBASE32) => {
                    return format!("base32\t{}", enc32)
                },
                Some(HashEncoding::PBASE64) => {
                    return format!("base64\t{}", enc64)
                },
                Some(HashEncoding::SRI) => {
                    return format!("SRI\t{}-{}", htype, enc64)
                },
                _ => return "".into()
            }
        }
        // If encoding argument is empty, then the default is printing all encoding
        return unindent(&format!("
        SRI\t{}-{}
        base16\t{}
        base32\t{}
        base64\t{}
        ",
            htype, enc64,
            enc16,
            enc32,
            enc64,
        ))
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


// TODO:
// - decoding test
// - encoding test

#[cfg(test)]
mod tests {
    use crate::Hash;
    use unindent::unindent;

    const INPUT: &str = "sha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=";

    #[test]
    fn test_opt_base16() {
        assert_eq!(Hash::print_hash(Some("BASE16"),INPUT), "base16\t637f4e56db1c221e95487e1607008333f7863c510ec735ed8271d4ef4f029ea5");
    }

    #[test]
    fn test_opt_base32() {
        assert_eq!(Hash::print_hash(Some("BASE32"),INPUT), "base32\t19cy097yzm3ihbnkbiqfa4y8dxrkhc00f5ky92aiw8hwvdb4wzv3");
    }

    #[test]
    fn test_opt_base64() {
        assert_eq!(Hash::print_hash(Some("BASE64"),INPUT), "base64\tY39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=");
    }

    #[test]
    fn test_opt_pbase16() {
        assert_eq!(Hash::print_hash(Some("PBASE16"),INPUT), "base16\t637f4e56db1c221e95487e1607008333f7863c510ec735ed8271d4ef4f029ea5");
    }

    #[test]
    fn test_opt_pbase32() {
        assert_eq!(Hash::print_hash(Some("PBASE32"),INPUT), "base32\t19cy097yzm3ihbnkbiqfa4y8dxrkhc00f5ky92aiw8hwvdb4wzv3");
    }

    #[test]
    fn test_opt_pbase64() {
        assert_eq!(Hash::print_hash(Some("PBASE64"),INPUT), "base64\tY39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=");
    }

    #[test]
    fn test_opt_sri() {
        assert_eq!(Hash::print_hash(Some("SRI"),INPUT), "SRI\tsha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=");
    }

    #[test]
    fn test_opt_all() {
        assert_eq!(Hash::print_hash(None,INPUT), unindent("
        SRI\tsha256-Y39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=
        base16\t637f4e56db1c221e95487e1607008333f7863c510ec735ed8271d4ef4f029ea5
        base32\t19cy097yzm3ihbnkbiqfa4y8dxrkhc00f5ky92aiw8hwvdb4wzv3
        base64\tY39OVtscIh6VSH4WBwCDM/eGPFEOxzXtgnHU708CnqU=
        "));
    }
}
