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

use cryptopals::{helper, crypto};
use std::error;
use std::collections::HashMap;

// Alice in Wonderland
const GUTENBERG_CORPUS_URL: &str = "https://www.gutenberg.org/files/11/11-0.txt";

const GUTENBERG_START_MARKER: &'static str = "*** START OF THIS PROJECT GUTENBERG EBOOK";
const GUTENBERG_END_MARKER: &'static str = "*** END OF THIS PROJECT GUTENBERG EBOOK";

pub fn main() {
    helper::section("Set 1 / Challenge 3");

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let corpus = match get_corpus(GUTENBERG_CORPUS_URL) {
        Ok(corpus) => corpus,
        Err(error) => {
            eprintln!("An error happened: {}", error);
            return;
        }
    };

    let corpus_freq = calc_frequencies(&corpus);
    debug!("ETAOIN code: {}", etaoin_code(&corpus_freq));

    let mut best_euclidean_score = f64::INFINITY;
    let mut best_xor = 0;
    let mut best_string = String::new();

    let input_bytes = crypto::hex2bytes(input).unwrap();

    // Test all values from 0 to 255 as XOR, reject invalid strings and compute
    // letter frequencies and Euclidean distance to our English corpus.
    // Keep the winner.
    let corpus_freq_u8 : Vec<_> = corpus_freq.iter().map(|x| x.1).collect();
    for xor in 0u8..=255 {
        let xored_input: Vec<_> = input_bytes.iter()
            .map(|byte| *byte ^ xor)
            .collect();
        if let Ok(xored_string) = String::from_utf8(xored_input) {
            let xored_freq = calc_frequencies(&xored_string);
            let xored_freq_u8 : Vec<_> = xored_freq.iter().map(|x| x.1).collect();

            let euclidean_score = euclidean_distance(&corpus_freq, &xored_freq);

            let cov = covariance(&corpus_freq_u8, &xored_freq_u8);

            let pearson = cov / std_dev(&corpus_freq_u8) / std_dev(&xored_freq_u8);

            if pearson > 0.0 {
                debug!("input xor {} = '{}'", xor, xored_string);
                debug!(" - Euclidean score: {}", euclidean_score);
                debug!(" - Covariance: {}", cov);
                debug!(" - Pearson: {}", pearson);
                debug!(" - ETAOIN code: {}", etaoin_code(&xored_freq));

                if euclidean_score < best_euclidean_score {
                    best_euclidean_score = euclidean_score;
                    best_xor = xor;
                    best_string = xored_string;
                    debug!(" - Best score!");
                }
            }
        } else {
            debug!("input xor {} is an invalid string!", xor);
        }
    }

    println!("XOR = {}, string = '{}'", best_xor, best_string);
}


/// Compute the 12 most used letters in a frequency vector
fn etaoin_code(text_freq: &Vec<(char, f64)>) -> String {
    let mut f = text_freq.clone();
    f.sort_by(|(_, percentage1), (_, percentage2)| percentage2.partial_cmp(percentage1).unwrap());
    let mut freq_code = String::new();
    for i in 0..12 {
        freq_code.push(f[i].0);
    }
    freq_code
}


/// Compute the letter frequency in a text
///
/// The code only takes the 26 ASCII letters into consideration and ignore any other character.
fn calc_frequencies(text: &str) -> Vec<(char, f64)> {
    let mut char_count: HashMap<char, u32> = HashMap::new();
    let mut total = 0u32;
    for c in text.chars() {
        // WARNING We ignore non-ASCII letters
        if c.is_ascii_alphabetic() {
            let count = char_count.entry(c.to_ascii_uppercase()).or_insert(0);
            *count += 1;
            total += 1;
        }
    }
    debug!("Character occurrence: {:?}", char_count);

    // Store letters and their frequency in order, adding 0 for missing letters
    let mut frequencies: Vec<(char, f64)> = Vec::with_capacity(26);
    for i in 0..=25 {
        let letter = (('A' as u8) + i) as char;
        if total != 0 {
            frequencies.push((letter, *char_count.get(&letter).unwrap_or(&0) as f64 / total as f64));
        } else {
            frequencies.push((letter, 0.0));
        }
    }

    debug!("Character frequencies: {:?}", frequencies);
    frequencies
}


/// Compute the Euclidean distance between two frequency series
///
/// # References
///
/// https://www.geeksforgeeks.org/pandas-compute-the-euclidean-distance-between-two-series/
fn euclidean_distance(freq1: &[(char, f64)], freq2: &[(char, f64)]) -> f64 {
    freq1.iter().zip(freq2.iter())
        .map(|(f1, f2)| (f1.1 - f2.1).powi(2))
        .sum::<f64>()
        .sqrt()
}


/// Read an English corpus from an URL
///
/// The code supposes that the text is formatted in Project Gutenberg's
/// style.
fn get_corpus(url: &str) -> Result<String, Box<dyn error::Error>> {
    debug!("Using {} as English corpus", url);
    let body = reqwest::blocking::get(url)?
        .text()?;

    let start_marker =
        body.find(GUTENBERG_START_MARKER).ok_or("Gutenberg start marker not found")?;
    let start_text =
        start_marker
            + body[start_marker..].find("\r\n").ok_or("Missing end of line")?
            + 2;
    let end_text =
        body.find(GUTENBERG_END_MARKER).ok_or("Gutenberg end marker not found")?
            - 1;

    debug!("Body len: {}", body.len());
    debug!("Start text: {}, end text: {}", start_text, end_text);

    Ok(body[start_text..=end_text].to_owned())
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
