//! Windows-specific functionality.

#[cfg(windows)]
mod win32 {
    use menubar::win32::{Menu, MenuItem};
    use winit::event_loop::EventLoop;
    use winit::window::Window;

    pub(super) fn main2() {
        // Create an event loop and a winit window.
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();

        // Create a menu.
        let mut menu = Menu::new().unwrap();

        // Add some items.
        menu.push(MenuItem::new("Foo", None, || {
            println!("Foo")
        })).unwrap();

        // Add this menu to our window.
        menu.apply(&window).unwrap();

        // Begin running the event loop.
        event_loop.run(|_, _, _| {})
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
