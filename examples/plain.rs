#[cfg(target_os = "macos")]
use menubar::macos::{MenuBar, MenuElement, MenuItem};

fn main() {
    let _menubar = MenuBar::new(|menu| {
        menu.add(MenuElement::Item(MenuItem::new(
            "item 1",
            "a",
            || unimplemented!(),
        )));
    });
}
