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


use cryptopals::helper;
use cryptopals::crypto::BytesCrypto;

pub fn main() {
    helper::section("Set 1 / Challenge 5");
    println!("Solving https://cryptopals.com/sets/1/challenges/5:\nImplement repeating-key XOR\n");

    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let xor = "ICE";
    let expected_output = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    let output = input.bytes().zip(xor.bytes().cycle())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .bytes2hex();

    println!("Input:\n{}", input);
    println!("ICE xored output:\n{}", output);
    assert_eq!(expected_output, output);
}
