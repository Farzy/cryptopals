use cryptopals::{helper, crypto};

// Set 1 / Challenge 1
pub fn main() {
    helper::section("Set 1 / Challenge 1");

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    match crypto::hex2u8(input) {
        Ok(bytes) => {
            let x = crypto::base64_encode_u8(&bytes);

            println!("Base64({}) = {}", input, x);
            assert_eq!(output, x);
        },
        Err(e) => {
            println!("'{}' is an invalid hex string: {}", input, e)
        },
    }
}
