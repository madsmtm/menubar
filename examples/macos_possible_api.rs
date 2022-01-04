#![allow(unused_imports)] // While testing
use menubar::macos::{MenuBar, NSMenu, NSMenuItem};

fn main() {
    // Dynamic content
    let _recent_projects = vec!["a", "a", "a"];
    let _recent_files = vec!["x", "y", "zwadjkwahjkdahsbjdlah"];

    let mut menu_bar = MenuBar::new(|_menu| {});
    menu_bar.add("File", |menu| {
        menu.add(NSMenuItem::new("New File", "CMD+N", None));
        menu.add(NSMenuItem::new("Open...", "CMD+O", None));
        // // Has dynamic content
        // let open_recent_menu = NSMenu::new();
        // open_recent_menu.on_open(|menu| {
        //     menu.remove_all();
        //     menu.add_item(NSMenuItem::new("Reopen Closed File", "SHIFT+CMD+T", None));
        //     menu.add_separator();
        //     recent_projects.iter().for_each(|p| {
        //         menu.add_item(NSMenuItem::new(p, "", None));
        //     });
        //     menu.add_separator();
        //     recent_files.iter().for_each(|p| {
        //         menu.add_item(NSMenuItem::new(p, "", None));
        //     });
        //     menu.add_separator();
        //     menu.add_item(NSMenuItem::new("Clear items", "", None));
        // });
        // menu.add_submenu("Open Recent", open_recent_menu);
    });
    // main_menu.set_visible(false);
    // unsafe { menu_bar.attach_to_application() };
}
