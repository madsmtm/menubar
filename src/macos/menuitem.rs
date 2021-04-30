use super::menu::Menu;
use super::util::{from_nsstring, nil, to_nsstring, Id, NSInteger};
use core::fmt;
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

#[doc(alias = "NSMenuItem")]
#[derive(PartialEq)]
pub struct MenuItem(Id);

impl MenuItem {
    // Defaults:
    //     State: NSOffState
    //     On-state image: Check mark
    //     Mixed-state image: Dash

    fn alloc() -> Id {
        unsafe { msg_send![class!(NSMenuItem), alloc] }
    }

    // Public only locally to allow for construction in Menubar
    pub(super) fn new_empty() -> Self {
        let item: Id = unsafe { msg_send![Self::alloc(), init] };
        assert_ne!(item, nil);
        Self(item)
    }

    #[doc(alias = "initWithTitle")]
    #[doc(alias = "initWithTitle:action:keyEquivalent:")]
    pub fn new(title: &str, key_equivalent: &str, _action: impl Fn() -> ()) -> Self {
        let title = to_nsstring(title);
        let key_equivalent = to_nsstring(key_equivalent);
        let item: Id = unsafe {
            msg_send![Self::alloc(), initWithTitle:title action:nil keyEquivalent:key_equivalent]
        };
        assert_ne!(item, nil);
        Self(item)
    }

    #[doc(alias = "separatorItem")]
    pub fn new_separator() -> Self {
        let separator: Id = unsafe { msg_send![class!(NSMenuItem), separatorItem] };
        assert_ne!(separator, nil);
        Self(separator)
    }

    pub unsafe fn as_raw(&self) -> Id {
        // TMP
        self.0
    }

    pub unsafe fn from_raw(item: Id) -> Self {
        // TMP
        Self(item)
    }

    // Enabling

