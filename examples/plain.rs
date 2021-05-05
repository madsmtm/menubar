#[cfg(target_os = "macos")]
use menubar::macos::{MenuBar, MenuItem};

fn main() {
    let _menubar = MenuBar::new(|menu| {
        menu.add(MenuItem::new("item 1", "a", None));
        menu.add(MenuItem::new_separator());
        menu.add(MenuItem::new("item 2", "a", None));
    });
}
