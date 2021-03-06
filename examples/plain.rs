#[cfg(target_os = "macos")]
use menubar::appkit::{NSMenu, NSMenuItem};
use objc2::rc::autoreleasepool;

fn main() {
    autoreleasepool(|_| {
        let mut menu = NSMenu::new();
        menu.add(NSMenuItem::new("item 1", "a", None));
        menu.add(NSMenuItem::new_separator());
        menu.add(NSMenuItem::new("item 2", "a", None));
    });
    autoreleasepool(|_| {
        NSMenuItem::new_separator();
    });
    loop {}
}
