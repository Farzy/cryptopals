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
use cryptopals::crypto::{HexString, BytesCrypto};

// Set 1 / Challenge 1
pub fn main() {
    helper::section("Set 1 / Challenge 1");

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    match input.hex2bytes() {
        Ok(bytes) => {
            let x = bytes.base64_encode();

            println!("Base64({}) = {}", input, x);
            println!("String translation: {}", input.hex2string().unwrap());
            assert_eq!(output, x);
        }
        Err(e) => {
            println!("'{}' is an invalid hex string: {}", input, e)
        }
    }
}
