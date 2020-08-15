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

use std::error;

/// Compute the characters frequency in a text
///
/// The code only takes ASCII characters into consideration and ignore any other character.
pub fn calc_frequencies(text: &str) -> Vec<f64> {
    // Store characters and their frequency in order, defaulting to 0
    let mut frequencies: Vec<f64> = Vec::new();
    frequencies.resize(128, 0.0);

    let mut total = 0u32;

    for c in text.chars() {
        // WARNING We ignore non-ASCII characters
        if c.is_ascii() {
            frequencies[c.to_ascii_uppercase() as usize] += 1.0;
            total += 1;
        }
    }

    if total != 0 {
        for item in frequencies.iter_mut() {
            *item /= total as f64;
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
pub fn euclidean_distance(freq1: &[f64], freq2: &[f64]) -> f64 {
    freq1.iter().zip(freq2.iter())
        .map(|(&f1, &f2)| (f1 - f2).powi(2))
        .sum::<f64>()
        .sqrt()
}


/// Read an English corpus from an URL
///
/// The code supposes that the text is formatted in Project Gutenberg's
/// style.
pub fn get_gutenberg_corpus(url: &str) -> Result<String, Box<dyn error::Error>> {
    const GUTENBERG_START_MARKER: &str = "*** START OF THIS PROJECT GUTENBERG EBOOK";
    const GUTENBERG_END_MARKER: &str = "*** END OF THIS PROJECT GUTENBERG EBOOK";

    debug!("Using {} as English corpus", url);
    let body = reqwest::blocking::get(url)?
        .text()?;

    // Select all text between the two markers, starting on a new line
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

#[cfg(test)]
mod test {
    use super::*;

    fn empty_freq() -> Vec<f64> {
        let mut expected_f : Vec<f64> = Vec::new();
        expected_f.resize(128, 0.0);
        expected_f
    }

    #[test]
    fn freq_empty() {
        let f = calc_frequencies("");
        assert_eq!(empty_freq(), f);
    }

    #[test]
    fn freq_abc_upcase() {
        let mut expected_freq = empty_freq();
        for i in b"ABC" {
            expected_freq[*i as usize] = 1.0 / 3.0;
        }
        let f = calc_frequencies("ABC");
        assert_eq!(expected_freq, f);
    }

    #[test]
    fn freq_abc_lowcase() {
        let mut expected_freq = empty_freq();
        for i in b"ABC" {
            expected_freq[*i as usize] = 1.0 / 3.0;
        }
        let f = calc_frequencies("abc");
        assert_eq!(expected_freq, f);
    }

    #[test]
    fn freq_abc_mixed_case() {
        let mut expected_freq = empty_freq();
        for i in b"ABC" {
            expected_freq[*i as usize] = 1.0 / 3.0;
        }
        let f = calc_frequencies("aCbBcA");
        assert_eq!(expected_freq, f);
    }

    #[test]
    fn freq_mixed_chars() {
        let str = "Hello, World!";
        let strlen = str.len();
        let mut expected_freq = empty_freq();
        for i in str.to_uppercase().as_bytes() {
            expected_freq[*i as usize] += 1.0 / strlen as f64;
        }
        let f = calc_frequencies(str);
        assert_eq!(expected_freq, f);
    }

    #[test]
    fn freq_ignored_chars() {
        let str = "Bonjour à vous";
        let str_clean = "Bonjour  vous";
        let strlen = str_clean.len();

        let mut expected_freq = empty_freq();
        for i in str_clean.to_uppercase().as_bytes() {
            expected_freq[*i as usize] += 1.0 / strlen as f64;
        }
        let f = calc_frequencies(str);
        assert_eq!(expected_freq, f);
    }
}
