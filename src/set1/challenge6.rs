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
use cryptopals::crypto::HexString;
use std::error::Error;

const CHALLENGE6_FILE: &str = "https://cryptopals.com/static/challenge-data/6.txt";

pub fn main() -> Result<(), Box<dyn Error>> {
    helper::section("Set 1 / Challenge 6");
    println!("Solving https://cryptopals.com/sets/1/challenges/6:\nBreak repeating-key XOR\n");

    let input = helper::read_from_url(CHALLENGE6_FILE)?.base64_decode()?;

    let raw_output= String::from_utf8(input)?;
    println!("Encrypted input:\n{:?}", raw_output);

    Ok(())
}
