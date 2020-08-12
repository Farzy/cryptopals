//! String, crytographic and mathematical functions


/// Convert a hex string to UTF-8 string
///
/// # Examples
///
/// ```
/// use cryptopals::crypto;
///
/// assert_eq!(String::from("A"), crypto::hex2string("41"));
/// ```
///
/// Display:
///
/// ```text
/// +------------+
/// | Statistics |
/// +------------+
/// ```
pub fn hex2string(input: &str) -> String {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap() as char)
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex1() {
        assert_eq!(String::from("A"), hex2string("41"));
    }

    #[test]
    fn hex_long() {
        assert_eq!(String::from("Hello, world!"), hex2string("48656c6c6f2c20776f726c6421"));
    }
}
