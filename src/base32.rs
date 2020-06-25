use lazy_static::lazy_static;

pub fn encoded_len(input_len: usize) -> usize {
    if input_len == 0 {
        0
    } else {
        (input_len * 8 - 1) / 5 + 1
    }
}

pub fn decoded_len(input_len: usize) -> usize {
    input_len * 5 / 8
}

static BASE32_CHARS: &'static [u8; 32] = &b"0123456789abcdfghijklmnpqrsvwxyz";

lazy_static! {
    static ref BASE32_CHARS_REVERSE: Box<[u8; 256]> = {
        let mut xs = [0xffu8; 256];
        for (n, c) in BASE32_CHARS.iter().enumerate() {
            xs[*c as usize] = n as u8;
        }
        Box::new(xs)
    };
}

pub(crate) fn encode(input: &[u8]) -> String {
    let mut buf = vec![0; encoded_len(input.len())];
    encode_into(input, &mut buf);
    std::str::from_utf8(&buf).unwrap().to_string()
}

pub fn encode_into(input: &[u8], output: &mut [u8]) {
    let len = encoded_len(input.len());
    assert_eq!(len, output.len());

    let mut nr_bits_left: usize = 0;
    let mut bits_left: u16 = 0;
    let mut pos = len;

    for b in input {
        bits_left |= (*b as u16) << nr_bits_left;
        nr_bits_left += 8;
        while nr_bits_left > 5 {
            output[pos - 1] = BASE32_CHARS[(bits_left & 0x1f) as usize];
            pos -= 1;
            bits_left >>= 5;
            nr_bits_left -= 5;
        }
    }

    if nr_bits_left > 0 {
        output[pos - 1] = BASE32_CHARS[(bits_left & 0x1f) as usize];
        pos -= 1;
    }

    assert_eq!(pos, 0);
}

pub(crate) fn decode(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut res = Vec::with_capacity(decoded_len(input.len()));
    let err: Box<dyn std::error::Error> = From::from("invalid base32 string");
    let mut nr_bits_left: usize = 0;
    let mut bits_left: u16 = 0;

    for c in input.chars().rev() {
        let b = BASE32_CHARS_REVERSE[c as usize];
        if b == 0xff {
            return Err(err);
        }
        bits_left |= (b as u16) << nr_bits_left;
        nr_bits_left += 5;
        if nr_bits_left >= 8 {
            res.push((bits_left & 0xff) as u8);
            bits_left >>= 8;
            nr_bits_left -= 8;
        }
    }

    if nr_bits_left > 0 && bits_left != 0 {
        return Err(err);
    }

    Ok(res)
}
