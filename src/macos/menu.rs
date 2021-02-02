use super::menuitem::MenuItem;
use super::util::to_nsstring;
use cocoa::base::{id, nil};
use objc::{class, msg_send, sel, sel_impl};

struct MenuDelegate;

struct USize {
    height: f64,
    width: f64,
}

#[repr(transparent)]
pub struct Menu(id);

impl Menu {
    // Creating menus

    fn alloc() -> id {
        unsafe { msg_send![class!(NSMenu), alloc] }
    }

    pub unsafe fn as_raw(&self) -> id {
        // TMP
        self.0
    }

    // We could probably make a plain "new" method if we wanted

    pub fn new(title: &str) -> Self {
        let title = to_nsstring(title);
        let menu = unsafe { msg_send![Self::alloc(), initWithTitle: title] };
        assert_ne!(menu, nil);
        Menu(menu)
    }

    // Managing items

    pub fn insert(&mut self, item: MenuItem, at: usize) {
        // SAFETY:
        // - Ids are valid
        // - The item must not exist in another menu!!!!!
        //     - We need to ensure this somehow, for now we'll just consume the item!

        // assert!(at < self.len())
        unsafe { msg_send![self.0, insertItem: item.as_raw() atIndex: at] }
    }
    pub fn add(&mut self, item: MenuItem) {
        // Same safety concerns as above
        unsafe { msg_send![self.0, addItem: item.as_raw()] }
    }
    // There exists `addItemWithTitle_action_keyEquivalent`

    // fn remove(&mut self, item: &MenuItem) {
    //     unsafe { msg_send![self.0, removeItem: item.as_raw()] }
    // }
    // fn remove_at_index(&mut self, at: isize) {
    //     unimplemented!()
    // }

    /// Does not post notifications.
    pub fn remove_all(&mut self) {
        // SAFETY: Id is valid
        unsafe { msg_send![self.0, removeAllItems] }
    }

    // Finding items

    fn find_by_tag(&self, tag: isize) -> Option<&MenuItem> {
        unimplemented!()
    }
    fn find_by_title<'a>(&'a self, title: &str) -> Option<&'a MenuItem> {
        unimplemented!()
    }
    unsafe fn get_at_index(&self, at: isize) -> &MenuItem {
        unimplemented!()
    }
    fn number_of_items(&self) -> isize {
        unimplemented!()
    } // Including separators
    fn get_all_items(&self) -> &[&MenuItem] {
        unimplemented!()
    }

    // Finding indices of items

    fn index_of_item(&self, item: &MenuItem) -> Option<isize> {
        unimplemented!()
    }
    fn index_of_item_by_title(&self, title: &str) -> Option<isize> {
        unimplemented!()
    }
    fn index_of_item_by_tag(&self, tag: isize) -> Option<isize> {
        unimplemented!()
    }
    // fn index_of_by_action_and_target(&self, ...) -> isize {}
    // fn index_of_item_by_represented_object(&self, ...) -> isize {}
    fn index_of_submenu(&self, submenu: &Menu) -> Option<isize> {
        unimplemented!()
    }

    // Managing submenus

    fn set_submenu(&self, submenu: &Menu, for_item: &MenuItem) {
        unimplemented!()
    } // Unsure about this!
      // fn submenuAction(&self) {} // Overridable!
    fn get_parent(&self) -> Option<&Menu> {
        unimplemented!()
    }
    // Has more deprecated methods!

    // Enable/disable items

    // Default on
    fn autoenables_items(&self) -> bool {
        unimplemented!()
    }
    fn set_autoenables_items(&mut self, state: bool) {
        unimplemented!()
    }
    fn update_enabled_state_of_items(&self) {
        unimplemented!()
    }

    // Control fonts for this and subitems

    // fn font() -> Font {}
    // fn set_font(&mut self, font: Font) {}

    // Handling keyboard events

    // fn perform_key_equivalent(&self, event: KeyEvent) -> bool {}

    // Simulating mouse clicks

    // fn perform_action_for_item_at(&self, index: isize) {}

    // Managing title

    fn title(&self) -> &str {
        unimplemented!()
    }
    fn set_title(&self, title: &str) {
        // Lifetimes unsure
        unimplemented!()
    }

    // Managing menu bar

    fn visible(&self) -> bool {
        unimplemented!()
    }
    fn height(&self) -> f64 {
        unimplemented!()
    }
    fn set_visible(&self, visible: bool) {
        unimplemented!()
    }

    // Size

    fn min_width(&self) -> Option<f64> {
        unimplemented!() // None / zero when not set
    }
    fn set_min_width(&mut self, width: Option<f64>) {
        unimplemented!() // None ~= zero
    }
    fn size(&self) -> USize {
        unimplemented!()
    }
    fn set_size(&mut self, size: USize) {
        unimplemented!()
    } // Might change the size if too big (or small?)

    // propertiesToUpdate - for efficiency when updating items

    fn allows_ontext_menu_plug_ins(&self) -> bool {
        unimplemented!()
    }
    fn set_allows_ontext_menu_plug_ins(&mut self, state: bool) {
        unimplemented!()
    }

    // fn displayPopUpContextMenu(&mut self, event: Event, view: Option<&View>) {}
    // fn displayPopUpContextMenuWithFont(&mut self, event: Event, view: Option<&View>, font: Font) {}
    // fn displayPopUpAtMenuPositioningItem(&mut self, position_item: Option<&MenuItem>, event: Event, view: Option<&View>)

    // Whether the menu displays the state column (the "Checkmark" column for items?)
    fn show_state_column(&self) -> bool {
        unimplemented!()
    }
    fn set_show_state_column(&mut self, show: bool) {
        unimplemented!()
    }

    fn currently_highlighted_item(&self) -> Option<&MenuItem> {
        unimplemented!()
    }

    // Should honestly probably not be changed!
    // fn layout_direction() {}
    // fn set_layout_direction() {}

    // You can use the delegate to populate a menu just before it is drawn
    // and to check for key equivalents without creating a menu item.
    fn delegate(&self) -> &MenuDelegate {
        unimplemented!()

        // Events / things this delegate can respond to
        // - menuHasKeyEquivalent(menu, forEvent, target, action)
        // - updateItemBeforeDisplayed
        // - displayLocation
        // - beforeHighlightItem
        // - beforeOpen
        // - afterClose
        // - numberOfItemsInMenu // Works together with updateItemBeforeDisplayed
        //     Newly created items are blank, and then updateItemBeforeDisplayed populates them
        // - `menuNeedsUpdate` // Alternatively, if the population can happen basically instantly
        //     (and don't need to do a lot of processing beforehand), this can just be used
    }

    // Handling tracking? Perhaps just means closing/dismissing the menu?

    fn cancel_tracking(&mut self) {
        unimplemented!()
    }
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
