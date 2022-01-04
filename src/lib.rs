//! Cross-platform native menu library.

// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/menubar/0.0.2")]
// While testing
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
pub mod appkit;

// TODO: Remove this when objc gets a new release
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(test)]
mod test_util;
