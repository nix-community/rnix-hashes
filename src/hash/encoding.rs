use crate::{
    base32,
    hash::{
      MD5SIZE,
      SHA1SIZE,
      SHA256SIZE,
      SHA512SIZE,
      base16_len,
      base32_len,
      base64_len,
      types::{HashType, HashError},
    },
};
use core::fmt;

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

impl fmt::Display for HashEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HashEncoding::BASE16 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::BASE32 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::BASE64 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::PBASE16 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::PBASE32 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::PBASE64 => {
                write!(f, "{}", self.from_encoding())
            },
            HashEncoding::SRI => {
                write!(f, "{}", self.from_encoding())
            },
        }
    }
}

impl HashEncoding {
  // TODO: write the implementation and test
  pub(crate) fn from_encoding<'a>(self) -> &'a str {
      match self {
          HashEncoding::BASE16 => "BASE16",
          HashEncoding::BASE32 => "BASE32",
          HashEncoding::BASE64 => "BASE64",
          HashEncoding::PBASE16 => "PBASE16",
          HashEncoding::PBASE32 => "PBASE32",
          HashEncoding::PBASE64 => "PBASE64",
          HashEncoding::SRI => "SRI",
      }
  }
  // TODO: write the implementation and test
  pub(crate) fn into_encoding<'a> (
      htype: Option<&'a str>,
      hdata: &'a str,
      is_sri: bool,
  ) -> Result<HashEncoding, HashError> {
      let is_prefix = htype.is_some();
      let hash_type = match htype.and_then(|x| HashType::into_type(x)) {
          Some(ht) => Ok(ht),
          None => HashType::find_hashing(hdata),
      };
      if let Ok(ht) = hash_type {
          match ht {
              HashType::MD5 => {
                  return HashEncoding::find_encoding(hdata, MD5SIZE, is_sri, is_prefix)
              }
              HashType::Sha1 => {
                  return HashEncoding::find_encoding(hdata, SHA1SIZE, is_sri, is_prefix)
              },
              HashType::Sha256 => {
                  return HashEncoding::find_encoding(hdata, SHA256SIZE, is_sri, is_prefix)
              },
              HashType::Sha512 => {
                  return HashEncoding::find_encoding(hdata, SHA512SIZE, is_sri, is_prefix)
              }
          }
      }
  return Err(HashError::InvalidType)
  }

  pub(super) fn find_encoding(
      hdata: &str,
      hash_size: usize,
      is_sri: bool,
      is_prefix: bool,
  ) -> Result<HashEncoding, HashError> {
      if is_sri {
          return Ok(HashEncoding::SRI);
      }
      if !is_sri && is_prefix {
          if hdata.len() == base16_len(hash_size) {
              return Ok(HashEncoding::PBASE16);
          }
          if hdata.len() == base32_len(hash_size) {
              return Ok(HashEncoding::PBASE32);
          }
          if hdata.len() == base64_len(hash_size) {
              return Ok(HashEncoding::PBASE64);
          }
      }
      if !is_sri && !is_prefix {
          if hdata.len() == base16_len(hash_size) {
              return Ok(HashEncoding::BASE16);
          }
          if hdata.len() == base32_len(hash_size) {
              return Ok(HashEncoding::BASE32);
          }
          if hdata.len() == base64_len(hash_size) {
              return Ok(HashEncoding::BASE64);
          }
      }
      return Err(HashError::InvalidType)
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
