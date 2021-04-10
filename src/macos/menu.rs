use super::menuitem::{MenuElement, MenuItem};
use super::util::{from_nsstring, nil, to_nsstring, Id, NSInteger};
use objc::{class, msg_send, sel, sel_impl};

struct MenuDelegate;

struct USize {
    height: f64,
    width: f64,
}

#[derive(Debug)]
#[doc(alias = "NSMenu")]
pub struct Menu(Id);

impl Menu {
    // Creating menus

    fn alloc() -> Id {
        unsafe { msg_send![class!(NSMenu), alloc] }
    }

    pub unsafe fn as_raw(&self) -> Id {
        // TMP
        self.0
    }

    pub unsafe fn from_raw(menu: Id) -> Self {
        // TMP
        Self(menu)
    }

    pub fn new() -> Self {
        let menu: Id = unsafe { msg_send![Self::alloc(), init] };
        assert_ne!(menu, nil);
        Menu(menu)
    }

    // Public only locally to allow for construction in Menubar
    #[doc(alias = "initWithTitle")]
    #[doc(alias = "initWithTitle:")]
    pub(super) fn new_with_title(title: &str) -> Self {
        let title = to_nsstring(title);
        let menu: Id = unsafe { msg_send![Self::alloc(), initWithTitle: title] };
        assert_ne!(menu, nil);
        Menu(menu)
    }

    // Title (only useful for MenuBar!)

    pub(super) fn title(&self) -> &str {
        let title: Id = unsafe { msg_send![self.0, title] };
        unsafe { from_nsstring(title) } // Lifetimes unsure!
    }

    #[doc(alias = "setTitle")]
    #[doc(alias = "setTitle:")]
    pub(super) fn set_title(&mut self, title: &str) {
        let title = to_nsstring(title);
        unsafe { msg_send![self.0, setTitle: title] }
    }

    // Managing items

    /// Insert an item at the specified index.
    ///
    /// Panics if `index > menu.len()`.
    #[doc(alias = "insertItem")]
    #[doc(alias = "insertItem:atIndex:")]
    pub fn insert(&mut self, item: MenuElement, index: usize) {
        let length = self.len();
        if index > length {
            panic!(
                "Failed inserting item: Index {} larger than number of items {}",
                index, length
            );
        }
        // SAFETY:
        // - Ids are valid
        // - The item must not exist in another menu!!!!!
        //     - We need to ensure this somehow, for now we'll just consume the item!
        //     - Should maybe return a reference to the menu, where the reference is now bound to self?
        // - 0 <= index <= self.len()
        unsafe { msg_send![self.0, insertItem: item.as_raw() atIndex: index as NSInteger] }
    }

    #[doc(alias = "addItem")]
    #[doc(alias = "addItem:")]
    pub fn add(&mut self, item: MenuElement) {
        // Same safety concerns as above
        unsafe { msg_send![self.0, addItem: item.as_raw()] }
    }

    // There exists `addItemWithTitle_action_keyEquivalent`

    // Can't use this yet, we need to find a way to let users have references to menu items safely!
    // #[doc(alias = "removeItem")]
    // #[doc(alias = "removeItem:")]
    // fn remove(&mut self, item: &MenuElement) {
    //     unsafe { msg_send![self.0, removeItem: item.as_raw()] }
    // }
    // #[doc(alias = "removeItemAtIndex")]
    // #[doc(alias = "removeItemAtIndex:")]
    // fn remove_at_index(&mut self, at: isize) {
    //     unimplemented!()
    // }

    /// Does not post notifications.
    #[doc(alias = "removeAllItems")]
    pub fn remove_all(&mut self) {
        // SAFETY: Id is valid
        unsafe { msg_send![self.0, removeAllItems] }
    }

    // Finding items

    #[doc(alias = "itemWithTag")]
    #[doc(alias = "itemWithTag:")]
    fn find_by_tag(&self, tag: isize) -> Option<&MenuItem> {
        unimplemented!()
    }

