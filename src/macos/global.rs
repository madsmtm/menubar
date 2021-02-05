use super::menu::Menu;
use super::menubar::MenuBar;
use super::util::to_nsstring;
use cocoa::appkit::{CGFloat, NSApp, NSApplication};
use cocoa::base::{id, nil};
use core::marker::PhantomData;
use objc::runtime::{Class, BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};

/// Helper tp make various global functions safe
pub struct InitializedApplication {
    _p: PhantomData<InitializedApplication>,
}

impl InitializedApplication {
    /// SAFETY: Must not be called before `applicationDidFinishLaunching` has run!
    pub unsafe fn new() -> Self {
        InitializedApplication { _p: PhantomData }
    }

    pub fn menubar(&self) -> Option<MenuBar> {
        let main_menu: id = unsafe { msg_send![NSApp(), mainMenu] };
        if main_menu != nil {
            Some(unsafe { MenuBar::from_raw(main_menu) })
        } else {
            None
        }
    }

    pub fn set_menubar(&self, menubar: &MenuBar) {
        // TODO: Should we consume menubar here?
        unsafe { msg_send![NSApp(), setMainMenu: menubar.as_raw()] }
    }

    pub fn menubar_visible(&self) -> bool {
        let visible: BOOL = unsafe { msg_send![class!(NSMenu), menuBarVisible] };
        visible != NO
    }

    /// Hide or show the menubar for the entire application.
    /// This also hides or shows the yellow minimize button.
    ///
    /// Might silently fail to set the menubar visible if in fullscreen mode or similar.
    pub fn set_menubar_visible(&self, visible: bool) {
        let visible: BOOL = if visible { YES } else { NO };
        unsafe { msg_send![class!(NSMenu), setMenuBarVisible: visible] }
    }

    // Only available on the global menu bar object
    // pub fn global_height(&self) -> f64 {
    //     let height: CGFloat = unsafe { msg_send![self.0.as_raw(), menuBarHeight] };
    //     height
    // }
}
