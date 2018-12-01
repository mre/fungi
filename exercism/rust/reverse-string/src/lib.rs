extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(slice: &str) -> String {
    slice.graphemes(true).rev().collect()
}
