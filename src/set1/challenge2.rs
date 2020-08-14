use cryptopals::{helper, crypto};
use cryptopals::crypto::hex2string;

// Set 1 / Challenge 2
pub fn main() {
    helper::section("Set 1 / Challenge 2");

    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let expected_output = "746865206b696420646f6e277420706c6179";

    let output = crypto::bytes2hex(
        &crypto::xor_arrays(
            &crypto::hex2bytes(input1).unwrap(),
            &crypto::hex2bytes(input2).unwrap()
        )
    );

    println!("{} ^ {} = {}", input1, input2, output);
    println!("output = {}", hex2string(&output).unwrap());
    assert_eq!(expected_output, output);
}
