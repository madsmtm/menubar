#[cfg(target_os = "macos")]
use menubar::appkit::{InitializedApplication, MenuBar, NSMenu, NSMenuItem};
#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Id, Owned};
use std::collections::HashMap;
use std::ptr::NonNull;
use winit::{
    event::{ElementState, Event, KeyboardInput, StartCause, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new();

    let mut windows = HashMap::new();
    for _ in 0..2 {
        let window = Window::new(&event_loop).unwrap();
        window.set_title("Outside .run");
        windows.insert(window.id(), window);
    }

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                for _ in 0..2 {
                    let window = Window::new(&event_loop).unwrap();
                    window.set_title("Inside StartCause::Init");
                    windows.insert(window.id(), window);
                }

                #[cfg(target_os = "macos")]
                {
                    autoreleasepool(|pool| {
                        let app = unsafe { InitializedApplication::new() };
                        let menubar = app.menubar(pool).unwrap();
                        // Yeah, this is not ok but we'll do it for now
                        let menubar: Id<NSMenu, Owned> =
                            unsafe { Id::retain(NonNull::from(menubar)) };
                        let mut menubar = unsafe { MenuBar::from_raw(menubar) };

                        let window_menu = menubar.add("Window menu", |menu| {
                            menu.add(NSMenuItem::new("Will be above the window data", "", None));
                        });

                        app.set_window_menu(&window_menu);
                    });
                }
            }
            Event::WindowEvent { event, window_id } => {
                match event {
                    WindowEvent::CloseRequested => {
                        // This drops the window, causing it to close.
                        windows.remove(&window_id);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        let window = Window::new(event_loop).unwrap();
                        window.set_title("Created by action");
                        windows.insert(window.id(), window);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    })
}
