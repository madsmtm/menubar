mod global;
mod menu;
mod menubar;
mod menuitem;
pub mod util; // TMP pub

pub use global::InitializedApplication;
pub use menu::Menu;
pub use menubar::MenuBar;
pub use menuitem::{MenuElement, MenuItem, MenuItemState, MenuSeparator};

// trait HasRawRepresentation {
//     unsafe fn as_raw(&self) -> id;
//     unsafe fn from_raw(separator: id) -> Self;
// }
