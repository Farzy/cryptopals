//! String, cryptographic and mathematical functions

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
}
