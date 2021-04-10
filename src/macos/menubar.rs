use super::menu::Menu;
use super::menuitem::{MenuElement, MenuItem};
use super::util::{to_nsstring, Id};
use objc::{class, msg_send, sel, sel_impl};

/// Helper to make constructing the menu bar easier
#[derive(Debug)]
pub struct MenuBar(Menu);

impl MenuBar {
    pub unsafe fn as_raw(&self) -> Id {
        // TMP
        self.0.as_raw()
    }

    pub unsafe fn from_raw(menu: Id) -> Self {
        // TMP
        Self(Menu::from_raw(menu))
    }

    pub fn new(f: impl FnOnce(&mut Menu) -> ()) -> Self {
        // The root menu title is irrelevant
        let menu = Menu::new();
        let mut menubar = Self(menu);
        // The first item's title is irrelevant.
        // Not sure if this is the best way to represent this?
        let mut first = Menu::new();
        f(&mut first);
        menubar.add_menu(first);
        menubar
    }

    fn add_menu(&mut self, menu: Menu) {
        // All parameters on menu items irrelevant in the menu bar
        let mut item = MenuItem::new_empty();
        item.set_submenu(Some(menu));
        self.0.add(MenuElement::Item(item));
    }

    pub fn add(&mut self, title: &str, f: impl FnOnce(&mut Menu) -> ()) {
        let mut menu = Menu::new_with_title(title);
        f(&mut menu);
        self.add_menu(menu);
    }

    #[doc(alias = "menuBarVisible")]
    fn global_visible() -> bool {
        unimplemented!()
    }

    #[doc(alias = "setMenuBarVisible")]
    #[doc(alias = "setMenuBarVisible:")]
    fn set_global_visible(visible: bool) {
        unimplemented!()
    }

    #[doc(alias = "menuBarHeight")]
    fn global_height() -> f64 {
        unimplemented!()
    }

    // How do we handle this???
    // pub fn title(index) {}
    // pub fn set_title(index, title) {}
}
