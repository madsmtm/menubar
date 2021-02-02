use menubar::{Menu, MenuItem};

fn main() {
    // Dynamic content
    let recent_projects = vec!["a", "a", "a"];
    let recent_files = vec!["x", "y", "zwadjkwahjkdahsbjdlah"];

    // main_menu.set_visible(false);
    let file_menu = Menu::new("File");
    file_menu.add(MenuItem::new("New File", "CMD+N", || unimplemented!()));
    file_menu.add(MenuItem::new("Open...", "CMD+O", || unimplemented!()));

    let open_recent_menu = Menu::new("Open Recent"); // Has dynamic content
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
    file_menu.add_submenu(open_recent_menu);

    let menu_bar = Menu::new_menu_bar();
    menu_bar.add_submenu(file_menu);
    menu_bar.attach_to_application();
}
