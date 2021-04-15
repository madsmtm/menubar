mod global;
mod menu;
mod menubar;
mod menuitem;
pub mod util; // TMP pub

pub use self::menubar::MenuBar;
pub use global::InitializedApplication;
pub use menu::Menu;
pub use menuitem::{MenuItem, MenuItemState};

// trait HasRawRepresentation {
//     unsafe fn as_raw(&self) -> id;
//     unsafe fn from_raw(separator: id) -> Self;
// }

// We need the Objectice-C symbols like NSString, NSMenu and so on to be available
#[link(name = "AppKit", kind = "framework")]
extern "C" {}
#[link(name = "Foundation", kind = "framework")]
extern "C" {}
