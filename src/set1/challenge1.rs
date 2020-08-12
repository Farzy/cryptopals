use cryptopals::{helper, crypto};

// Set 1 / Challenge 1
pub fn main() {
    helper::section("Set 1 / Challenge 1");

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let x = crypto::base64_encode_u8(&crypto::hex2u8(input));

    println!("Base64({}) = {}", input, x);
    assert_eq!(output, x);
}
