// While testing
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
pub mod macos;

pub use macos::menu::Menu;
pub use macos::menuitem::MenuItem;
