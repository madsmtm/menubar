use super::menu::Menu;
use super::util::{from_nsstring, to_nsstring};
use cocoa::base::{id, nil};
use cocoa::foundation::NSInteger;
use objc::runtime::{BOOL, NO, YES};
use objc::{class, msg_send, sel, sel_impl};

struct Target; // Normal NSObject. Should return YES in worksWhenModal.
struct ActionSelector; // objc::Sel - a method selector
struct Image;

#[derive(Debug, PartialEq)]
pub enum MenuItemState {
    /// Checked
    On,
    Mixed,
    /// Unchecked
    Off,
}

#[derive(Debug)]
pub enum MenuElement {
    Separator(MenuSeparator),
    Item(MenuItem),
}

impl MenuElement {
    pub unsafe fn as_raw(&self) -> id {
        // TMP
        match self {
            Self::Separator(separator) => separator.as_raw(),
            Self::Item(item) => item.as_raw(),
        }
    }
    pub unsafe fn from_raw(element: id) -> Self {
        // TMP
        let is_separator: BOOL = msg_send![element, separatorItem];
        if is_separator != NO {
            Self::Separator(MenuSeparator::from_raw(element))
        } else {
            Self::Item(MenuItem::from_raw(element))
        }
    }

    pub fn hidden(&self) -> bool {
        match self {
            Self::Separator(separator) => separator.hidden(),
            Self::Item(item) => item.hidden(),
        }
    }
    pub fn set_hidden(&mut self, hidden: bool) {
        match self {
            Self::Separator(separator) => separator.set_hidden(hidden),
            Self::Item(item) => item.set_hidden(hidden),
        }
    }
}

#[derive(Debug)]
pub struct MenuSeparator(id);

impl MenuSeparator {
    pub fn new() -> Self {
        let separator: id = unsafe { msg_send![class!(NSMenuItem), separatorItem] };
        assert_ne!(separator, nil);
        Self(separator)
    }

    pub unsafe fn as_raw(&self) -> id {
        // TMP
        self.0
    }

    pub unsafe fn from_raw(separator: id) -> Self {
        // TMP
        Self(separator)
    }

    pub fn hidden(&self) -> bool {
        unimplemented!() // Same impl as MenuItem
    }
    pub fn set_hidden(&mut self, hidden: bool) {
        unimplemented!() // Same impl as MenuItem
    }
}

#[derive(Debug)]
pub struct MenuItem(id);

impl MenuItem {
    // Defaults:
    //     State: NSOffState
    //     On-state image: Check mark
    //     Mixed-state image: Dash

    fn alloc() -> id {
        unsafe { msg_send![class!(NSMenuItem), alloc] }
    }

    // Public only locally to allow for construction in Menubar
    pub(super) fn new_empty() -> Self {
        let item: id = unsafe { msg_send![Self::alloc(), init] };
        assert_ne!(item, nil);
        Self(item)
    }

    // Probably not: fn new() -> Self {unimplemented!()}
    pub fn new(title: &str, key_equivalent: &str, _action: impl Fn() -> ()) -> Self {
        let title = to_nsstring(title);
        let key_equivalent = to_nsstring(key_equivalent);
        let item: id = unsafe {
            msg_send![Self::alloc(), initWithTitle:title action:nil keyEquivalent:key_equivalent]
        };
        assert_ne!(item, nil);
        Self(item)
    }

    pub unsafe fn as_raw(&self) -> id {
        // TMP
        self.0
    }

    pub unsafe fn from_raw(item: id) -> Self {
        // TMP
        Self(item)
    }

    // Enabling

    fn enabled(&self) -> bool {
        unimplemented!()
    }
    fn set_enabled(&mut self, state: bool) {
        unimplemented!()
    }

    // Managing Hidden Status