    fn enabled(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setEnabled")]
    #[doc(alias = "setEnabled:")]
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

    #[doc(alias = "setHidden")]
    #[doc(alias = "setHidden:")]
    pub fn set_hidden(&mut self, hidden: bool) {
        let hidden: BOOL = if hidden { YES } else { NO };
        unsafe { msg_send![self.0, setHidden: hidden] }
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
    fn set_target(&mut self, target: Target) {
        unimplemented!()
    }

    fn action(&self) -> ActionSelector {
        unimplemented!()
    }

    #[doc(alias = "setAction")]
    #[doc(alias = "setAction:")]
    fn set_action(&mut self, action: ActionSelector) {
        unimplemented!()
    }

    // Title

    pub fn title(&self) -> &str {
        let title: Id = unsafe { msg_send![self.0, title] };
        assert_ne!(title, nil);
        unsafe { from_nsstring(title) } // Lifetimes unsure!
    }

    #[doc(alias = "setTitle")]
    #[doc(alias = "setTitle:")]
    pub fn set_title(&mut self, title: &str) {
        let title = to_nsstring(title);
        unsafe { msg_send![self.0, setTitle: title] }
    }

    // #[doc(alias = "attributedTitle")]
    // pub fn attributed_title(&self) -> ??? { unimplemented!() }
    // #[doc(alias = "setAttributedTitle")]
    // #[doc(alias = "setAttributedTitle:")]
    // pub fn set_attributed_title(&mut self, title: ???) { unimplemented!() }

    // Tag

    fn tag(&self) -> isize {
        unimplemented!()
    }

    #[doc(alias = "setTag")]
    #[doc(alias = "setTag:")]
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
    #[doc(alias = "setState")]
    #[doc(alias = "setState:")]
    pub fn set_state(&mut self, state: MenuItemState) {
        // TODO: Link or something to these?
        // static const NSControlStateValue NSControlStateValueMixed = -1;
        // static const NSControlStateValue NSControlStateValueOff = 0;
        // static const NSControlStateValue NSControlStateValueOn = 1;

        let state: NSInteger = match state {
            MenuItemState::On => 1,
            MenuItemState::Mixed => -1,
            MenuItemState::Off => 0,
        };
        unsafe { msg_send![self.0, setState: state] }
    }

    // Images

    fn image(&self) -> Option<&Image> {
        unimplemented!()
    }

    #[doc(alias = "setImage")]
    #[doc(alias = "setImage:")]
    fn set_image(&mut self, image: Option<&Image>) {
        unimplemented!()
    }

    #[doc(alias = "onStateImage")]
    #[doc(alias = "offStateImage")]
    #[doc(alias = "mixedStateImage")]
    fn image_for_state(&self, state: MenuItemState) -> Option<&Image> {
        unimplemented!()
    }

    #[doc(alias = "setOnStateImage")]
    #[doc(alias = "setOnStateImage:")]
    #[doc(alias = "setOffStateImage")]
    #[doc(alias = "setOffStateImage:")]
    #[doc(alias = "setMixedStateImage")]
    #[doc(alias = "setMixedStateImage:")]
    fn set_image_for_state(&mut self, state: MenuItemState, image: Option<&Image>) {
        unimplemented!()
    }

    // Submenus

    // Unsure about lifetime of the returned type
    pub fn submenu(&self) -> Option<Menu> {
        let submenu: Id = unsafe { msg_send![self.0, submenu] };
        if submenu != nil {
            Some(unsafe { Menu::from_raw(submenu) })
        } else {
            None
        }
    }

    #[doc(alias = "setSubmenu")]
    #[doc(alias = "setSubmenu:")]
    pub fn set_submenu(&mut self, menu: Option<Menu>) {
        // The submenu must not already have a parent!
        // TMP: owning Menu??
        let submenu: Id = if let Some(menu) = menu {
            unsafe { menu.as_raw() }
        } else {
            nil
        };
        unsafe { msg_send![self.0, setSubmenu: submenu] }
    }

    #[doc(alias = "hasSubmenu")]
    fn has_submenu(&self) -> bool {
        unimplemented!()
    }

    /// The parent submenu's menuitem
    #[doc(alias = "parentItem")]
    fn parent_item(&self) -> Option<&MenuItem> {
        unimplemented!()
    }

    #[doc(alias = "isSeparatorItem")]
    pub fn separator(&self) -> bool {
        // TODO: Maybe call this is_separator?
        let is_separator: BOOL = unsafe { msg_send![self.0, isSeparatorItem] };
        is_separator != NO
    }

    // Owning menu

    #[doc(alias = "menu")]
    fn parent_menu(&self) -> &Menu {
        unimplemented!()
    }

    #[doc(alias = "setMenu")]
    #[doc(alias = "setMenu:")]
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
    fn represented_object(&self) -> Id {
        unimplemented!()
    }

    #[doc(alias = "setRepresentedObject")]
    #[doc(alias = "setRepresentedObject:")]
    fn set_represented_object(&self, tooltip: Id) {
        unimplemented!()
    }

    // View - most other attributes are ignore if this is set

    fn view(&self) -> Id {
        unimplemented!()
    }

    #[doc(alias = "setView")]
    #[doc(alias = "setView:")]
    fn set_view(&self, tooltip: Id) {
        unimplemented!()
    }

    /// Get whether the menu should be drawn highlighted
    ///
    /// You should probably use the [`Menu`] delegate method "willHighlightItem"
    #[doc(alias = "isHighlighted")]
    fn highlighted(&self) -> bool {
        unimplemented!()
    }

    // Protocols: Same as Menu + "NSValidatedUserInterfaceItem"
    // This will have to be researched, is the way for the system to
    // automatically enable and disable items based on context
}

impl fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MenuItem")
            .field("id", &self.0)
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
    use super::*;
    use crate::test_util::STRINGS;

    fn get_items() -> [MenuItem; 3] {
        [
            MenuItem::new_separator(),
            MenuItem::new_empty(),
            MenuItem::new("", "", || unimplemented!()),
        ]
    }

    #[test]
    fn test_hidden() {
        get_items().iter_mut().for_each(|item| {
            assert!(!item.hidden());
            item.set_hidden(true);
            assert!(item.hidden());
            item.set_hidden(false);
            assert!(!item.hidden());
        })
    }

    #[test]
    fn test_title() {
        get_items().iter_mut().for_each(|item| {
            STRINGS.iter().for_each(|&title| {
                item.set_title(title);
                assert_eq!(item.title(), title);
            })
        })
    }

    #[test]
    fn test_title_init() {
        STRINGS.iter().for_each(|&title| {
            let item = MenuItem::new(title, "", || unimplemented!());
            assert_eq!(item.title(), title);
        })
    }

    #[test]
    fn test_title_default() {
        let item = MenuItem::new_empty();
        assert_eq!(item.title(), "NSMenuItem");
        let item = MenuItem::new_separator();
        assert_eq!(item.title(), "");
    }

    #[test]
    fn test_separator() {
        let item = MenuItem::new_separator();
        assert!(item.separator());
        let item = MenuItem::new_empty();
        assert!(!item.separator());
        let item = MenuItem::new("", "", || unimplemented!());
        assert!(!item.separator());
    }

    #[test]
    fn test_state() {
        get_items().iter_mut().for_each(|item| {
            assert_eq!(item.state(), MenuItemState::Off);
            item.set_state(MenuItemState::On);
            assert_eq!(item.state(), MenuItemState::On);
            item.set_state(MenuItemState::Mixed);
            assert_eq!(item.state(), MenuItemState::Mixed);
            item.set_state(MenuItemState::Off);
            assert_eq!(item.state(), MenuItemState::Off);
        })
    }

    #[test]
    fn test_submenu() {
        get_items().iter_mut().for_each(|item| {
            assert!(item.submenu().is_none());
            let menu = Menu::new();
            // TODO: Clean up lifetime mess
            let id = unsafe { menu.as_raw() };
            item.set_submenu(Some(menu));
            let menu = unsafe { Menu::from_raw(id) };
            assert_eq!(item.submenu(), Some(menu));
            item.set_submenu(None);
            assert!(item.submenu().is_none());
        })
    }

    #[test]
    fn test_raw() {
        let item = MenuItem::new_empty();
        let id = unsafe { item.as_raw() };
        let item2 = unsafe { MenuItem::from_raw(id) };
        assert_eq!(item, item2);
    }
}
