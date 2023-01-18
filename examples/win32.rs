//! Windows-specific functionality.

#[cfg(windows)]
mod win32 {
    use menubar::win32::{Menu, MenuItem};
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::EventLoop;
    use winit::window::Window;

    pub(super) fn main2() {
        // Create an event loop and a winit window.
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();

        // Create a menu.
        let mut menu = Menu::new().unwrap();

        // Add some items.
        let rust_bar = {
            let evl_target = event_loop.create_proxy();

            let mut menu = Menu::new().unwrap();
            menu.push(MenuItem::new("Blazingly Fast", None, || {
                println!("Zoom zoom!")
            }))
            .unwrap();
            menu.push(MenuItem::new("Safe", None, || {
                println!("No segfaults here!")
            }))
            .unwrap();
            menu.push(MenuItem::new("Productive", None, || {
                println!("Rust is great!")
            }))
            .unwrap();
            menu.push(MenuItem::separator()).unwrap();
            menu.push(MenuItem::new("Exit", None, move || {
                evl_target.send_event(()).unwrap();
            }))
            .unwrap();

            menu
        };

        let about_bar = {
            let mut menu = Menu::new().unwrap();
            menu.push(MenuItem::new("About", None, || println!("About!")))
                .unwrap();
            menu.push(MenuItem::new("Help", None, || {
                println!("The code *is* the documentation")
            }))
            .unwrap();
            menu
        };

        menu.push(MenuItem::submenu("Rust", rust_bar)).unwrap();
        menu.push(MenuItem::submenu("About", about_bar)).unwrap();

        // Add this menu to our window.
        menu.apply(&window).unwrap();

        // Begin running the event loop.
        event_loop.run(move |event, _, flow| {
            flow.set_wait();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window.id() == window_id => flow.set_exit(),
                Event::UserEvent(()) => flow.set_exit(),
                _ => {}
            }
        })
    }
}

#[cfg(windows)]
fn main() {
    win32::main2();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("This example only works on Windows.");
}
