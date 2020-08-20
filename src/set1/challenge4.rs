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

const CHALLENGE4_FILE: &str = "https://cryptopals.com/static/challenge-data/4.txt";

pub fn main() -> Result<(), Box<dyn Error>> {
    helper::section("Set 1 / Challenge 4");
    println!("Solving https://cryptopals.com/sets/1/challenges/4:\nDetect single-character XOR\n");

    let corpus_freq = english::get_english_frequency()?;

    let inputs = helper::read_from_url(CHALLENGE4_FILE)?;

    let mut best_euclidean_score = f64::INFINITY;
    let mut best_pearson_score = f64::NEG_INFINITY;
    let mut best_xor = 0;
    let mut best_input = String::new();
    let mut best_string = String::new();

    for input in inputs.lines() {
        debug!("Analyzing candidate '{}…", input);
        let input_bytes = input.hex2bytes().unwrap();

        let (xored_string, xor, euclidean_score, pearson_score) =
            english::decrypt_text(&input_bytes, &corpus_freq);

        if euclidean_score < best_euclidean_score {
            best_euclidean_score = euclidean_score;
            best_xor = xor;
            best_input = input.into();
            best_string = xored_string;
            debug!(" - Best Euclidean score!");
        }
        if pearson_score > best_pearson_score {
            best_pearson_score = pearson_score;
            debug!(" - Best Pearson score!");
        }
    }

    println!("Input = '{}', XOR character = '{}'.", best_input, best_xor as char);
    println!("Output = {}", best_string);

    Ok(())
}


