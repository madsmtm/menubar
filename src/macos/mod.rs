mod global;
mod menu;
mod menubar;
mod menuitem;
pub mod util; // TMP pub

pub use self::menubar::MenuBar;
pub use global::InitializedApplication;
pub use menu::Menu;
pub use menuitem::{MenuItem, MenuItemState};

// We need the Objectice-C symbols like NSString, NSMenu and so on to be available
#[link(name = "AppKit", kind = "framework")]
extern "C" {}
#[link(name = "Foundation", kind = "framework")]
extern "C" {}
