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

extern crate reqwest;

use cryptopals::{helper, english};
use cryptopals::crypto::HexString;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    helper::section("Set 1 / Challenge 3");
    println!("Solving https://cryptopals.com/sets/1/challenges/3:\nSingle-byte XOR cipher\n");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let corpus_freq = english::get_english_frequency()?;
    let input_bytes = input.hex2bytes().unwrap();

    let (solution, key, _, _) = english::decrypt_text(&input_bytes, &corpus_freq);

    println!("XOR character = '{}', string = '{}'", key as char, solution);

    Ok(())
}
