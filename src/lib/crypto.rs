// Copyright 2020 Farzad FARID <farzy@farzy.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! String, cryptographic and mathematical functions

use std::{error, fmt};
use std::fmt::Write;
use std::char;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{english, stats};

const BASE64_ALPHABET: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/', '='
];


// Create a custom error and boxing dyn errors

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
struct InvalidHexString;

impl fmt::Display for InvalidHexString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid hexadecimal string")
    }
}

impl error::Error for InvalidHexString {}

/// Add hexadecimal string manipulation to strings.
pub trait HexString {
    fn hex2bytes(&self) -> Result<Vec<u8>>;
    fn hex2string(&self) -> Result<String>;
    fn base64_decode(&self) -> Result<Vec<u8>>;
}

impl HexString for str {
    /// Convert a hex string to an array of bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::HexString;
    ///
    /// assert_eq!(vec![65], "41".hex2bytes().unwrap());
    /// assert_eq!(vec![16, 32, 48], "102030".hex2bytes().unwrap());
    /// assert!("1020ZZ".hex2bytes().is_err());
    /// ```
    fn hex2bytes(&self) -> Result<Vec<u8>> {
        let l = self.len();
        if l == 0 || (l & 0b1) == 1 {
            return Err(Box::new(InvalidHexString));
        }
        (0..l)
            .step_by(2)
            .map(|i|
                u8::from_str_radix(&self[i..i + 2], 16)
                    .map_err(|e| e.into())) // Converts to Box
            .collect()
    }

    /// Convert a hex string to a string
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::HexString;
    ///
    /// assert_eq!("A".to_owned(), "41".hex2string().unwrap());
    /// assert_eq!("the kid don't play", "746865206b696420646f6e277420706c6179".hex2string().unwrap());
    /// assert!("1020ZZ".hex2string().is_err());
    /// ```
    fn hex2string(&self) -> Result<String> {
        let l = self.len();
        if l == 0 || (l & 0b1) == 1 {
            return Err(Box::new(InvalidHexString));
        }
        let mut s = String::with_capacity(l / 2);
        for i in (0..l).step_by(2) {
            let c = u8::from_str_radix(&self[i..i + 2], 16)? as char;
            s.push(c);
        }
        Ok(s)
    }

    /// Decode a Base64 string to a byte array
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::HexString;
    ///
    /// assert_eq!(String::from("QUJD").base64_decode().unwrap(), vec![65, 66, 67]);
    /// assert_eq!("QUJD".base64_decode().unwrap(), &[65, 66, 67]);
    /// assert_eq!("SGVsbG8sIHdvcmxkIQ==".base64_decode().unwrap(), "Hello, world!".as_bytes());
    /// ```
    ///
    /// # References
    ///
    /// This code is inspired by [this article](https://levelup.gitconnected.com/implementing-base64-in-rust-34ef6db1e73a).
    fn base64_decode(&self) -> Result<Vec<u8>> {
        let mut padding_count = 0;
        // We need interior mutability here because we both update and read the byte array's
        // length in the same expression, but not at the same time.
        let b64_length = Rc::new(RefCell::new(self.len()));
        let b64_bytes: Result<Vec<u8>> = self
            .bytes()
            // Remove return chars all the while adjusting array length
            .filter(|&b| {
                if b != '\n' as u8 && b != '\r' as u8 {
                    return true;
                } else {
                    *b64_length.borrow_mut() -= 1;
                    return false;
                }
            })
            .enumerate()
            .map(| (index, byte)| {
                match byte {
                    // A to Z => 0 to 25
                    65..=90 => Ok(byte - 65),
                    // a to z => 26 to 51
                    97..=122 => Ok(byte - 97 + 26),
                    // 0 to 9 => 52 to 61
                    48..=57 => Ok(byte + 4),
                    // + => 62
                    43 => Ok(62),
                    // / => 63
                    47 => Ok(63),
                    // = => 0
                    61 => {
                        // Equal sign only authorized at end of string
                        if index >= *b64_length.borrow() - 2 {
                            padding_count += 1;
                            Ok(0)
                        } else {
                            Err(format!("invalid byte '=' at position {} in Base64 string", index).into())
                        }
                    },
                    _ => Err(format!("invalid byte '{}' (0x{:X}) at position {} in Base64 string", byte as char, byte, index).into())
                }
            })
            .collect();
        if b64_bytes.is_err() {
            return b64_bytes;
        }
        if *b64_length.borrow() % 4 != 0 {
            return Err(format!("invalid Base64 length: {}", *b64_length.borrow()).into());
        }
        let mut bytes = b64_bytes
            .unwrap()
            .chunks(4)
            .map(|quartet| {
                let b1 = quartet[0] << 2                | (quartet[1] & 0b00110000) >> 4;
                let b2 = (quartet[1] & 0b00001111) << 4 | (quartet[2] & 0b00111100) >> 2;
                let b3 = (quartet[2] & 0b00000011) << 6 | quartet[3];
                vec![b1, b2, b3]
            })
            .flatten()
            .collect::<Vec<u8>>();
        // Remove extra bytes created by the padding
        bytes.resize(bytes.len() - padding_count, 0);
        Ok(bytes)
    }
}

