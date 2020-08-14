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
    println!("String translation = {}", hex2string(&output).unwrap());
    assert_eq!(expected_output, output);
}
