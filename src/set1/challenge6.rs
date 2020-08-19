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
    helper::section("Set 1 / Challenge 6");

    let text1 = "this is a test";
    let text2 = "wokka wokka!!!";

    let hamming = text1.as_bytes().hamming_distance(text2.as_bytes());

    println!("The Hamming distance between '{}' and '{}' is {}.", text1, text2, hamming);
}
