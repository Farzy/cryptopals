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


use cryptopals::{helper, english, crypto};
use cryptopals::crypto::{HexString, BytesCrypto};
use std::error::Error;
use std::ops::Range;
use std::collections::HashSet;

const CHALLENGE6_FILE: &str = "https://cryptopals.com/static/challenge-data/6.txt";
const KEYSIZE_RANGE: Range<usize> = 2..42;

pub fn main() -> Result<(), Box<dyn Error>> {
    helper::section("Set 1 / Challenge 6");
    println!("Solving https://cryptopals.com/sets/1/challenges/6:\nBreak repeating-key XOR\n");

    let input = helper::read_from_url(CHALLENGE6_FILE)?.base64_decode()?;

    // let raw_output = String::from_utf8(input)?;
    // println!("Encrypted input:\n{:?}", raw_output);

    // Guess Key size
    let mut keysize_distances: Vec<(usize, f64)> = Vec::with_capacity(KEYSIZE_RANGE.len());
    let mut keysize_distances2: Vec<(usize, f64)> = Vec::with_capacity(KEYSIZE_RANGE.len());
    for keysize in KEYSIZE_RANGE {
        keysize_distances.push(
            (keysize,
             (input[0..keysize].hamming_distance(&input[keysize..(2 * keysize)])) as f64
                 / (keysize as f64)));

        let mut sum = 0.0;
        for i in 0..3 {
            sum += input[(0 * keysize)..((0 + 1) * keysize)]
                .hamming_distance(&input[((i + 1) * keysize)..((i + 2) * keysize)]) as f64;
        }
        sum /= 3.0 * (keysize as f64);
        keysize_distances2.push((keysize, sum));
    }
    keysize_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    keysize_distances2.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Keysize scores 1: {:?}",
             keysize_distances.iter()
                 .map(|(k, v)| format!("({}: {:.3})", *k, *v))
                 .collect::<Vec<_>>().join(", "));
    println!("Keysize scores 2: {:?}",
             keysize_distances2.iter()
                 .map(|(k, v)| format!("({}: {:.3})", *k, *v))
                 .collect::<Vec<_>>().join(", "));

    // Keep union for best keysizes
    let keysize_set: HashSet<usize> = keysize_distances[0..3].iter().map(|k| k.0).collect();
    let keysize_set2: HashSet<usize> = keysize_distances2[0..3].iter().map(|k| k.0).collect();
    let keysizes: Vec<_> = keysize_set.union(&keysize_set2).cloned().collect();
    println!("Most popular key sizes from first 3 entries: {:?}", keysizes);

    let corpus_freq = english::get_english_frequency()?;

    let mut best_euclidean_score = f64::INFINITY;
    let mut best_key = String::new();
    let mut best_text = String::new();

    for keysize in keysizes {
        println!("Trying keysize = {}", keysize);
        let mut transposed_strings: Vec<String> = vec![String::from(""); keysize];
        for idx_char in input.iter().cloned().enumerate() {
            transposed_strings[idx_char.0 % keysize].push(idx_char.1 as char);
        }

        let mut full_key = String::new();
        for string in transposed_strings {
            let (_, key, _, _) =
                crypto::decrypt_text(string.as_bytes(), &corpus_freq);
            full_key.push(key as char);
        }
        println!("Candidate key found: '{}'", full_key);

        // Now decode all text
        let output = input.iter()
            .zip(full_key.bytes().cycle())
            .map(|(a, b)| a ^ b)
            .collect::<Vec<u8>>();
        let text = String::from_utf8(output)?;

        let euclidean_distance = english::euclidean_distance(
            &english::calc_frequencies(&text),
            &corpus_freq
        );
        if euclidean_distance < best_euclidean_score {
            best_euclidean_score = euclidean_distance;
            best_key = full_key;
            best_text = text;
        }
    }

    println!("Best key: '{}'", best_key);
    println!("Full text:\n{}", best_text);

    Ok(())
}
