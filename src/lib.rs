// While testing
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
pub mod macos;

// TODO: Remove this when objc gets a new release
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(test)]
mod test_util;
