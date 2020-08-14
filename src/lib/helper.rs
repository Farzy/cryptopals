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
    let dashes = "-".repeat(len+1);
    println!("\n{}:", title);
    println!("{}\n", dashes);
}
