use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;

use crate::hash::{
    MD5SIZE,
    SHA1SIZE,
    SHA256SIZE,
    SHA512SIZE,
    base16_len,
    base32_len,
    base64_len,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum HashType {
    MD5,    // 16 bytes
    Sha1,   // 20 bytes
    Sha256, // 32 bytes
    Sha512, // 64 bytes
}

#[derive(Debug, Clone, Copy)]
pub enum HashError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    InvalidType,
}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HashError::InvalidType => {
                write!(f, "Hash has invalid type")
            }
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for HashError {
    fn description(&self) -> &str {
        match *self {
            HashError::InvalidType => "Invalid type"

        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}


impl fmt::Display for HashType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HashType::MD5 => {
                write!(f, "{}", self.from_type())
            },
            HashType::Sha1 => {
                write!(f, "{}", self.from_type())
            },
            HashType::Sha256 => {
                write!(f, "{}", self.from_type())
            },
            HashType::Sha512 => {
                write!(f, "{}", self.from_type())
            }
        }
    }
}

impl HashType {
    // TODO: Write test
    pub(crate) fn from_type<'a>(self) -> &'a str {
        match self {
            HashType::MD5 => "md5",
            HashType::Sha1 => "sha1",
            HashType::Sha256 => "sha256",
            HashType::Sha512 => "sha512",
        }
    }

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
    pub(super) fn find_hashing(hdata: &str) -> Result<Self, HashError> {
        match hdata.len() {
            x if x == base16_len(MD5SIZE) | base32_len(MD5SIZE) | base64_len(MD5SIZE) => {
                return Ok(HashType::MD5)
            }
            x if x == base16_len(SHA1SIZE) | base32_len(SHA1SIZE) | base64_len(SHA1SIZE) => {
                return Ok(HashType::MD5)
            }
            x if x == base16_len(SHA256SIZE) | base32_len(SHA256SIZE) | base64_len(SHA256SIZE) => {
                return Ok(HashType::MD5)
            }
            x if x == base16_len(SHA512SIZE) | base32_len(SHA512SIZE) | base64_len(SHA512SIZE) => {
                return Ok(HashType::MD5)
            }
            _ => return Err(HashError::InvalidType),
        }
    }
}