/// Add hexadecimal strings, base64 and xor functions to arrays of bytes.
pub trait BytesCrypto {
    fn bytes2hex(&self) -> String;
    fn base64_encode(&self) -> String;
    fn xor(&self, other: &[u8]) -> Vec<u8>;
    fn hamming_distance(&self, other: &[u8]) -> u32;
}

impl BytesCrypto for [u8] {
    /// Convert an array of bytes to a hex string
     ///
     /// # Examples
     ///
     /// ```
     /// use cryptopals::crypto::BytesCrypto;
     ///
     /// assert_eq!("41".to_owned(), vec![65].bytes2hex());
     /// assert_eq!(
     ///    "48656c6c6f2c20776f726c6421".to_owned(),
     ///    vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].bytes2hex());
     /// ```
    fn bytes2hex(&self) -> String {
        let mut s = String::with_capacity(self.len() * 2);
        for b in self {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }

    /// Convert an array of bytes to Base64
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::BytesCrypto;
    ///
    /// assert_eq!(String::from("QUJD"), [65, 66, 67].base64_encode());
    /// assert_eq!(String::from("QUJD"), vec![65, 66, 67].base64_encode());
    /// assert_eq!(String::from("SGVsbG8sIHdvcmxkIQ=="), "Hello, world!".as_bytes().base64_encode());
    /// ```
    ///
    /// # References
    ///
    /// This code is inspired by [this article](https://levelup.gitconnected.com/implementing-base64-in-rust-34ef6db1e73a).
    fn base64_encode(&self) -> String {
        self
            .chunks(3)
            .map(|chunk| {
                match chunk.len() {
                    1 => [chunk[0] >> 2, (chunk[0] & 0b00000011) << 4, 64, 64],
                    2 => [chunk[0] >> 2, (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4, (chunk[1] & 0b00001111) << 2, 64],
                    _ => [chunk[0] >> 2, (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4, (chunk[1] & 0b00001111) << 2 | (chunk[2] & 0b11000000) >> 6, chunk[2] & 0b00111111],
                }.iter()
                    .map(|x| BASE64_ALPHABET[*x as usize])
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("")
    }

    /// XOR two equal length arrays of bytes
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::BytesCrypto;
    ///
    /// assert_eq!(vec![0], vec![0].xor(&vec![0]));
    /// assert_eq!(
    ///    vec![0b11111111, 0b01101100],
    ///    vec![0b10101010, 0b11111111].xor(&vec![0b01010101, 0b10010011])
    /// );
    /// ```
    fn xor(&self, other: &[u8]) -> Vec<u8> {
        self.iter().zip(other.iter())
            .map(|(&x, &y)| x ^ y)
            .collect()
    }

    /// Compute the Hamming distance between two byte arrays
    ///
    /// # Examples
    ///
    /// ```
    /// use cryptopals::crypto::BytesCrypto;
    ///
    /// assert_eq!(
    ///            37,
    ///            "this is a test".as_bytes()
    ///                .hamming_distance("wokka wokka!!!".as_bytes()));
    /// ```
    fn hamming_distance(&self, other: &[u8]) -> u32 {
        assert_eq!(self.len(), other.len(), "bytes arrays differ in size");

        self.iter().zip(other.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum::<u32>()
    }
}


/// Decrypt a XORed text using a frequency table
///
/// # Examples:
///
/// ```
/// use cryptopals::{crypto, english};
///
/// let corpus_frequency: Vec<f64> = english::get_english_frequency().unwrap();
///
/// let (text, key, euclidean_score, pearson_score) = crypto::decrypt_text("SHRDLU".as_bytes(),
///                                                                 &corpus_frequency);
/// ```
pub fn decrypt_text(input_bytes: &[u8], corpus_freq: &[f64]) -> (String, u8, f64, f64) {
    let mut best_euclidean_score = f64::INFINITY;
    let mut best_pearson_score = f64::NEG_INFINITY;
    let mut best_xor = 0;
    let mut best_string = String::new();

    // Test all values from 0 to 255 as XOR, reject invalid strings and compute
    // letter frequencies and Euclidean distance to our English corpus.
    // Keep the winner.
    for xor in 0u8..=255 {
        let xored_input: Vec<_> = input_bytes.iter()
            .map(|byte| *byte ^ xor)
            .collect();
        if let Ok(xored_string) = String::from_utf8(xored_input) {
            let xored_freq = english::calc_frequencies(&xored_string);

            let euclidean_score = english::euclidean_distance(&corpus_freq, &xored_freq);

            let pearson_score = stats::covariance(&corpus_freq, &xored_freq)
                / stats::std_dev(&corpus_freq)
                / stats::std_dev(&xored_freq);

            debug!("input xor {} = '{}'", xor, xored_string);
            debug!(" - Euclidean score: {}", euclidean_score);
            debug!(" - Pearson: {}", pearson_score);

            if euclidean_score < best_euclidean_score {
                best_euclidean_score = euclidean_score;
                best_xor = xor;
                best_string = xored_string;
                debug!(" - Best Euclidean score!");
            }
            if pearson_score > best_pearson_score {
                best_pearson_score = pearson_score;
                debug!(" - Best Pearson score!");
            }
        } else {
            debug!("input xor {} is an invalid string!", xor);
        }
    }

    (best_string, best_xor, best_euclidean_score, best_pearson_score)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex1() {
        assert_eq!(vec![65], "41".hex2bytes().unwrap());
    }

    #[test]
    fn hex_invalid_char() {
        assert!("4Z".hex2bytes().is_err());
    }

    #[test]
    fn hex_empty() {
        assert_eq!("invalid hexadecimal string", "".hex2bytes().unwrap_err().to_string());
    }

    #[test]
    fn hex_odd() {
        assert_eq!("invalid hexadecimal string", "123".hex2bytes().unwrap_err().to_string());
    }

    #[test]
    fn hex_long() {
        // The string represents "Hello, world!"
        assert_eq!(vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33],
                   "48656c6c6f2c20776f726c6421".hex2bytes().unwrap());
    }

    #[test]
    fn hex2str_short() {
        assert_eq!("A".to_owned(), "41".hex2string().unwrap());
    }

    #[test]
    fn hex2str_long() {
        assert_eq!("the kid don't play".to_owned(), "746865206b696420646f6e277420706c6179".hex2string().unwrap());
    }

    #[test]
    fn hex2str_err() {
        assert!("1020ZZ".hex2string().is_err());
    }

    #[test]
    fn base64_decode_short_string() {
        assert_eq!(
            String::from("QUJD").base64_decode().unwrap(),
            vec![65, 66, 67]
        );
    }

    #[test]
    fn base64_decode_short_str() {
        assert_eq!(
            "QUJD".base64_decode().unwrap(),
            &[65, 66, 67]
        );
    }

    #[test]
    fn base64_one_equal_sign() {
        assert_eq!(
            "QUI=".base64_decode().unwrap(),
            &[65, 66]
        );
    }

    #[test]
    fn base64_decode_classic() {
        assert_eq!(
            "SGVsbG8sIHdvcmxkIQ==".base64_decode().unwrap(),
            "Hello, world!".as_bytes()
        );
    }

    #[test]
    fn base64_decode_bad_length() {
        assert_eq!(
            "SGVsbG8sIHdvcmxkIQ=".base64_decode().unwrap_err().to_string(),
            "invalid Base64 length: 19"
        );
    }

    #[test]
    fn base64_decode_bad_char() {
        assert_eq!(
            "SGVs!G8sIHdvcmxkIQ==".base64_decode().unwrap_err().to_string(),
            "invalid byte '!' (0x21) at position 4 in Base64 string"
        );
    }

    #[test]
    fn base64_decode_bad_equal() {
        assert_eq!(
            "S=VsbG8sIHdvcmxkIQ==".base64_decode().unwrap_err().to_string(),
            "invalid byte '=' at position 1 in Base64 string"
        );
    }

    #[test]
    fn base64_decode_return() {
        assert_eq!(
            "VGhpcyBpcyBhCm11bHRpbGluZSBzdHJpbmcu".base64_decode().unwrap(),
            "This is a\nmultiline string.".as_bytes()
        );
    }

    #[test]
    fn base64_decode_return_multiple() {
        assert_eq!(
            "VGhpcyBpcyBhCgptdWx0aWxpbmUgc3RyaW5nLgo=".base64_decode().unwrap(),
            "This is a\n\nmultiline string.\n".as_bytes()
        );
    }

    #[test]
    fn bytes_empty() {
        assert_eq!("".to_owned(), [].bytes2hex());
        assert_eq!("".to_owned(), vec![].bytes2hex());
    }

    #[test]
    fn bytes_short() {
        assert_eq!("41".to_owned(), [65].bytes2hex());
    }

    #[test]
    fn bytes_small() {
        assert_eq!("09".to_owned(), vec![9].bytes2hex());
    }

    #[test]
    fn bytes_hello_world() {
        assert_eq!(
            "48656c6c6f2c20776f726c6421".to_owned(),
            vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33].bytes2hex()
        );
    }


    #[test]
    fn base64_1_byte() {
        assert_eq!(String::from("QQ=="), [65].base64_encode())
    }

    #[test]
    fn base64_2_byte() {
        assert_eq!(String::from("QUI="), [65, 66].base64_encode())
    }

    #[test]
    fn base64_3_bytes() {
        assert_eq!(String::from("QUJD"), [65, 66, 67].base64_encode())
    }

    #[test]
    fn base64_4_bytes() {
        assert_eq!(String::from("QUJDRA=="), [65, 66, 67, 68].base64_encode())
    }

    #[test]
    fn base64_5_bytes() {
        assert_eq!(String::from("QUJDREU="), [65, 66, 67, 68, 69].base64_encode())
    }

    #[test]
    fn base64_6_bytes() {
        assert_eq!(String::from("QUJDREVG"), [65, 66, 67, 68, 69, 70].base64_encode())
    }

    #[test]
    fn base64_hello_world() {
        assert_eq!(String::from("SGVsbG8sIHdvcmxkIQ=="), "Hello, world!".as_bytes().base64_encode())
    }

    #[test]
    fn xor_empty() {
        assert_eq!(vec![] as Vec<u8>, vec![].xor(&vec![]))
    }

    #[test]
    fn xor_zero() {
        assert_eq!(vec![0], vec![0].xor(&vec![0]))
    }

    #[test]
    fn xor_all_ones() {
        assert_eq!(vec![0b00000000], vec![0b11111111].xor(&vec![0b11111111]))
    }

    #[test]
    fn xor_multiple() {
        assert_eq!(
            vec![0b00000000, 0b11001100],
            vec![0b11111111, 0b11110000].xor(&vec![0b11111111, 0b00111100]))
    }

    #[test]
    fn hamming_37() {
        assert_eq!(
            37,
            "this is a test".as_bytes()
                .hamming_distance("wokka wokka!!!".as_bytes()));
    }

    #[test]
    #[should_panic]
    fn hamming_different_len() {
        assert_eq!(
            37,
            "this is a test".as_bytes()
                .hamming_distance("wokka wokka".as_bytes()));
    }
}
