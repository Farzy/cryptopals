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
use aes::Aes128;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;

// create an alias for convenience
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

const CHALLENGE7_FILE: &str = "https://cryptopals.com/static/challenge-data/7.txt";
const KEY: &str = "YELLOW SUBMARINE";

pub fn main() -> Result<(), Box<dyn Error>> {
    helper::section("Set 1 / Challenge 7");
    println!("Solving https://cryptopals.com/sets/1/challenges/7:\nAES in ECB mode\n");

    let key = KEY.as_bytes();
    let ciphertext = helper::read_from_url(CHALLENGE7_FILE)?.base64_decode()?;

    let cipher = Aes128Ecb::new_var(key, Default::default())?;
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext)?;

    println!("Decrypted AES EBC ciphertext:\n{}", String::from_utf8(decrypted_ciphertext)?);

    Ok(())
}
