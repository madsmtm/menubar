#![windows_subsystem = "windows"]
#![allow(unused_imports)] // While testing

use env_logger;
#[cfg(target_os = "macos")]
use menubar::macos::{InitializedApplication, Menu, MenuBar, MenuItem, MenuItemState};
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::error::Error;
#[cfg(target_os = "macos")]
use std::ptr;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;
use winit::{
    event::{
        DeviceEvent, ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    env_logger::init();

    #[cfg(target_os = "macos")]
    let (menubar, mut window_menu, mut services_menu, mut help_menu) = {
        let mut services_menu = ptr::null_mut();

        let mut menubar = MenuBar::new(|menu| {
            menu.add(MenuItem::new("item 1", "a", || unimplemented!()));
            menu.add(MenuItem::new_separator());
            menu.add({
                let mut item = MenuItem::new("Services", "", || unimplemented!());
                item.set_submenu({
                    let mut submenu = Menu::new();
                    submenu.add(MenuItem::new(
                        "will get removed or disappear?",
                        "",
                        || unimplemented!(),
                    ));
                    services_menu = unsafe { submenu.as_raw() };
                    Some(submenu)
                });
                item
            });
            menu.add(MenuItem::new_separator());
            // let mut item = unsafe {
            //     MenuItem::from_raw(msg_send![class!(NSMenuItem), separatorItem])
            // };
            // unsafe {
            //     let _: () = msg_send![menu.as_raw(), setAutoenablesItems: 0];
            // }
            // unsafe {
            //     let _: () = msg_send![item.as_raw(), setEnabled: 1];
            // }
            // item.set_state(MenuItemState::On);
            // item.set_title("xyz");
            // item.set_hidden(true);
            // item.set_submenu({
            //     let mut submenu = Menu::new();
            //     submenu.add(MenuItem::new("submenu item", "d", || unimplemented!()));
            //     Some(submenu)
            // });
            // menu.add(item);
            // let item = MenuItem::new("item 2", "b", || unimplemented!());
            // unsafe {
            //     let _: () = msg_send![item.as_raw(), setEnabled: 1];
            // }
            // menu.add(item);
            menu.add({
                // Unsure how key equivalents affect submenuitems???
                let mut item = MenuItem::new("item w. submenu", "c", || unimplemented!());
                item.set_submenu({
                    let mut submenu = Menu::new();
                    submenu.add(MenuItem::new("submenu item 1 🤖", "d", || unimplemented!()));
                    submenu.add(MenuItem::new("submenu item 2", "e", || unimplemented!()));
                    Some(submenu)
                });
                assert_eq!(item.state(), MenuItemState::Off);
                item.set_state(MenuItemState::On);
                assert_eq!(item.state(), MenuItemState::On);
                item.set_state(MenuItemState::Mixed);
                assert_eq!(item.state(), MenuItemState::Mixed);
                item.set_state(MenuItemState::Off);
                assert_eq!(item.state(), MenuItemState::Off);
                item
            });
            let mut item = MenuItem::new("item x", "f", || unimplemented!());
            assert_eq!(item.title(), "item x");
            item.set_title("item 4");
            assert_eq!(item.title(), "item 4");
            menu.add(item);
        });

        menubar.add("menu hidden", |menu| {
            let item = MenuItem::new("item 1", "g", || unimplemented!());
            assert!(!item.hidden());
            menu.add(item);
            let mut item = MenuItem::new("item 2", "h", || unimplemented!());
            assert!(!item.hidden());
            item.set_hidden(true);
            assert!(item.hidden());
            item.set_hidden(false);
            assert!(!item.hidden());
            menu.add(item);
            let mut item = MenuItem::new("item 3", "i", || unimplemented!());
            item.set_hidden(true);
            menu.add(item);
        });

        let mut window_menu = ptr::null_mut();

        menubar.add("Window menu", |menu| {
            window_menu = unsafe { menu.as_raw() };
            menu.add(MenuItem::new(
                "Will be above the window data",
                "",
                || unimplemented!(),
            ));
        });

        menubar.add("Duplicate key equvalent", |menu| {
            menu.add(MenuItem::new("item 1", "j", || unimplemented!()));
            menu.add(MenuItem::new("item 2", "j", || unimplemented!()));
        });

        menubar.add("Submenus gallore", |menu| {
            menu.add({
                let mut item = MenuItem::new("Item 1", "", || unimplemented!());
                item.set_submenu({
                    let mut submenu = Menu::new();
                    submenu.add(MenuItem::new("Item 1 : 1", "", || unimplemented!()));
                    submenu.add(MenuItem::new("Item 1 : 2", "", || unimplemented!()));
                    submenu.add({
                        let mut submenuitem = MenuItem::new("Item 1 : 3", "", || unimplemented!());
                        submenuitem.set_submenu({
                            let mut submenu2 = Menu::new();
                            submenu2.add(MenuItem::new("Item 1 : 3 : 1", "", || unimplemented!()));
                            submenu2.add({
                                let mut submenuitem2 =
                                    MenuItem::new("Item 1 : 3 : 2", "", || unimplemented!());
                                submenuitem2.set_submenu({
                                    let mut submenu3 = Menu::new();
                                    let mut submenuitem3 = MenuItem::new(
                                        "Item 1 : 3 : 2 : 1",
                                        "",
                                        || unimplemented!(),
                                    );
                                    submenuitem3.set_state(MenuItemState::On);
                                    submenu3.add(submenuitem3);
                                    submenu3.add(MenuItem::new(
                                        "Item 1 : 3 : 2 : 2",
                                        "k",
                                        || unimplemented!(),
                                    ));
                                    Some(submenu3)
                                });
                                submenuitem2
                            });
                            submenu2.add(MenuItem::new("Item 1 : 3 : 3", "", || unimplemented!()));
                            Some(submenu2)
                        });
                        submenuitem.set_state(MenuItemState::Mixed);
                        submenuitem
                    });
                    Some(submenu)
                });
                item.set_state(MenuItemState::On);
                item
            });
            menu.add({
                let mut item = MenuItem::new("Item 2", "", || unimplemented!());
                item.set_submenu({
                    let mut submenu = Menu::new();
                    submenu.add(MenuItem::new("Item 2 : 1", "", || unimplemented!()));
                    submenu.add(MenuItem::new("Item 2 : 2", "", || unimplemented!()));
                    Some(submenu)
                });
                item
            });
        });

        menubar.add("Empty menu", |_| {});

        menubar.add("Menu with a really loooooooooooong name!", |menu| {
            menu.add(MenuItem::new("Item with a really loooooooooooong name!", "", || unimplemented!()));
            menu.add(MenuItem::new("Item with an even looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonger name!", "", || unimplemented!()));
            menu.add(MenuItem::new("Item with the looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooongest name!", "", || unimplemented!()));
        });

        menubar.add("A lot of items", |menu| {
            for i in 1..=100 {
                menu.add(MenuItem::new(
                    &format!("item {}", i),
                    "",
                    || unimplemented!(),
                ));
            }
            assert_eq!(menu.len(), 100);
        });

        menubar.add("This menu text is truncated on smaller screens since there's too many long menus already!", |menu| {
            menu.add(MenuItem::new("item", "", || unimplemented!()));
        });

        menubar.add("Length tests", |menu| {
            assert_eq!(menu.len(), 0);
            menu.add(MenuItem::new("item", "", || unimplemented!()));
            assert_eq!(menu.len(), 1);
            menu.remove_all();
            assert_eq!(menu.len(), 0);
        });

        let mut help_menu = ptr::null_mut();

        menubar.add("Help menu", |menu| {
            help_menu = unsafe { menu.as_raw() };
            menu.add(MenuItem::new(
                "Will be below the help search box",
                "",
                || unimplemented!(),
            ));
        });

        menubar.add("Insert tests", |menu| {
            menu.add(MenuItem::new("item 4", "", || unimplemented!()));
            menu.insert(MenuItem::new("item 3", "", || unimplemented!()), 0);
            menu.insert(MenuItem::new("item 1", "", || unimplemented!()), 0);
            menu.insert(MenuItem::new("item 2", "", || unimplemented!()), 1);
            menu.insert(MenuItem::new("item 5", "", || unimplemented!()), 4);
        });

        println!("{:#?}", menubar);

        unsafe {
            (
                menubar,
                Menu::from_raw(window_menu),
                Menu::from_raw(services_menu),
                Menu::from_raw(help_menu),
            )
        }
    };

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

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                dbg!("Init");
                #[cfg(target_os = "macos")]
                {
                    let app = unsafe { InitializedApplication::new() };
                    app.set_window_menu(&mut window_menu);
                    app.set_services_menu(&mut services_menu);
                    app.set_help_menu(Some(&mut help_menu));

                    app.set_menubar(&menubar);
                    unsafe { assert_eq!(menubar.as_raw(), app.menubar().unwrap().as_raw()) };
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(VirtualKeyCode::Return),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                #[cfg(target_os = "macos")]
                {
                    let app = unsafe { InitializedApplication::new() };
                    if state == ElementState::Pressed {
                        app.set_menubar_visible(true);
                        dbg!(app.menubar_visible());
                    } else {
                        app.set_menubar_visible(false);
                        dbg!(app.menubar_visible());
                    }
                };
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                window.set_fullscreen(None);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::X),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                let window = WindowBuilder::new()
                    .with_title("test2")
                    .with_inner_size(winit::dpi::LogicalSize::new(800, 640))
                    .build(&event_loop)
                    .unwrap();
                std::mem::forget(window);
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
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            Event::LoopDestroyed => {
                dbg!("Loop destroyed");
                #[cfg(target_os = "windows")]
                unsafe {
                    winapi::um::wincon::FreeConsole()
                };
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { .. },
                ..
            } => (),
            Event::DeviceEvent {
                event: DeviceEvent::Motion { .. },
                ..
            } => (),
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { .. },
                ..
            } => (),
            Event::NewEvents(StartCause::WaitCancelled {
                requested_resume: None,
                ..
            }) => (),
            Event::RedrawEventsCleared => (),
            _ => {
                // dbg!(&event);
            }
        }
    });
}
