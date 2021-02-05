// While testing
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub use macos::menu::Menu;
#[cfg(target_os = "macos")]
pub use macos::menubar::MenuBar;
#[cfg(target_os = "macos")]
pub use macos::menuitem::MenuItem;
