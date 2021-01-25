#![windows_subsystem = "windows"]

#[cfg(windows)]
use std::os::windows::prelude::*;
#[cfg(windows)]
use std::{ffi::OsStr, mem, ptr};

#[cfg(windows)]
use winapi::shared::{minwindef::UINT, ntdef::LPCWSTR};
#[cfg(windows)]
use winapi::um::{
    libloaderapi,
    winuser::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, MessageBoxW,
        RegisterClassExW, ShowWindow, TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW,
        CW_USEDEFAULT, LPMSG, SW_SHOW, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
    },
};

// #[cfg(windows)]
// unsafe fn wnd_proc(h_wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
//     dbg!(h_wnd, message, w_param, l_param);
// }

#[cfg(windows)]
unsafe fn windows_window() {
    let class_name: Vec<_> = OsStr::new("DesktopApp")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let class = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(DefWindowProcW),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
        hIcon: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null_mut(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut(),
    };

    if RegisterClassExW(&class) == 0 {
        MessageBoxW(
            ptr::null_mut(),
            OsStr::new("Call to RegisterClassEx failed!")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_ptr(),
            OsStr::new("Windows Desktop Guided Tour")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_ptr(),
            0,
        );
        return;
    }

    let title = OsStr::new("Windows Desktop Guided Tour Application")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    let handle = CreateWindowExW(
        0,
        class_name.as_ptr(),
        title.as_ptr() as LPCWSTR,
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        ptr::null_mut(),
        ptr::null_mut(),
        libloaderapi::GetModuleHandleW(ptr::null()),
        ptr::null_mut(),
    );

    if handle.is_null() {
        MessageBoxW(
            ptr::null_mut(),
            OsStr::new("Call to CreateWindow failed!")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_ptr(),
            OsStr::new("Windows Desktop Guided Tour")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>()
                .as_ptr(),
            0,
        );
        return;
    }

    ShowWindow(handle, SW_SHOW);
    UpdateWindow(handle);

    // Main message loop:
    let msg: LPMSG = ptr::null_mut();
    while GetMessageW(msg, handle, 0, 0) != 0 {
        TranslateMessage(msg);
        DispatchMessageW(msg);
    }

    dbg!((*msg).wParam);
}

fn main() {
    #[cfg(windows)]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    #[cfg(windows)]
    unsafe {
        windows_window()
    };

    println!("hello world");

    #[cfg(windows)]
    unsafe {
        winapi::um::wincon::FreeConsole();
    }
}