    /// Whether the menu item is hidden or not.
    ///
    /// If hidden, it does not appear in a menu and does not participate in command key matching.
    pub fn hidden(&self) -> bool {
        let hidden: BOOL = unsafe { msg_send![self.0, isHidden] };
        hidden != NO
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        let hidden: BOOL = if hidden { YES } else { NO };
        unsafe { msg_send![self.0, setHidden: hidden] }
    }
    // fn hidden_or_has_hidden_ancestor(&self) -> bool {
    //     unimplemented!()
    // }

    // Target and action

    fn target(&self) -> Target {
        unimplemented!()
    }
    fn set_target(&mut self, target: Target) {
        unimplemented!()
    }
    fn action(&self) -> ActionSelector {
        unimplemented!()
    }
    fn set_action(&mut self, action: ActionSelector) {
        unimplemented!()
    }

    // Title

    pub fn title(&self) -> &str {
        let title: id = unsafe { msg_send![self.0, title] };
        unsafe { from_nsstring(title) } // Lifetimes unsure!
    }

    pub fn set_title(&mut self, title: &str) {
        let title = to_nsstring(title);
        unsafe { msg_send![self.0, setTitle: title] }
    }
    // Property attributedTitle???

    // Tag

    fn tag(&self) -> isize {
        unimplemented!()
    }
    fn set_tag(&mut self, tag: isize) {
        unimplemented!()
    }

    /// Get the menu item's state
    pub fn state(&self) -> MenuItemState {
        let state: NSInteger = unsafe { msg_send![self.0, state] };
        match state {
            1 => MenuItemState::On,
            -1 => MenuItemState::Mixed,
            0 => MenuItemState::Off,
            _ => unreachable!(),
        }
    }

    /// Set the menu item's state
    pub fn set_state(&mut self, state: MenuItemState) {
        // TODO: Link or something to these?
        // static const NSControlStateValue NSControlStateValueMixed = -1;
        // static const NSControlStateValue NSControlStateValueOff = 0;
        // static const NSControlStateValue NSControlStateValueOn = 1;

        let state = match state {
            MenuItemState::On => 1,
            MenuItemState::Mixed => -1,
            MenuItemState::Off => 0,
        };
        unsafe { msg_send![self.0, setState: state as NSInteger] }
    }

    // Images

    fn image(&self) -> Option<&Image> {
        unimplemented!()
    }
    fn set_image(&mut self, image: Option<&Image>) {
        unimplemented!()
    }
    fn image_for_state(&self, state: MenuItemState) -> Option<&Image> {
        unimplemented!()
    }
    fn set_image_for_state(&mut self, state: MenuItemState, image: Option<&Image>) {
        unimplemented!()
    }

    // Submenus

    fn submenu(&self) -> Option<&Menu> {
        unimplemented!()
    }
    pub fn set_submenu(&mut self, menu: Option<Menu>) {
        // TMP: owning Menu??
        let submenu: id = if let Some(menu) = menu {
            unsafe { menu.as_raw() }
        } else {
            nil
        };
        unsafe { msg_send![self.0, setSubmenu: submenu] }
    } // The submenu must not already have a parent!
    fn has_submenu(&self) -> bool {
        unimplemented!()
    }
    fn parent_item(&self) -> Option<&MenuItem> {
        unimplemented!()
    } // The parent submenu's menuitem

    // Owning menu

    fn parent_menu(&self) -> &Menu {
        unimplemented!()
    }
    fn set_parent_menu(&mut self, menu: &Menu) {
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
    fn set_alternate(&self, alternate: bool) {
        unimplemented!()
    }

    // Indentation level (0-15)

    fn indentation_level(&self) -> isize {
        unimplemented!()
    }
    fn set_indentation_level(&self, level: isize) {
        unimplemented!()
    }

    // Tooltop / help tag

    fn tooltip(&self) -> &str {
        unimplemented!()
    }
    fn set_tooltip(&self, tooltip: &str) {
        unimplemented!()
    }

    // Protocols: Same as Menu + "NSValidatedUserInterfaceItem"
    // This will have to be researched, is the way for the system to
    // automatically enable and disable items based on context
}
