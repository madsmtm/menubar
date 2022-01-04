#![windows_subsystem = "windows"]
#![allow(unused_imports)] // While testing

use env_logger;
#[cfg(target_os = "macos")]
use menubar::appkit::{InitializedApplication, MenuBar, MenuItemState, NSMenu, NSMenuItem};
#[cfg(target_os = "macos")]
use objc::{
    class, msg_send,
    rc::{autoreleasepool, Owned},
    sel,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::error::Error;
#[cfg(target_os = "macos")]
use std::ptr;
#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopExtMacOS;
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
    let (menubar, window_menu, services_menu, help_menu) = {
        let mut services_menu = None;

        let mut menubar = MenuBar::new(|menu| {
            menu.add(NSMenuItem::new("item 1", "a", None));
            menu.add(NSMenuItem::new_separator());
            menu.add({
                let mut item = NSMenuItem::new("Services", "", None);
                services_menu = item.set_submenu({
                    let mut submenu = NSMenu::new();
                    submenu.add(NSMenuItem::new("will get removed or disappear?", "", None));
                    Some(submenu)
                });
                item
            });
            menu.add(NSMenuItem::new_separator());
            // let mut item = unsafe {
            //     NSMenuItem::from_raw(msg_send![class!(NSNSMenuItem), separatorItem])
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
            //     let mut submenu = NSMenu::new();
            //     submenu.add(NSMenuItem::new("submenu item", "d", None));
            //     Some(submenu)
            // });
            // menu.add(item);
            // let item = NSMenuItem::new("item 2", "b", None);
            // unsafe {
            //     let _: () = msg_send![item.as_raw(), setEnabled: 1];
            // }
            // menu.add(item);
            menu.add({
                // Unsure how key equivalents affect submenuitems???
                let mut item = NSMenuItem::new("item w. submenu", "c", None);
                item.set_submenu({
                    let mut submenu = NSMenu::new();
                    submenu.add(NSMenuItem::new("submenu item 1 🤖", "d", None));
                    submenu.add(NSMenuItem::new("submenu item 2", "e", None));
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
            let mut item = NSMenuItem::new("item x", "f", None);
            autoreleasepool(|pool| {
                assert_eq!(item.title(pool), "item x");
                item.set_title("item 4");
                assert_eq!(item.title(pool), "item 4");
            });
            menu.add(item);
        });

        menubar.add("menu hidden", |menu| {
            let item = NSMenuItem::new("item 1", "g", None);
            assert!(!item.hidden());
            menu.add(item);
            let mut item = NSMenuItem::new("item 2", "h", None);
            assert!(!item.hidden());
            item.set_hidden(true);
            assert!(item.hidden());
            item.set_hidden(false);
            assert!(!item.hidden());
            menu.add(item);
            let mut item = NSMenuItem::new("item 3", "i", None);
            item.set_hidden(true);
            menu.add(item);
        });

        let window_menu = menubar.add("Window menu", |menu| {
            menu.add(NSMenuItem::new("Will be above the window data", "", None));
        });

        menubar.add("Duplicate key equvalent", |menu| {
            menu.add(NSMenuItem::new("item 1", "j", None));
            menu.add(NSMenuItem::new("item 2", "j", None));
        });

        menubar.add("Submenus gallore", |menu| {
            menu.add({
                let mut item = NSMenuItem::new("Item 1", "", None);
                item.set_submenu({
                    let mut submenu = NSMenu::new();
                    submenu.add(NSMenuItem::new("Item 1 : 1", "", None));
                    submenu.add(NSMenuItem::new("Item 1 : 2", "", None));
                    submenu.add({
                        let mut submenuitem = NSMenuItem::new("Item 1 : 3", "", None);
                        submenuitem.set_submenu({
                            let mut submenu2 = NSMenu::new();
                            submenu2.add(NSMenuItem::new("Item 1 : 3 : 1", "", None));
                            submenu2.add({
                                let mut submenuitem2 = NSMenuItem::new("Item 1 : 3 : 2", "", None);
                                submenuitem2.set_submenu({
                                    let mut submenu3 = NSMenu::new();
                                    let mut submenuitem3 =
                                        NSMenuItem::new("Item 1 : 3 : 2 : 1", "", None);
                                    submenuitem3.set_state(MenuItemState::On);
                                    submenu3.add(submenuitem3);
                                    submenu3.add(NSMenuItem::new("Item 1 : 3 : 2 : 2", "k", None));
                                    Some(submenu3)
                                });
                                submenuitem2
                            });
                            submenu2.add(NSMenuItem::new("Item 1 : 3 : 3", "", None));
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
                let mut item = NSMenuItem::new("Item 2", "", None);
                item.set_submenu({
                    let mut submenu = NSMenu::new();
                    submenu.add(NSMenuItem::new("Item 2 : 1", "", None));
                    submenu.add(NSMenuItem::new("Item 2 : 2", "", None));
                    Some(submenu)
                });
                item
            });
        });

        menubar.add("Empty menu", |_| {});

        menubar.add("Menu with a really loooooooooooong name!", |menu| {
            menu.add(NSMenuItem::new("Item with a really loooooooooooong name!", "", None));
            menu.add(NSMenuItem::new("Item with an even looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonger name!", "", None));
            menu.add(NSMenuItem::new("Item with the looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooongest name!", "", None));
        });

        menubar.add("This menu text is truncated on smaller screens since there's too many long menus already!", |menu| {
            menu.add(NSMenuItem::new("item", "", None));
        });

        menubar.add("Length tests", |menu| {
            assert_eq!(menu.len(), 0);
            menu.add(NSMenuItem::new("item", "", None));
            assert_eq!(menu.len(), 1);
            menu.remove_all();
            assert_eq!(menu.len(), 0);
        });

        let help_menu = menubar.add("Help menu", |menu| {
            menu.add(NSMenuItem::new(
                "Will be below the help search box",
                "",
                None,
            ));
        });

        menubar.add("Insert tests", |menu| {
            menu.add(NSMenuItem::new("item 4", "", None));
            menu.insert(NSMenuItem::new("item 3", "", None), 0);
            menu.insert(NSMenuItem::new("item 1", "", None), 0);
            menu.insert(NSMenuItem::new("item 2", "", None), 1);
            menu.insert(NSMenuItem::new("item 5", "", None), 4);
        });

        // Debug print before we add a bunch of items
        println!("{:#?}", menubar);

        menubar.add("A lot of items", |menu| {
            const COUNT: usize = 65535;
            for i in 1..=COUNT {
                menu.add(NSMenuItem::new(&format!("item {}", i), "", None));
            }
            assert_eq!(menu.len(), COUNT);
        });

        (menubar, window_menu, services_menu.unwrap(), help_menu)
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
    //         NSMenu, NSNSMenuItem,
    //     };
    //     use cocoa::base::nil;
    //     use cocoa::foundation::NSAutoreleasePool;

    //     // Get a reference to the app
    //     let app = unsafe { NSApp() };
    //     dbg!(3, app);
    //     // Don't bother ensuring the activation policy is Regular!

    //     let pool = unsafe { NSAutoreleasePool::new(nil) };

    //     // let menubar = NSNSMenu::new(nil).autorelease();

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

    let mut event_loop = EventLoop::new();

    event_loop.set_activation_policy(winit::platform::macos::ActivationPolicy::Regular);

    let builder = WindowBuilder::new()
        .with_title("test")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 640));
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

    let menubar = std::cell::Cell::new(Some(menubar));

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                dbg!("Init");
                #[cfg(target_os = "macos")]
                {
                    let app = unsafe { InitializedApplication::new() };
                    app.set_window_menu(&window_menu);
                    app.set_services_menu(&services_menu);
                    app.set_help_menu(Some(&help_menu));

                    let menubar = app.set_menubar(menubar.take().unwrap());
                    autoreleasepool(|pool| assert_eq!(&*menubar, app.menubar(pool).unwrap()));
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
