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
