use super::menu::Menu;
use super::menuitem::MenuItem;
use objc::rc::{Owned, Retained};
use objc::{class, msg_send, sel};

/// Helper to make constructing the menu bar easier
#[derive(Debug)]
pub struct MenuBar(Owned<Menu>);

impl MenuBar {
    pub fn into_raw(self) -> Owned<Menu> {
        self.0
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

    fn add_menu<'a>(&'a mut self, menu: Owned<Menu>) -> &'a Menu {
        // All parameters on menu items irrelevant in the menu bar
        let item = MenuItem::new_empty();
        let item = self.0.add(item);
        item.set_submenu(Some(menu)).unwrap()
    }

    pub fn add<'a>(&'a mut self, title: &str, f: impl FnOnce(&mut Menu) -> ()) -> &'a Menu {
        let mut menu = Menu::new_with_title(title);
        f(&mut menu);
        self.add_menu(menu)
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
