#[cfg(target_os = "macos")]
use menubar::macos::{Menu, MenuItem};
use objc::rc::autoreleasepool;

fn main() {
    autoreleasepool(|_pool| {
        let mut menu = Menu::new();
        menu.add(MenuItem::new("item 1", "a", None));
        // menu.add(MenuItem::new_separator(pool));
        menu.add(MenuItem::new("item 2", "a", None));
    });
    autoreleasepool(|pool| {
        MenuItem::new_separator(pool);
    });
    loop {}
}
