#[cfg(target_os = "macos")]
use menubar::appkit::{MainThreadMarker, MenuItemWrapper, MenuWrapper};

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let menu = MenuWrapper::new(mtm);
    menu.add(MenuItemWrapper::new("item 1", "a", None));
    menu.add(MenuItemWrapper::new_separator());
    menu.add(MenuItemWrapper::new("item 2", "a", None));

    let _ = MenuItemWrapper::new_separator();
    loop {}
}
