use super::menu::Menu;
use super::menuitem::MenuItem;
use super::util::to_nsstring;
use cocoa::appkit::{CGFloat, NSApp, NSApplication};
use cocoa::base::{id, nil};
use objc::runtime::{BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};

/// Helper to make constructing the menu bar easier
#[derive(Debug)]
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

    pub unsafe fn get_global() -> Option<Self> {
        let main_menu: id = msg_send![NSApp(), mainMenu];
        if main_menu != nil {
            Some(Self(Menu::from_raw(main_menu)))
        } else {
            None
        }
    }

    pub unsafe fn attach_to_application(&self) {
        // TODO: Should we consume self here?
        let _: () = msg_send![NSApp(), setMainMenu: self.as_raw()];
    }

    pub fn global_visible() -> bool {
        let res: BOOL = unsafe { msg_send![class!(NSMenu), menuBarVisible] };
        res != NO
    }

    /// Hide or show the menubar for the entire application.
    /// This also hides or shows the yellow minimize button.
    ///
    /// Might silently fail to set the menubar visible if in fullscreen mode or similar.
    ///
    /// SAFETY: Must not be called before `applicationDidFinishLaunching` has run!
    pub unsafe fn set_global_visible(visible: bool) {
        let visible: BOOL = if visible { YES } else { NO };
        msg_send![class!(NSMenu), setMenuBarVisible: visible]
    }

    // Only available on the global menu bar object
    // pub fn global_height(&self) -> f64 {
    //     let height: CGFloat = unsafe { msg_send![self.0.as_raw(), menuBarHeight] };
    //     height
    // }
}
