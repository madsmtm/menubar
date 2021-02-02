#![windows_subsystem = "windows"]
#![allow(unused_imports)] // While testing

#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::error::Error;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;
use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    // #[cfg(target_os = "macos")]
    // let child = std::thread::spawn(move || {
    //     let app = unsafe { cocoa::appkit::NSApp() };
    //     dbg!(1, app);
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    //     dbg!(2, app);
    // });

    // #[cfg(target_os = "macos")]
    // {
    //     use cocoa::appkit::{
    //         NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSEventModifierFlags,
    //         NSMenu, NSMenuItem,
    //     };
    //     use cocoa::base::nil;
    //     use cocoa::foundation::NSAutoreleasePool;

    //     // Get a reference to the app
    //     let app = unsafe { NSApp() };
    //     dbg!(3, app);
    //     // Don't bother ensuring the activation policy is Regular!

    //     let pool = unsafe { NSAutoreleasePool::new(nil) };

    //     // let menubar = NSMenu::new(nil).autorelease();

    //     // app.setMainMenu_(menubar);

    //     unsafe { pool.drain() };

    //     child.join().unwrap();
    // }

    #[cfg(target_os = "windows")]
    let menu = {
        use std::os::windows::ffi::OsStrExt;
        use std::{ffi, mem, ptr};
        use winapi::shared::basetsd;
        use winapi::shared::minwindef;
        use winapi::shared::ntdef;
        use winapi::shared::windef;
        use winapi::um::errhandlingapi;
        use winapi::um::winuser;

        let menu = unsafe { dbg!(winuser::CreateMenu()) };

        let mut menu_text = ffi::OsStr::new("test")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        dbg!(&menu_text);

        let state = winuser::MFS_ENABLED | winuser::MFS_UNCHECKED | winuser::MFS_UNHILITE;

        let menuiteminfo = winuser::MENUITEMINFOW {
            cbSize: mem::size_of::<winuser::MENUITEMINFOW>() as minwindef::UINT,
            fMask: winuser::MIIM_STRING,         // | winuser::MIIM_TYPE,
            fType: 0,  // Maybe: winuser::MFT_STRING,     // Type set in fMask for now
            fState: 0, // Normal item. Also not MFS_DEFAULT
            wID: 0,    // Don't use ids
            hSubMenu: ptr::null_mut(), // Not a submenu
            hbmpChecked: ptr::null_mut(), // Would allow customizing the checked icon
            hbmpUnchecked: ptr::null_mut(), // Would allow customizing the unchecked icon
            dwItemData: 0 as basetsd::ULONG_PTR, // Used to specify a custom icon w. hbmpItem (I think...)
            dwTypeData: menu_text.as_mut_ptr() as ntdef::LPWSTR,
            cch: 0,
            hbmpItem: ptr::null_mut(), // Would allow customizing an icon in general
        };

        dbg!(
            menuiteminfo.cbSize,
            menuiteminfo.fMask,
            menuiteminfo.fType,
            menuiteminfo.dwTypeData,
            menuiteminfo.cch
        );

        let ptr: winuser::LPCMENUITEMINFOW = (&menuiteminfo as *const winuser::MENUITEMINFOW);

        // Insert menu item at position 0
        if unsafe { winuser::InsertMenuItemW(menu, 0xffff, minwindef::TRUE, ptr) } == 0 {
            unsafe { dbg!(errhandlingapi::GetLastError()) };
        }

        menu
    };

    let event_loop = EventLoop::new();

    let builder = WindowBuilder::new()
        .with_title("test")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 640));
    #[cfg(target_os = "macos")]
    let builder = builder.with_activation_policy(winit::platform::macos::ActivationPolicy::Regular);
    #[cfg(target_os = "windows")]
    let builder = builder
        .with_menu(menu)
        .with_theme(Some(winit::window::Theme::Light));
    let window = builder.build(&event_loop)?;

    #[cfg(target_os = "windows")]
    if let RawWindowHandle::Windows(handle) = window.raw_window_handle() {
        unsafe { dbg!(handle.hinstance, winapi::um::errhandlingapi::GetLastError()) };
    };

    dbg!(window.inner_size());

    println!("before event loop");

    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                #[cfg(target_os = "macos")]
                {
                    use cocoa::appkit::{NSApp, NSApplication};
                    use menubar::macos::menu::Menu;
                    use menubar::macos::menuitem::MenuItem;

                    let mut menubar = Menu::new("menubar");
                    menubar.add({
                        let mut menubar_item =
                            MenuItem::new("menubar item 1", "", || unimplemented!()); // Title irrelevant
                        menubar_item.set_submenu({
                            let mut menu = Menu::new("menu 1");
                            menu.add(MenuItem::new("item 1", "", || unimplemented!()));
                            menu.add(MenuItem::new("item 2", "", || unimplemented!()));
                            menu.add({
                                let mut item =
                                    MenuItem::new("item w. submenu", "", || unimplemented!());
                                item.set_submenu({
                                    let mut submenu = Menu::new("submenu 🤪©æ"); // Title irrelevant!
                                    submenu.add(MenuItem::new(
                                        "submenu item 1 🤖",
                                        "",
                                        || unimplemented!(),
                                    ));
                                    submenu.add(MenuItem::new(
                                        "submenu item 2",
                                        "",
                                        || unimplemented!(),
                                    ));
                                    Some(submenu)
                                });
                                item
                            });
                            menu.add(MenuItem::new("item 4", "", || unimplemented!()));
                            Some(menu)
                        });
                        menubar_item
                    });

                    menubar.add({
                        let mut menubar_item =
                            MenuItem::new("menubar item 2", "a", || unimplemented!()); // Title irrelevant
                        menubar_item.set_submenu({
                            let mut menu = Menu::new("menu 2");
                            menu.add(MenuItem::new("item 1", "b", || unimplemented!()));
                            menu.add(MenuItem::new("item 2", "c", || unimplemented!()));
                            menu.add(MenuItem::new("item 3", "d", || unimplemented!()));
                            Some(menu)
                        });
                        menubar_item
                    });

                    let app = unsafe { NSApp() };
                    unsafe { app.setMainMenu_(menubar.as_raw()) };
                };
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                // window.request_redraw();
                println!("MainEventsCleared");
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            Event::LoopDestroyed => {
                #[cfg(target_os = "windows")]
                unsafe {
                    winapi::um::wincon::FreeConsole()
                };
            }
            _ => (),
        }
    });
}
