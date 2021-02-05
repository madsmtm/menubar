use menubar::{Menu, MenuBar, MenuItem};

fn main() {
    // Dynamic content
    let recent_projects = vec!["a", "a", "a"];
    let recent_files = vec!["x", "y", "zwadjkwahjkdahsbjdlah"];

    let menu_bar = MenuBar::new(|menu| {});
    menu_bar.add("File", |menu| {
        menu.add(MenuItem::new("New File", "CMD+N", || unimplemented!()));
        menu.add(MenuItem::new("Open...", "CMD+O", || unimplemented!()));
        let open_recent_menu = Menu::new(); // Has dynamic content
        open_recent_menu.on_open(|menu| {
            menu.remove_all();
            menu.add_item(MenuItem::new(
                "Reopen Closed File",
                "SHIFT+CMD+T",
                || unimplemented!(),
            ));
            menu.add_separator();
            recent_projects.iter().for_each(|p| {
                menu.add_item(MenuItem::new(p, "", || unimplemented!()));
            });
            menu.add_separator();
            recent_files.iter().for_each(|p| {
                menu.add_item(MenuItem::new(p, "", || unimplemented!()));
            });
            menu.add_separator();
            menu.add_item(MenuItem::new("Clear items", "", || unimplemented!()));
        });
        menu.add_submenu("Open Recent", open_recent_menu);
    });
    // main_menu.set_visible(false);
    menu_bar.attach_to_application();
}
