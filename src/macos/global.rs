use super::menu::Menu;
use super::menubar::MenuBar;
use super::util::{nil, to_nsstring, Id};
use core::marker::PhantomData;
use objc::runtime::{Class, BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};

/// Helper to make various functions on the global application safe
#[doc(alias = "NSApp")]
#[doc(alias = "NSApplication")]
pub struct InitializedApplication {
    ns_app: Id,
}

impl InitializedApplication {
    /// SAFETY: Must not be called before `applicationDidFinishLaunching` has run!
    ///
    /// In `winit`, this is at or after [`winit::event::StartCause::Init`] has been emitted.
    #[doc(alias = "sharedApplication")]
    pub unsafe fn new() -> Self {
        let ns_app = msg_send![class!(NSApplication), sharedApplication];
        InitializedApplication { ns_app }
    }

    #[doc(alias = "mainMenu")]
    pub fn menubar(&self) -> Option<MenuBar> {
        let main_menu: Id = unsafe { msg_send![self.ns_app, mainMenu] };
        if main_menu != nil {
            Some(unsafe { MenuBar::from_raw(main_menu) })
        } else {
            None
        }
    }

    /// Setting the menubar to `null` does not work properly, so we don't allow
    /// that functionality here!
    #[doc(alias = "setMainMenu")]
    #[doc(alias = "setMainMenu:")]
    pub fn set_menubar(&self, menubar: &MenuBar) {
        // TODO: Should we consume menubar here?
        unsafe { msg_send![self.ns_app, setMainMenu: menubar.as_raw()] }
    }

    /// Returns the first menu set with [`set_window_menu`]
    #[doc(alias = "windowsMenu")]
    pub fn window_menu(&self) -> Option<Menu> {
        let window_menu: Id = unsafe { msg_send![self.ns_app, windowsMenu] };
        if window_menu != nil {
            Some(unsafe { Menu::from_raw(window_menu) })
        } else {
            None
        }
    }

    /// Set the global window menu.
    ///
    /// The "Window: menu has items and keyboard shortcuts for entering
    /// fullscreen, managing tabs (e.g. "Show Next Tab") and a list of the
    /// application's windows.
    ///
    /// Should be called before [`set_menubar`], otherwise the window menu
    /// won't be properly populated.
    ///
    /// Un-setting the window menu (to `null`) does not work properly, so we
    /// don't expose that functionality here.
    ///
    /// Additionally, you can have luck setting the window menu more than once,
    /// though this is not recommended.
    #[doc(alias = "setWindowsMenu")]
    #[doc(alias = "setWindowsMenu:")]
    pub fn set_window_menu(&self, menu: &mut Menu) {
        let _: () = unsafe { msg_send![self.ns_app, setWindowsMenu: menu.as_raw()] };
    }

    /// Returns the first menu set with [`set_services_menu`]
    #[doc(alias = "servicesMenu")]
    pub fn services_menu(&self) -> Option<Menu> {
        let services_menu: Id = unsafe { msg_send![self.ns_app, servicesMenu] };
        if services_menu != nil {
            Some(unsafe { Menu::from_raw(services_menu) })
        } else {
            None
        }
    }

    /// Set the global services menu.
    ///
    /// The user can have a number of system configured services and
    /// corresponding keyboard shortcuts that can be accessed from this menu.
    ///
    /// Un-setting the services menu (to `null`) does not work properly, so we
    /// don't expose that functionality here.
    ///
    /// Additionally, you can sometimes have luck setting the services menu
    /// more than once, but this is really flaky.
    #[doc(alias = "setServicesMenu")]
    #[doc(alias = "setServicesMenu:")]
    pub fn set_services_menu(&self, menu: &mut Menu) {
        // TODO: The menu should (must?) not contain any items!
        // TODO: Setting this and pressing the close button doesn't work in winit
        let _: () = unsafe { msg_send![self.ns_app, setServicesMenu: menu.as_raw()] };
    }

    // TODO: registerServicesMenuSendTypes

    /// Get the menu that is currently assigned as the help menu, or `None` if the system is configured to autodetect this.
    #[doc(alias = "helpMenu")]
    pub fn help_menu(&self) -> Option<Menu> {
        let help_menu: Id = unsafe { msg_send![self.ns_app, helpMenu] };
        if help_menu != nil {
            Some(unsafe { Menu::from_raw(help_menu) })
        } else {
            None
        }
    }

    /// Set the global menu that should have the spotlight Help Search
    /// functionality at the top of it.
    ///
    /// If this is set to `None`, the system will place the search bar somewhere
    /// else, usually on an item named "Help" (unknown if localization applies).
    /// To prevent this, specify a menu that does not appear anywhere.
    #[doc(alias = "setHelpMenu")]
    #[doc(alias = "setHelpMenu:")]
    pub fn set_help_menu(&self, menu: Option<&mut Menu>) {
        let help_menu: Id = match menu {
            Some(menu) => unsafe { menu.as_raw() },
            None => nil,
        };
        let _: () = unsafe { msg_send![self.ns_app, setHelpMenu: help_menu] };
    }

    // TODO: applicationDockMenu (the application delegate should implement this function)

    #[doc(alias = "menuBarVisible")]
    pub fn menubar_visible(&self) -> bool {
        let visible: BOOL = unsafe { msg_send![class!(NSMenu), menuBarVisible] };
        visible != NO
    }

    /// Hide or show the menubar for the entire application.
    /// This also hides or shows the yellow minimize button.
    ///
    /// Might silently fail to set the menubar visible if in fullscreen mode or similar.
    #[doc(alias = "setMenuBarVisible")]
    #[doc(alias = "setMenuBarVisible:")]
    pub fn set_menubar_visible(&self, visible: bool) {
        let visible: BOOL = if visible { YES } else { NO };
        unsafe { msg_send![class!(NSMenu), setMenuBarVisible: visible] }
    }

    // Only available on the global menu bar object
    // #[doc(alias = "menuBarHeight")]
    // pub fn global_height(&self) -> f64 {
    //     let height: CGFloat = unsafe { msg_send![self.0.as_raw(), menuBarHeight] };
    //     height
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_app() -> InitializedApplication {
        unimplemented!()
    }

    fn create_menu() -> Menu {
        unimplemented!()
    }

    #[test]
    fn test_services_menu() {
        let app = init_app();
        let mut menu1 = create_menu();
        let mut menu2 = create_menu();

        assert!(app.services_menu().is_none());

        app.set_services_menu(&mut menu1);
        unsafe { assert_eq!(app.services_menu().unwrap().as_raw(), menu1.as_raw()) };

        app.set_services_menu(&mut menu2);
        unsafe { assert_eq!(app.services_menu().unwrap().as_raw(), menu2.as_raw()) };

        // At this point `menu1` still shows as a services menu...
    }
}
