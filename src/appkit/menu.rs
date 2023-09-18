use core::fmt;

use icrate::AppKit::{NSMenu, NSMenuDelegate, NSMenuItem};
use icrate::Foundation::{MainThreadMarker, NSArray, NSSize, NSString};
use objc2::rc::Id;
use objc2::runtime::ProtocolObject;

use super::MenuItemWrapper;

/// The maximum number of items a menu can hold is 65534
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MenuWrapper(pub Id<NSMenu>);

/// Creating menus
impl MenuWrapper {
    pub fn new(mtm: MainThreadMarker) -> Self {
        Self(unsafe { NSMenu::new() })
    }

    // Public only locally to allow for construction in Menubar
    #[doc(alias = "initWithTitle")]
    #[doc(alias = "initWithTitle:")]
    pub(super) fn new_with_title(mtm: MainThreadMarker, title: &str) -> Self {
        let title = NSString::from_str(title);
        let menu = unsafe { NSMenu::initWithTitle(mtm.alloc(), &title) };
        Self(menu)
    }

    // Title (only useful for MenuBar!)

    pub(super) fn title(&self) -> String {
        unsafe { self.0.title() }.to_string()
    }

    #[doc(alias = "setTitle")]
    #[doc(alias = "setTitle:")]
    pub(super) fn set_title(&self, title: &str) {
        let title = NSString::from_str(title);
        unsafe { self.0.setTitle(&title) };
    }
}

/// Managing items
impl MenuWrapper {
    /// Insert an item at the specified index.
    ///
    /// Panics if `index > menu.len()`.
    #[doc(alias = "insertItem")]
    #[doc(alias = "insertItem:atIndex:")]
    // TODO: Reorder arguments to match `Vec::insert`?
    pub fn insert(&self, item: MenuItemWrapper, index: usize) {
        let length = self.len();
        if index > length {
            panic!(
                "Failed inserting item: Index {} larger than number of items {}",
                index, length
            );
        }
        // SAFETY:
        // - References are valid
        // - The item must not exist in another menu!!!!!
        //     - We need to ensure this somehow, for now we'll just consume the item!
        //     - Should maybe return a reference to the menu, where the reference is now bound to self?
        // - 0 <= index <= self.len()
        // TODO: Thread safety!
        unsafe { self.0.insertItem_atIndex(&item.0, index as isize) };
    }

    #[doc(alias = "addItem")]
    #[doc(alias = "addItem:")]
    pub fn add(&self, item: MenuItemWrapper) {
        // Same safety concerns as above
        unsafe { self.0.addItem(&item.0) }
    }

    // There exists `addItemWithTitle_action_keyEquivalent`

    // Can't use this yet, we need to find a way to let users have references to menu items safely!
    // #[doc(alias = "removeItem")]
    // #[doc(alias = "removeItem:")]
    // fn remove(&self, item: &NSMenuItem) {
    //     unsafe { msg_send![self, removeItem: item] }
    // }
    // #[doc(alias = "removeItemAtIndex")]
    // #[doc(alias = "removeItemAtIndex:")]
    // fn remove_at_index(&self, at: isize) {
    //     unimplemented!()
    // }

    /// Does not post notifications.
    #[doc(alias = "removeAllItems")]
    pub fn remove_all(&self) {
        // SAFETY: Reference is valid
        unsafe { self.0.removeAllItems() }
    }

    // Finding items

    #[doc(alias = "itemWithTag")]
    #[doc(alias = "itemWithTag:")]
    fn find_by_tag(&self, tag: isize) -> Option<Id<NSMenuItem>> {
        unimplemented!()
    }

    #[doc(alias = "itemWithTitle")]
    #[doc(alias = "itemWithTitle:")]
    fn find_by_title(&self, title: &str) -> Option<Id<NSMenuItem>> {
        unimplemented!()
    }

