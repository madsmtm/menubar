#![windows_subsystem = "windows"]

#[cfg(target_os = "macos")]
use objc::msg_send;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::error::Error;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    #[cfg(target_os = "macos")]
    let child = std::thread::spawn(move || {
        let app = unsafe { cocoa::appkit::NSApp() };
        dbg!(1, app);
        std::thread::sleep(std::time::Duration::from_millis(500));
        dbg!(2, app);
    });

    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new().with_title("test");
    #[cfg(target_os = "macos")]
    let builder = builder.with_activation_policy(winit::platform::macos::ActivationPolicy::Regular);
    let window = builder.build(&event_loop)?;

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{
            NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSEventModifierFlags,
            NSMenu, NSMenuItem,
        };
        use cocoa::base::nil;
        use cocoa::foundation::NSAutoreleasePool;

        let window_ptr = match window.raw_window_handle() {
            RawWindowHandle::MacOS(handle) => handle.ns_window,
            _ => unreachable!(),
        };
        dbg!(4, window_ptr);

        // Get a reference to the
        let app = unsafe { NSApp() };
        dbg!(3, app);
        // Don't bother ensuring the activation policy is Regular!

        let pool = unsafe { NSAutoreleasePool::new(nil) };

        // let menubar = NSMenu::new(nil).autorelease();

        // app.setMainMenu_(menubar);

        unsafe { pool.drain() };

        child.join().unwrap();
    }

    let window = window;

    println!("hello world");

    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
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
                #[cfg(target_os = "windows")]
                unsafe {
                    winapi::um::wincon::FreeConsole()
                };
            }
            _ => (),
        }
    });
}
