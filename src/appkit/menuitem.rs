use core::ffi;
use core::fmt;
use core::mem;
use core::ptr;

use icrate::AppKit::{
    NSControlStateValueMixed, NSControlStateValueOff, NSControlStateValueOn, NSMenu, NSMenuItem,
};
use icrate::Foundation::{NSInteger, NSString};
use objc2::rc::Id;
use objc2::runtime::{AnyObject, Sel};
use objc2::ClassType;

use super::MenuWrapper;

type Target = AnyObject; // Normal NSObject. Should return YES in worksWhenModal.
struct Image;

#[derive(Debug, PartialEq)]
pub enum MenuItemState {
    /// Checked
    On,
    Mixed,
    /// Unchecked
    Off,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MenuItemWrapper(pub Id<NSMenuItem>);

impl MenuItemWrapper {
    // Defaults:
    //     State: NSOffState
    //     On-state image: Check mark
    //     Mixed-state image: Dash

    // Public only locally to allow for construction in Menubar
    pub(super) fn new_empty() -> Self {
        Self(unsafe { NSMenuItem::init(NSMenuItem::alloc()) })
    }

    #[doc(alias = "initWithTitle")]
    #[doc(alias = "initWithTitle:action:keyEquivalent:")]
    pub fn new(title: &str, key_equivalent: &str, action: Option<Sel>) -> Self {
        let title = NSString::from_str(title);
        let key_equivalent = NSString::from_str(key_equivalent);
        Self(unsafe {
            NSMenuItem::initWithTitle_action_keyEquivalent(
                NSMenuItem::alloc(),
                &title,
                action,
                &key_equivalent,
            )
        })
    }

    #[doc(alias = "separatorItem")]
    pub fn new_separator() -> Self {
        Self(unsafe { NSMenuItem::separatorItem() })
    }

    // Enabling