    #[doc(alias = "itemAtIndex")]
    #[doc(alias = "itemAtIndex:")]
    unsafe fn get_at_index(&self, at: isize) -> Id<NSMenuItem> {
        unimplemented!()
    }

    // Getting all items

    /// Number of items in this menu, including separators
    #[doc(alias = "numberOfItems")]
    pub fn len(&self) -> usize {
        unsafe { self.0.numberOfItems() as usize }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "itemArray")]
    fn get_all_items(&self) -> Id<NSArray<NSMenuItem>> {
        unsafe { self.0.itemArray() }
    }

    // Finding indices of elements

    #[doc(alias = "indexOfItem")]
    #[doc(alias = "indexOfItem:")]
    fn index_of(&self, item: &NSMenuItem) -> Option<isize> {
        unimplemented!()
    }

    #[doc(alias = "indexOfItemWithTitle")]
    #[doc(alias = "indexOfItemWithTitle:")]
    fn index_of_by_title(&self, title: &str) -> Option<isize> {
        unimplemented!()
    }

    #[doc(alias = "indexOfItemWithTag")]
    #[doc(alias = "indexOfItemWithTag:")]
    fn index_of_by_tag(&self, tag: isize) -> Option<isize> {
        unimplemented!()
    }

    // fn index_of_by_action_and_target(&self, ...) -> isize {}
    // fn index_of_item_by_represented_object(&self, ...) -> isize {}

    #[doc(alias = "indexOfItemWithSubmenu")]
    #[doc(alias = "indexOfItemWithSubmenu:")]
    fn index_of_submenu(&self, submenu: &NSMenu) -> Option<isize> {
        unimplemented!()
    }

    // Managing submenus

    #[doc(alias = "setSubmenu")]
    #[doc(alias = "setSubmenu:forItem:")]
    // Unsure about this!
    fn set_submenu(&self, submenu: &NSMenu, for_item: &NSMenuItem) {
        unimplemented!()
    }

    // fn submenuAction(&self) {} // Overridable!

    #[doc(alias = "supermenu")]
    fn get_parent(&self) -> Option<Id<NSMenu>> {
        unimplemented!()
    }

    // Has more deprecated methods!

    // Enable/disable items

    /// Default on
    #[doc(alias = "autoenablesItems")]
    fn autoenables_items(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setAutoenablesItems")]
    #[doc(alias = "setAutoenablesItems:")]
    fn set_autoenables_items(&self, state: bool) {
        unimplemented!()
    }

    #[doc(alias = "update")]
    fn update_enabled_state_of_items(&self) {
        unimplemented!()
    }

    // Control fonts for this and subitems

    // fn font() -> Font {}

    // #[doc(alias = "setFont")]
    // #[doc(alias = "setFont:")]
    // fn set_font(&self, font: Font) {}

    // Handling keyboard events

    // #[doc(alias = "performKeyEquivalent")]
    // #[doc(alias = "performKeyEquivalent:")]
    // fn perform_key_equivalent(&self, event: KeyEvent) -> bool {}

    // Simulating mouse clicks

    // #[doc(alias = "performActionForItemAtIndex")]
    // #[doc(alias = "performActionForItemAtIndex:")]
    // fn perform_action_for_item_at(&self, index: isize) {}

    // Size

    #[doc(alias = "minimumWidth")]
    fn min_width(&self) -> Option<f64> {
        // None / zero when not set
        unimplemented!()
    }

    #[doc(alias = "setMinimumWidth")]
    #[doc(alias = "setMinimumWidth:")]
    fn set_min_width(&self, width: Option<f64>) {
        // None ~= zero
        unimplemented!()
    }

    fn size(&self) -> NSSize {
        unimplemented!()
    }

    #[doc(alias = "setSize")]
    #[doc(alias = "setSize:")]
    fn set_size(&self, size: NSSize) {
        // Might change the size if too big (or small?)
        unimplemented!()
    }

    // propertiesToUpdate - for efficiency when updating items

    #[doc(alias = "allowsContextMenuPlugIns")]
    fn allows_context_menu_plug_ins(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setAllowsContextMenuPlugIns")]
    #[doc(alias = "setAllowsContextMenuPlugIns:")]
    fn set_allows_context_menu_plug_ins(&self, state: bool) {
        unimplemented!()
    }

    // #[doc(alias = "popUpContextMenu:withEvent:forView:")]
    // fn displayPopUpContextMenu(&self, event: Event, view: Option<&View>) {}
    // #[doc(alias = "popUpContextMenu:withEvent:forView:withFont:")]
    // fn displayPopUpContextMenuWithFont(&self, event: Event, view: Option<&View>, font: Font) {}
    // #[doc(alias = "popUpMenuPositioningItem:atLocation:inView:")]
    // fn displayPopUpAtMenuPositioningItem(&self, position_item: Option<&NSMenuItem>, event: Event, view: Option<&View>)

    // Whether the menu displays the state column (the "Checkmark" column for items?)
    #[doc(alias = "showsStateColumn")]
    fn show_state_column(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setShowsStateColumn")]
    #[doc(alias = "setShowsStateColumn:")]
    fn set_show_state_column(&self, show: bool) {
        unimplemented!()
    }

    #[doc(alias = "highlightedItem")]
    fn currently_highlighted_item(&self) -> Option<Id<NSMenuItem>> {
        unimplemented!()
    }

    // Should honestly probably not be changed! (userInterfaceLayoutDirection)
    // fn layout_direction() {}
    // fn set_layout_direction() {}

    // You can use the delegate to populate a menu just before it is drawn
    // and to check for key equivalents without creating a menu item.
    fn delegate(&self) -> Id<ProtocolObject<dyn NSMenuDelegate>> {
        // Tied to a pool or the current item?
        unimplemented!()

        // Events / things this delegate can respond to
        // - menuHasKeyEquivalent:forEvent:target:action:
        // - menu:updateItem:atIndex:shouldCancel: (update_item_before_displayed)
        // - confinementRectForMenu:onScreen: (display_location)
        // - menu:willHighlightItem: (before_highlight_item)
        // - menuWillOpen: (before_open)
        // - menuDidClose: (after_close)
        // - numberOfItemsInMenu: // Works together with updateItemBeforeDisplayed
        //     Newly created items are blank, and then updateItemBeforeDisplayed populates them
        // - menuNeedsUpdate: // Alternatively, if the population can happen basically instantly
        //     (and don't need to do a lot of processing beforehand), this can just be used
    }

    #[doc(alias = "setDelegate")]
    #[doc(alias = "setDelegate:")]
    fn set_delegate(&self, delegate: &ProtocolObject<dyn NSMenuDelegate>) {
        unimplemented!()
    }

    // Handling tracking? Perhaps just means closing/dismissing the menu?

    #[doc(alias = "cancelTracking")]
    fn cancel_tracking(&self) {
        unimplemented!()
    }

    #[doc(alias = "cancelTrackingWithoutAnimation")]
    #[doc(alias = "cancelTrackingWithoutAnimation:")]
    fn cancel_tracking_without_animation(&self) {
        unimplemented!()
    }

    // "Notifications" - not sure what these are yet!
    // - https://developer.apple.com/documentation/foundation/nsnotificationcenter?language=objc
    // - https://developer.apple.com/documentation/foundation/nsnotificationname?language=objc
    // - https://developer.apple.com/documentation/foundation/nsnotification?language=objc

    // Conforms to protocols:
    //     NSAccessibility - wow big guy [...]
    //     NSAccessibilityElement
    //     NSAppearanceCustomization - we should probably not allow editing this?
    //     NSCoding
    //     NSCopying
    //     NSUserInterfaceItemIdentification - May become important!
}

impl fmt::Debug for MenuWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NSMenu")
            .field("id", &(self as *const Self))
            .field("title", &self.title())
            // TODO: parent?
            // TODO: size and stuff
            .field("items", &self.get_all_items())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{appkit::menuitem::MenuItemWrapper, test_util::STRINGS};

    #[test]
    fn test_title() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let menu = MenuWrapper::new(mtm);
        assert_eq!(menu.title(), "");
        STRINGS.iter().for_each(|&title| {
            menu.set_title(title);
            assert_eq!(menu.title(), title);
        });
    }

    #[test]
    fn test_title_init() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        STRINGS.iter().for_each(|&title| {
            let menu = MenuWrapper::new_with_title(mtm, title);
            assert_eq!(menu.title(), title);
        });
    }

    #[test]
    fn test_length() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let menu = MenuWrapper::new(mtm);
        assert_eq!(menu.len(), 0);
        menu.add(MenuItemWrapper::new_empty());
        assert_eq!(menu.len(), 1);
        menu.add(MenuItemWrapper::new_separator());
        assert_eq!(menu.len(), 2);
        menu.add(MenuItemWrapper::new("test", "", None));
        assert_eq!(menu.len(), 3);
        menu.insert(MenuItemWrapper::new("test", "", None), 2);
        assert_eq!(menu.len(), 4);
        menu.remove_all();
        assert_eq!(menu.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let menu = MenuWrapper::new(mtm);
        assert!(menu.get_all_items().is_empty());

        // A few different iterations
        menu.add(MenuItemWrapper::new_empty());
        menu.add(MenuItemWrapper::new_empty());
        menu.add(MenuItemWrapper::new_separator());
        let mut iter = menu.get_all_items().into_iter();
        assert_eq!(iter.size_hint(), (0, Some(3)));
        assert!(unsafe { !iter.next().unwrap().isSeparatorItem() });
        assert!(unsafe { !iter.next().unwrap().isSeparatorItem() });
        assert!(unsafe { iter.next().unwrap().isSeparatorItem() });
        assert!(iter.next().is_none());

        // Modifying after creating the iterator (the iterator is unaffected)
        let mut iter = menu.get_all_items().into_iter();

        menu.add(MenuItemWrapper::new_empty());
        assert_eq!(iter.size_hint(), (0, Some(3)));
        assert!(unsafe { !iter.next().unwrap().isSeparatorItem() });

        menu.add(MenuItemWrapper::new_separator());
        assert_eq!(iter.size_hint(), (2, Some(3)));
        assert!(unsafe { !iter.next().unwrap().isSeparatorItem() });

        menu.remove_all();
        assert_eq!(iter.size_hint(), (1, Some(3)));
        assert!(unsafe { iter.next().unwrap().isSeparatorItem() });

        menu.add(MenuItemWrapper::new_separator());
        assert_eq!(iter.size_hint(), (0, Some(3)));
        assert!(iter.next().is_none());

        // Test fused-ness
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_max_count() {
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        let menu = MenuWrapper::new(mtm);
        const COUNT: usize = 65534;
        for i in 1..=COUNT {
            menu.add(MenuItemWrapper::new(&format!("item {}", i), "", None));
        }
        assert_eq!(menu.len(), COUNT);

        // The menu, if we could render it at this point, should render fine

        menu.add(MenuItemWrapper::new(
            &format!("item {}", COUNT + 1),
            "",
            None,
        ));

        // The menu item should fail rendering, and we should get an error similar to the following logged:
        // 2021-01-01 00:00:00.000 my_program[12345:678901] InsertMenuItemTextWithCFString(_principalMenuRef, (useAccessibilityTitleDescriptionTrick ? CFSTR("") : (CFStringRef)title), carbonIndex - 1, attributes, [self _menuItemCommandID]) returned error -108 on line 2638 in -[NSCarbonMenuImpl _carbonMenuInsertItem:atCarbonIndex:]
    }
}