    #[doc(alias = "itemWithTitle")]
    #[doc(alias = "itemWithTitle:")]
    fn find_by_title<'a>(&'a self, title: &str) -> Option<&'a MenuItem> {
        unimplemented!()
    }

    #[doc(alias = "itemAtIndex")]
    #[doc(alias = "itemAtIndex:")]
    unsafe fn get_at_index(&self, at: isize) -> &MenuElement {
        unimplemented!()
    }

    // Getting all items

    /// Number of items in this menu, including separators
    #[doc(alias = "numberOfItems")]
    pub fn len(&self) -> usize {
        let number_of_items: NSInteger = unsafe { msg_send![self.0, numberOfItems] };
        number_of_items as usize
    }

    #[doc(alias = "itemArray")]
    fn get_all_items(&self) -> &[&MenuElement] {
        unimplemented!()
    }

    // Finding indices of elements

    #[doc(alias = "indexOfItem")]
    #[doc(alias = "indexOfItem:")]
    fn index_of(&self, item: &MenuElement) -> Option<isize> {
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
    fn index_of_submenu(&self, submenu: &Menu) -> Option<isize> {
        unimplemented!()
    }

    // Managing submenus

    #[doc(alias = "setSubmenu")]
    #[doc(alias = "setSubmenu:forItem:")]
    // Unsure about this!
    fn set_submenu(&self, submenu: &Menu, for_item: &MenuItem) {
        unimplemented!()
    }

    // fn submenuAction(&self) {} // Overridable!

    #[doc(alias = "supermenu")]
    fn get_parent(&self) -> Option<&Menu> {
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
    fn set_autoenables_items(&mut self, state: bool) {
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
    // fn set_font(&mut self, font: Font) {}

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
    fn set_min_width(&mut self, width: Option<f64>) {
        // None ~= zero
        unimplemented!()
    }

    fn size(&self) -> USize {
        unimplemented!()
    }

    #[doc(alias = "setSize")]
    #[doc(alias = "setSize:")]
    fn set_size(&mut self, size: USize) {
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
    fn set_allows_context_menu_plug_ins(&mut self, state: bool) {
        unimplemented!()
    }

    // #[doc(alias = "popUpContextMenu:withEvent:forView:")]
    // fn displayPopUpContextMenu(&mut self, event: Event, view: Option<&View>) {}
    // #[doc(alias = "popUpContextMenu:withEvent:forView:withFont:")]
    // fn displayPopUpContextMenuWithFont(&mut self, event: Event, view: Option<&View>, font: Font) {}
    // #[doc(alias = "popUpMenuPositioningItem:atLocation:inView:")]
    // fn displayPopUpAtMenuPositioningItem(&mut self, position_item: Option<&MenuItem>, event: Event, view: Option<&View>)

    // Whether the menu displays the state column (the "Checkmark" column for items?)
    #[doc(alias = "showsStateColumn")]
    fn show_state_column(&self) -> bool {
        unimplemented!()
    }

    #[doc(alias = "setShowsStateColumn")]
    #[doc(alias = "setShowsStateColumn:")]
    fn set_show_state_column(&mut self, show: bool) {
        unimplemented!()
    }

    #[doc(alias = "highlightedItem")]
    fn currently_highlighted_item(&self) -> Option<&MenuItem> {
        unimplemented!()
    }

    // Should honestly probably not be changed! (userInterfaceLayoutDirection)
    // fn layout_direction() {}
    // fn set_layout_direction() {}

    // You can use the delegate to populate a menu just before it is drawn
    // and to check for key equivalents without creating a menu item.
    fn delegate(&self) -> &MenuDelegate {
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
    fn set_delegate(&self, delegate: &MenuDelegate) {
        unimplemented!()
    }

    // Handling tracking? Perhaps just means closing/dismissing the menu?

    #[doc(alias = "cancelTracking")]
    fn cancel_tracking(&mut self) {
        unimplemented!()
    }

    #[doc(alias = "cancelTrackingWithoutAnimation")]
    #[doc(alias = "cancelTrackingWithoutAnimation:")]
    fn cancel_tracking_without_animation(&mut self) {
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
