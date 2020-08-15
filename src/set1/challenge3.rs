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

// Alice in Wonderland
const GUTENBERG_CORPUS_URL: &str = "https://www.gutenberg.org/files/11/11-0.txt";


pub fn main() {
    helper::section("Set 1 / Challenge 3");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let corpus = match english::get_gutenberg_corpus(GUTENBERG_CORPUS_URL) {
        Ok(corpus) => corpus,
        Err(error) => {
            eprintln!("An error happened: {}", error);
            return;
        }
    };

    let corpus_freq = english::calc_frequencies(&corpus);

    let mut best_euclidean_score = f64::INFINITY;
    let mut best_pearson_score = f64::NEG_INFINITY;
    let mut best_xor = 0;
    let mut best_string = String::new();

    let input_bytes = input.hex2bytes().unwrap();

    // Test all values from 0 to 255 as XOR, reject invalid strings and compute
    // letter frequencies and Euclidean distance to our English corpus.
    // Keep the winner.
    for xor in 0u8..=255 {
        let xored_input: Vec<_> = input_bytes.iter()
            .map(|byte| *byte ^ xor)
            .collect();
        if let Ok(xored_string) = String::from_utf8(xored_input) {
            let xored_freq = english::calc_frequencies(&xored_string);

            let euclidean_score = english::euclidean_distance(&corpus_freq, &xored_freq);

            let pearson_score = covariance(&corpus_freq, &xored_freq)
                / std_dev(&corpus_freq)
                / std_dev(&xored_freq);

            debug!("input xor {} = '{}'", xor, xored_string);
            debug!(" - Euclidean score: {}", euclidean_score);
            debug!(" - Pearson: {}", pearson_score);

            if euclidean_score < best_euclidean_score {
                best_euclidean_score = euclidean_score;
                best_xor = xor;
                best_string = xored_string;
                debug!(" - Best Euclidean score!");
            }
            if pearson_score > best_pearson_score {
                best_pearson_score = pearson_score;
                debug!(" - Best Pearson score!");
            }
        } else {
            debug!("input xor {} is an invalid string!", xor);
        }
    }

    println!("XOR character = '{}', string = '{}'", best_xor as char, best_string);
}


fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn std_dev(values: &[f64]) -> f64 {
    let m = mean(values);
    (values.iter()
        .map(|x| (*x - m).powi(2))
        .sum::<f64>()
        / (values.len() as f64)).sqrt()
}

fn covariance(values_x: &[f64], values_y: &[f64]) -> f64 {
    assert_eq!(values_x.len(), values_y.len(), "Both arrays must be the same size");

    let mean_x = mean(values_x);
    let mean_y = mean(values_y);
    values_x.iter().zip(values_y)
        .map(|(x, y)| (*x - mean_x) * (*y - mean_y))
        .sum::<f64>() / values_x.len() as f64
}
