use super::menu::Menu;
use super::menuitem::MenuItem;
use super::util::to_nsstring;
use cocoa::base::{id, nil};
use objc::{class, msg_send, sel, sel_impl};

/// Helper to make constructing the menu bar easier
pub struct MenuBar(Menu);

impl MenuBar {
    pub unsafe fn as_raw(&self) -> id {
        // TMP
        self.0.as_raw()
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
        self.0.add(item);
    }

    pub fn add(&mut self, title: &str, f: impl FnOnce(&mut Menu) -> ()) {
        let mut menu = Menu::new_with_title(title);
        f(&mut menu);
        self.add_menu(menu);
    }
}
