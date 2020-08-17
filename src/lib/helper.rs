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

//! Helper functions for displaying titles and subtitles for readability

use std::{fs, error};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

/// Display a section title
///
/// # Examples
///
/// ```
/// use cryptopals::helper;
///
/// helper::section("Statistics");
/// ```
///
/// Display:
///
/// ```text
/// +------------+
/// | Statistics |
/// +------------+
/// ```
pub fn section(title: &str) {
    let len = title.len();
    let dashes = "-".repeat(len);
    println!("\n+-{}-+", dashes);
    println!("| {} |", title);
    println!("+-{}-+", dashes);
}

/// Display a subsection title
///
/// # Examples
///
/// ```
/// use cryptopals::helper;
///
/// helper::subsection("Permutations");
/// ```
///
/// Display:
///
/// ```text
/// Permutations:
/// -------------
/// ```
#[allow(dead_code)]
pub fn subsection(title: &str) {
    let len = title.len();
    let dashes = "-".repeat(len + 1);
    println!("\n{}:", title);
    println!("{}\n", dashes);
}

/// Read a text from from an URL and cache it in /var/tmp, return the body
///
///# Examples
///
/// ```
/// use cryptopals::helper;
///
/// let body = helper::read_from_url("https://httpbin.org/base64/SFRUUEJJTiBpcyBhd2Vzb21l").unwrap();
/// assert_eq!("HTTPBIN is awesome", body);
/// ```
pub fn read_from_url(url: &str) -> Result<String, Box<dyn error::Error>> {
    // Create filename for the file cache
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let filename = format!("/var/tmp/cryptopals-{:x}.txt", hasher.finish());

    // Read file from the cache or Internet
    let body: String;
    if let Ok(body_from_file) = fs::read_to_string(&filename) {
        info!("Read text of {} from cache file {}", url, filename);
        body = body_from_file;
    } else {
        body = reqwest::blocking::get(url)?
            .text()?;
        info!("Write text from {} to cache file {}", url, filename);
        let mut f = fs::File::create(filename)?;
        f.write_all(body.as_bytes())?;
    }

    Ok(body)
}