    fn enabled(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setEnabled")]
    #[doc(alias = "setEnabled:")]
    fn set_enabled(&self, state: bool) {
        unimplemented!()
    }

    // Managing Hidden Status

    /// Whether the menu item is hidden or not.
    ///
    /// If hidden, it does not appear in a menu and does not participate in command key matching.
    pub fn hidden(&self) -> bool {
        unsafe { self.0.isHidden() }
    }

    #[doc(alias = "setHidden")]
    #[doc(alias = "setHidden:")]
    pub fn set_hidden(&self, hidden: bool) {
        unsafe { self.0.setHidden(hidden) }
    }

    // #[doc(alias = "hiddenOrHasHiddenAncestor")]
    // fn hidden_or_has_hidden_ancestor(&self) -> bool {
    //     unimplemented!()
    // }

    // Target and action

    fn target(&self) -> Target {
        unimplemented!()
    }

    #[doc(alias = "setTarget")]
    #[doc(alias = "setTarget:")]
    fn set_target(&self, target: Target) {
        unimplemented!()
    }

    fn action(&self) -> Sel {
        unimplemented!()
    }

    #[doc(alias = "setAction")]
    #[doc(alias = "setAction:")]
    fn set_action(&self, action: Sel) {
        unimplemented!()
    }

    // Title

    pub fn title(&self) -> String {
        unsafe { self.0.title().to_string() }
    }

    #[doc(alias = "setTitle")]
    #[doc(alias = "setTitle:")]
    pub fn set_title(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { self.0.setTitle(&title) };
    }

    // #[doc(alias = "attributedTitle")]
    // pub fn attributed_title(&self) -> ??? { unimplemented!() }
    // #[doc(alias = "setAttributedTitle")]
    // #[doc(alias = "setAttributedTitle:")]
    // pub fn set_attributed_title(&self, title: ???) { unimplemented!() }

    // Tag

    fn tag(&self) -> isize {
        unimplemented!()
    }

    #[doc(alias = "setTag")]
    #[doc(alias = "setTag:")]
    fn set_tag(&self, tag: isize) {
        unimplemented!()
    }

    /// Get the menu item's state
    pub fn state(&self) -> MenuItemState {
        let state = unsafe { self.0.state() };
        if state == NSControlStateValueOn {
            MenuItemState::On
        } else if state == NSControlStateValueMixed {
            MenuItemState::Mixed
        } else if state == NSControlStateValueOff {
            MenuItemState::Off
        } else {
            unreachable!()
        }
    }

    /// Set the menu item's state
    #[doc(alias = "setState")]
    #[doc(alias = "setState:")]
    pub fn set_state(&self, state: MenuItemState) {
        let state = match state {
            MenuItemState::On => NSControlStateValueOn,
            MenuItemState::Mixed => NSControlStateValueMixed,
            MenuItemState::Off => NSControlStateValueOff,
        };
        unsafe { self.0.setState(state) };
    }

    // Images

    fn image(&self) -> Option<&Image> {
        unimplemented!()
    }

    #[doc(alias = "setImage")]
    #[doc(alias = "setImage:")]
    fn set_image(&self, image: Option<&Image>) {
        unimplemented!()
    }

    #[doc(alias = "onStateImage")]
    #[doc(alias = "offStateImage")]
    #[doc(alias = "mixedStateImage")]
    fn image_for_state(&self, state: MenuItemState) -> Option<Id<Image>> {
        unimplemented!()
    }

    #[doc(alias = "setOnStateImage")]
    #[doc(alias = "setOnStateImage:")]
    #[doc(alias = "setOffStateImage")]
    #[doc(alias = "setOffStateImage:")]
    #[doc(alias = "setMixedStateImage")]
    #[doc(alias = "setMixedStateImage:")]
    fn set_image_for_state(&self, state: MenuItemState, image: Option<&Image>) {
        unimplemented!()
    }

    // Submenus

    pub fn submenu(&self) -> Option<MenuWrapper> {
        unsafe { self.0.submenu() }.map(MenuWrapper)
    }

    #[doc(alias = "setSubmenu")]
    #[doc(alias = "setSubmenu:")]
    pub fn set_submenu(&self, menu: Option<MenuWrapper>) -> Option<MenuWrapper> {
        // The submenu must not already have a parent!
        unsafe { self.0.setSubmenu(menu.as_ref().map(|menu| &*menu.0)) };
        menu
    }

    #[doc(alias = "hasSubmenu")]
    fn has_submenu(&self) -> bool {
        unimplemented!()
    }

    /// The parent submenu's menuitem
    #[doc(alias = "parentItem")]
    fn parent_item(&self) -> Option<Id<NSMenuItem>> {
        unimplemented!()
    }

    #[doc(alias = "isSeparatorItem")]
    pub fn separator(&self) -> bool {
        // TODO: Maybe call this is_separator?
        unsafe { self.0.isSeparatorItem() }
    }

    // Owning menu

    #[doc(alias = "menu")]
    fn parent_menu(&self) -> Id<NSMenu> {
        unimplemented!()
    }

    #[doc(alias = "setMenu")]
    #[doc(alias = "setMenu:")]
    fn set_parent_menu(&self, menu: &NSMenu) {
        unimplemented!()
    }

    // Handling keyboard events

    // fn key_equvalent()
    // fn key_equvalent_something_modifiers()
    // fn something_user_key_equvalents
    // fn user_key_equvalent() (readonly)

    // Marks the menu item as an alternate to the previous menu item

    fn alternate(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setAlternate")]
    #[doc(alias = "setAlternate:")]
    fn set_alternate(&self, alternate: bool) {
        unimplemented!()
    }

    // Indentation level (0-15)

    #[doc(alias = "indentationLevel")]
    fn indentation_level(&self) -> isize {
        unimplemented!()
    }

    #[doc(alias = "setIndentationLevel")]
    #[doc(alias = "setIndentationLevel:")]
    fn set_indentation_level(&self, level: isize) {
        unimplemented!()
    }

    // Tooltop / help tag

    #[doc(alias = "toolTip")]
    fn tooltip(&self) -> &str {
        unimplemented!()
    }

    #[doc(alias = "setToolTip")]
    #[doc(alias = "setToolTip:")]
    fn set_tooltip(&self, tooltip: &str) {
        unimplemented!()
    }

    // Represented object (kinda like tags)

    #[doc(alias = "representedObject")]
    fn represented_object(&self) -> *const AnyObject {
        unimplemented!()
    }

    #[doc(alias = "setRepresentedObject")]
    #[doc(alias = "setRepresentedObject:")]
    fn set_represented_object(&self, tooltip: *mut AnyObject) {
        unimplemented!()
    }

    // View - most other attributes are ignore if this is set

    fn view(&self) -> *const AnyObject {
        unimplemented!()
    }

    #[doc(alias = "setView")]
    #[doc(alias = "setView:")]
    fn set_view(&self, tooltip: *mut AnyObject) {
        unimplemented!()
    }

    /// Get whether the menu should be drawn highlighted
    ///
    /// You should probably use the [`NSMenu`] delegate method "willHighlightItem"
    #[doc(alias = "isHighlighted")]
    fn highlighted(&self) -> bool {
        unimplemented!()
    }

    // Protocols: Same as NSMenu + "NSValidatedUserInterfaceItem"
    // This will have to be researched, is the way for the system to
    // automatically enable and disable items based on context
}

impl fmt::Debug for MenuItemWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSMenuItem")
            .field("id", &(self as *const Self))
            .field("separator", &self.separator())
            .field("title", &self.title())
            .field("hidden", &self.hidden())
            .field("state", &self.state())
            .field("submenu", &self.submenu())
            // TODO: parent?
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use icrate::Foundation::MainThreadMarker;

    use super::*;
    use crate::{appkit::MenuWrapper, test_util::STRINGS};

    fn for_each_item(mut f: impl FnMut(&MenuItemWrapper)) {
        f(&MenuItemWrapper::new_separator());
        f(&MenuItemWrapper::new_empty());
        f(&MenuItemWrapper::new("", "", None));
    }

    #[test]
    fn test_hidden() {
        for_each_item(|item| {
            assert!(!item.hidden());
            item.set_hidden(true);
            assert!(item.hidden());
            item.set_hidden(false);
            assert!(!item.hidden());
        })
    }

    #[test]
    fn test_title() {
        for_each_item(|item| {
            STRINGS.iter().for_each(|&title| {
                item.set_title(title);
                assert_eq!(item.title(), title);
            });
        });
    }

    #[test]
    fn test_title_init() {
        STRINGS.iter().for_each(|&title| {
            let item = MenuItemWrapper::new(title, "", None);
            assert_eq!(item.title(), title);
        });
    }

    #[test]
    fn test_title_default() {
        let item = MenuItemWrapper::new_empty();
        assert_eq!(item.title(), "NSMenuItem");
        let item = MenuItemWrapper::new_separator();
        assert_eq!(item.title(), "");
    }

    #[test]
    fn test_separator() {
        let item = MenuItemWrapper::new_separator();
        assert!(item.separator());
        let item = MenuItemWrapper::new_empty();
        assert!(!item.separator());
        let item = MenuItemWrapper::new("", "", None);
        assert!(!item.separator());
    }

    #[test]
    fn test_state() {
        for_each_item(|item| {
            assert_eq!(item.state(), MenuItemState::Off);
            item.set_state(MenuItemState::On);
            assert_eq!(item.state(), MenuItemState::On);
            item.set_state(MenuItemState::Mixed);
            assert_eq!(item.state(), MenuItemState::Mixed);
            item.set_state(MenuItemState::Off);
            assert_eq!(item.state(), MenuItemState::Off);
        });
    }

    #[test]
    fn test_submenu() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        for_each_item(|item| {
            assert!(item.submenu().is_none());
            let menu = MenuWrapper::new(mtm);
            let menu = item.set_submenu(Some(menu));
            assert_eq!(item.submenu(), menu);
            item.set_submenu(None);
            assert!(item.submenu().is_none());
        })
    }
}
