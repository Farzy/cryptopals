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

use cryptopals::helper;
use std::error;

const ALICE_WONDERLAND_URL : &str = "https://www.gutenberg.org/files/11/11-0.txt";

const GUTENBERG_START_MARKER: &'static str = "*** START OF THIS PROJECT GUTENBERG EBOOK";
const GUTENBERG_END_MARKER: &'static str = "*** END OF THIS PROJECT GUTENBERG EBOOK";

pub fn main() -> Result<(), Box<dyn error::Error>> {
    helper::section("Set 1 / Challenge 3");

    let body = reqwest::blocking::get(ALICE_WONDERLAND_URL)?
        .text()?;

    let start_marker = body.find(GUTENBERG_START_MARKER).ok_or("Gutenberg start marker not found")?;
    let end_marker = body.find(GUTENBERG_END_MARKER).ok_or("Gutenberg end marker not found")?;

    println!("Body len: {}, start: {}, end: {}", body.len(), start_marker, end_marker);
    Ok(())
}
