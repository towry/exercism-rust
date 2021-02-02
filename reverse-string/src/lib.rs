#[cfg(feature = "grapheme")]
extern crate unicode_segmentation;
#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;
#[cfg(feature = "grapheme")]
fn reverse_char(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

#[cfg(not(feature = "grapheme"))]
fn reverse_char(input: &str) -> String {
    input.chars().rev().collect::<_>()
}

pub fn reverse(input: &str) -> String {
    reverse_char(input)
}
