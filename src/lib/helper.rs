//! Helper functions for displaying titles and subtitles for readability

/// Display a section title
///
/// # Examples
///
/// ```
/// use statistics::helper;
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
/// use statistics::helper;
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
