use icrate::AppKit::{NSApp, NSApplication, NSMenu};
use icrate::Foundation::MainThreadMarker;
use objc2::rc::Id;
use objc2::{msg_send, msg_send_id};

use super::menubar::MenuBar;
use super::MenuWrapper;

/// Helper to make various functions on the global application object safe.
pub struct InitializedApplication {
    app: Id<NSApplication>,
}

impl InitializedApplication {
    /// # Safety
    ///
    /// This must not be called before `applicationDidFinishLaunching`.
    ///
    /// In `winit`, this is at or after
    /// `winit::event::StartCause::Init` has been emitted.
    #[doc(alias = "sharedApplication")]
    pub unsafe fn new(_mtm: MainThreadMarker) -> Self {
        Self {
            app: NSApplication::sharedApplication(),
        }
    }

    /// Corresponds to the `systemMenu = "main"` key in storyboards.
    #[doc(alias = "mainMenu")]
    pub fn menubar(&self) -> Option<MenuWrapper> {
        unsafe { self.app.mainMenu() }.map(MenuWrapper)
    }

    /// Setting the menubar to `null` does not work properly, so we don't allow
    /// that functionality here!
    #[doc(alias = "setMainMenu")]
    #[doc(alias = "setMainMenu:")]
    pub fn set_menubar(&self, menubar: MenuBar) -> MenuWrapper {
        let menu = menubar.into_raw();
        unsafe { self.app.setMainMenu(Some(&menu.0)) };
        menu
    }

    /// Returns the first menu set with [`set_window_menu`]
    ///
    /// Corresponds to the `systemMenu = "window"` key in storyboards.
    #[doc(alias = "windowsMenu")]
    pub fn window_menu(&self) -> Option<MenuWrapper> {
        unsafe { self.app.windowsMenu() }.map(MenuWrapper)
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
    pub fn set_window_menu(&self, menu: &MenuWrapper) {
        unsafe { self.app.setWindowsMenu(Some(&menu.0)) }
    }

    /// Returns the first menu set with [`set_services_menu`]
    ///
    /// Corresponds to the `systemMenu = "services"` key in storyboards.
    #[doc(alias = "servicesMenu")]
    pub fn services_menu(&self) -> Option<MenuWrapper> {
        unsafe { self.app.servicesMenu() }.map(MenuWrapper)
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
    pub fn set_services_menu(&self, menu: &MenuWrapper) {
        // TODO: Is it safe to immutably set this?
        // TODO: The menu should (must?) not contain any items!
        // TODO: Setting this and pressing the close button doesn't work in winit
        unsafe { self.app.setServicesMenu(Some(&menu.0)) }
    }

    // TODO: registerServicesMenuSendTypes

    /// Get the menu that is currently assigned as the help menu, or `None` if the system is configured to autodetect this.
    ///
    /// Corresponds to the `systemMenu = "help"` key in storyboards.
    #[doc(alias = "helpMenu")]
    pub fn help_menu(&self) -> Option<MenuWrapper> {
        unsafe { self.app.helpMenu() }.map(MenuWrapper)
    }

    /// Set the global menu that should have the spotlight Help Search
    /// functionality at the top of it.
    ///
    /// If this is set to `None`, the system will place the search bar somewhere
    /// else, usually on an item named "Help" (unknown if localization applies).
    /// To prevent this, specify a menu that does not appear anywhere.
    #[doc(alias = "setHelpMenu")]
    #[doc(alias = "setHelpMenu:")]
    pub fn set_help_menu(&self, menu: Option<&MenuWrapper>) {
        // TODO: Is it safe to immutably set this?
        unsafe { self.app.setHelpMenu(menu.map(|menu| &*menu.0)) }
    }

    /// Removed from headers, this is a no-op by now.
    ///
    /// Corresponds to the `systemMenu = "apple"` key in storyboards.
    #[doc(alias = "appleMenu")]
    pub fn apple_menu(&self) -> Option<MenuWrapper> {
        let menu: Option<Id<NSMenu>> = unsafe { msg_send_id![&self.app, appleMenu] };
        menu.map(MenuWrapper)
    }

    #[doc(alias = "setAppleMenu")]
    #[doc(alias = "setAppleMenu:")]
    pub fn set_apple_menu(&self, menu: Option<&MenuWrapper>) {
        let menu: Option<&NSMenu> = menu.map(|menu| &*menu.0);
        unsafe { msg_send![&self.app, setAppleMenu: menu] }
    }

    // TODO: applicationDockMenu (the application delegate should implement this function)

    #[doc(alias = "menuBarVisible")]
    pub fn menubar_visible(&self) -> bool {
        unsafe { NSMenu::menuBarVisible() }
    }

    /// Hide or show the menubar for the entire application.
    /// This also hides or shows the yellow minimize button.
    ///
    /// Might silently fail to set the menubar visible if in fullscreen mode or similar.
    #[doc(alias = "setMenuBarVisible")]
    #[doc(alias = "setMenuBarVisible:")]
    pub fn set_menubar_visible(&self, visible: bool) {
        unsafe { NSMenu::setMenuBarVisible(visible) }
    }

    // Only available on the global menu bar object
    // #[doc(alias = "menuBarHeight")]
    // pub fn global_height(&self) -> f64 {
    //     let height: CGFloat = unsafe { msg_send![self, menuBarHeight] };
    //     height
    // }

    // TODO: `systemMenu = "recentDocuments"` key in storyboards.
    // TODO: `systemMenu = "font"` key in storyboards.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_app() -> InitializedApplication {
        unimplemented!()
    }

    fn create_menu() -> MenuWrapper {
        unimplemented!()
    }

    #[test]
    #[ignore = "not implemented"]
    fn test_services_menu() {
        let app = init_app();
        let menu1 = create_menu();
        let menu2 = create_menu();

        assert!(app.services_menu().is_none());

        app.set_services_menu(&menu1);
        assert_eq!(app.services_menu().unwrap(), menu1);

        app.set_services_menu(&menu2);
        assert_eq!(app.services_menu().unwrap(), menu2);

        // At this point `menu1` still shows as a services menu...
    }
}
