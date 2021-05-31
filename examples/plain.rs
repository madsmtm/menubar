#[cfg(target_os = "macos")]
use menubar::macos::{Menu, MenuItem};
use objc::rc::autoreleasepool;

fn main() {
    autoreleasepool(|_| {
        let mut menu = Menu::new();
        menu.add(MenuItem::new("item 1", "a", None));
        menu.add(MenuItem::new_separator());
        menu.add(MenuItem::new("item 2", "a", None));
    });
    autoreleasepool(|_| {
        MenuItem::new_separator();
    });
    loop {}
}
