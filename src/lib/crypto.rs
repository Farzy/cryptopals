//! String, cryptographic and mathematical functions

const BASE64_ALPHABET : [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/', '='
];


/// Convert a hex string to an array of bytes
///
/// # Examples
///
/// ```
/// use cryptopals::crypto;
///
/// assert_eq!(vec![65], crypto::hex2u8("41"));
/// assert_eq!(vec![16, 32, 48], crypto::hex2u8("102030"));
/// ```
pub fn hex2u8(input: &str) -> Vec<u8> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}


/// Convert an array of bytes to Base64
///
/// # Examples
///
/// ```
/// use cryptopals::crypto;
///
/// assert_eq!(String::from("QUJD"), crypto::base64_encode_u8(&[65, 66, 67]));
/// assert_eq!(String::from("SGVsbG8sIHdvcmxkIQ=="), crypto::base64_encode_u8("Hello, world!".as_bytes()));
/// ```
///
/// # References
///
/// This code is inspired by this article: https://levelup.gitconnected.com/implementing-base64-in-rust-34ef6db1e73a
pub fn base64_encode_u8(bytes: &[u8]) -> String {
    bytes
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
        .into()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex1() {
        assert_eq!(vec![65], hex2u8("41"));
    }

    #[test]
    fn hex_long() {
        // The string represents "Hello, world!"
        assert_eq!(vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33],
                   hex2u8("48656c6c6f2c20776f726c6421"));
    }

    #[test]
    fn base64_1_byte() {
        assert_eq!(String::from("QQ=="), base64_encode_u8(&[65]))
    }

    #[test]
    fn base64_2_byte() {
        assert_eq!(String::from("QUI="), base64_encode_u8(&[65, 66]))
    }

    #[test]
    fn base64_3_bytes() {
        assert_eq!(String::from("QUJD"), base64_encode_u8(&[65, 66, 67]))
    }

    #[test]
    fn base64_4_bytes() {
        assert_eq!(String::from("QUJDRA=="), base64_encode_u8(&[65, 66, 67, 68]))
    }

    #[test]
    fn base64_5_bytes() {
        assert_eq!(String::from("QUJDREU="), base64_encode_u8(&[65, 66, 67, 68, 69]))
    }

    #[test]
    fn base64_6_bytes() {
        assert_eq!(String::from("QUJDREVG"), base64_encode_u8(&[65, 66, 67, 68, 69, 70]))
    }

    #[test]
    fn base64_hello_world() {
        assert_eq!(String::from("SGVsbG8sIHdvcmxkIQ=="), base64_encode_u8("Hello, world!".as_bytes()))
    }
}
