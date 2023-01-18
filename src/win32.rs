//! Win32 implementation of menubars.

use crate::Error;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroIsize;
use std::rc::Rc;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

use windows_sys::Win32::UI::Shell::{DefSubclassProc, SetWindowSubclass};

use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateMenu, CreatePopupMenu, DestroyMenu, GetMenu, InsertMenuItemA, SetMenu, SetMenuInfo, MIIM_ID,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{HMENU, MENUINFO, MENUITEMINFOA};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MFT_SEPARATOR, MFT_STRING, MIIM_DATA, MIIM_FTYPE, MIIM_STRING, MIIM_SUBMENU, MIIM_TYPE,
    MIM_STYLE, MNS_NOTIFYBYPOS, WM_MENUCOMMAND, WM_NCDESTROY,
};

#[derive(Debug)]
pub struct NotProperWindow;

impl fmt::Display for NotProperWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the given window is not a proper window")
    }
}

impl std::error::Error for NotProperWindow {}

macro_rules! syscall {
    // The null value (0) is an error.
    (nul $fname: ident $($args: tt)*) => {{
        let res = unsafe { $fname $($args)* };
        if res == 0 {
            return Err(crate::Error::last_io_error());
        } else {
            res
        }
    }}
}

// No one else should use this very unique ID.
const SUBCLASS_ID: usize = 4 * 8 * 15 * 16 * 23 * 42;

unsafe extern "system" fn menu_subclass_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    uidsubclass: usize,
    refdata: usize,
) -> LRESULT {
    abort_on_panic(move || {
        // Early out.
        macro_rules! early_out {
            () => {{
                return DefSubclassProc(hwnd, msg, wparam, lparam);
            }};
        }

        macro_rules! leap {
            ($e: expr) => {{
                match $e {
                    Some(e) => e,
                    None => early_out!(),
                }
            }};
        }

        // If we are being destroyed, free our refdata.
        if msg == WM_NCDESTROY {
            drop(Box::from_raw(refdata as *mut DataMapCell));
            early_out!();
        } else if msg == WM_MENUCOMMAND {
            // Get a reference to the hash map containing our menu item data.
            let map_cell = &*(refdata as *const DataMapCell);

            // Shouldn't be called reentrantly.
            let mut map = map_cell.borrow_mut();

            // Get the item key.
            let hmenu = leap!(NonZeroIsize::new(lparam as _));
            let key = ItemKey(hmenu, wparam);

            // Get the item data.
            let data = leap!(map.get_mut(&key));

            // Call the handler.
            (data.handler)();
        }

        early_out!();
    })
}

#[doc(hidden)]
pub enum Empty {}

pub struct Hotkey<'a> {
    // TODO
    key: &'a str,
}

type DataMap = HashMap<ItemKey, MenuItemData>;
type DataMapCell = RefCell<DataMap>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct ItemKey(NonZeroIsize, usize);

/// A menu to be attached to a window.
pub struct Menu {
    /// Handle to the menu.
    ///
    /// This is semantically an `HMENU`, but we use `NonZeroIsize` to avoid
    /// allocating an extra pointer here.
    ///
    /// This is also used to uniquely identify the menu.
    menu: Option<NonZeroIsize>,

    /// Data associated with the menu.
    data: DataMap,

    /// Total number of top-level items in the menu.
    len: usize,

    /// Menus are not thread-safe.
    _marker: PhantomData<*mut ()>,
}

impl Menu {
    /// Create a new menu.
    pub fn new() -> Result<Self, Error> {
        // Create the menu.
        let menu = syscall!(nul CreateMenu());

        // Set up the menu so WM_MENUCOMMAND gets used.
        let info = MENUINFO {
            cbSize: mem::size_of::<MENUINFO>() as _,
            fMask: MIM_STYLE,
            dwStyle: MNS_NOTIFYBYPOS,
            ..unsafe { mem::zeroed() }
        };
        syscall!(nul SetMenuInfo(menu, &info));

        unsafe { Ok(Menu::from_hmenu(menu)) }
    }

    /// Create a new, empty popup menu.
    pub fn new_popup() -> Result<Self, Error> {
        // Create the menu.
        let menu = syscall!(nul CreatePopupMenu());

        unsafe { Ok(Menu::from_hmenu(menu)) }
    }

    unsafe fn from_hmenu(menu: HMENU) -> Self {
        Menu {
            menu: Some(NonZeroIsize::new_unchecked(menu)),
            data: HashMap::new(),
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Add a new menu item to the menu.
    pub fn push<'t, 'h, H: MenuItemHandler>(
        &mut self,
        item: impl Into<MenuItem<'t, 'h, H>>,
    ) -> Result<(), Error> {
        // Create the menu item.
        let item = item.into();
        match item.inner {
            Inner::Separator => {
                // Menu item is a separator.
                let info = MENUITEMINFOA {
                    cbSize: mem::size_of::<MENUITEMINFOA>() as _,
                    fMask: MIIM_FTYPE,
                    fType: MFT_SEPARATOR,
                    ..unsafe { mem::zeroed() }
                };

                // Insert the menu item.
                syscall!(nul InsertMenuItemA(self.menu.unwrap().get(), self.len as _, true as _, &info));
            }

            Inner::Submenu { text, mut submenu } => {
                // Menu item is a submenu.
                let handle = submenu.menu.take().unwrap();
                let items = mem::take(&mut submenu.data);

                // Append items to our items.
                self.data.extend(items.into_iter());

                let text = CString::new(text).unwrap();

                let info = MENUITEMINFOA {
                    cbSize: mem::size_of::<MENUITEMINFOA>() as _,
                    fMask: MIIM_FTYPE | MIIM_SUBMENU | MIIM_TYPE,
                    fType: MFT_STRING,
                    hSubMenu: handle.get(),
                    dwTypeData: text.as_ptr() as _,
                    ..unsafe { mem::zeroed() }
                };

                // Insert the menu item.
                syscall!(nul InsertMenuItemA(self.menu.unwrap().get(), self.len as _, true as _, &info));
            }

            Inner::Item {
                text,
                hotkey,
                mut handler,
            } => {
                // Create a new item key.
                let key = ItemKey(self.menu.unwrap(), self.len);

                // Add this key to our map.
                self.data.insert(
                    key,
                    MenuItemData {
                        handler: Box::new(move || handler.invoke()),
                    },
                );

                let text = CString::new(text).unwrap();

                let info = MENUITEMINFOA {
                    cbSize: mem::size_of::<MENUITEMINFOA>() as _,
                    fMask: MIIM_FTYPE | MIIM_TYPE | MIIM_ID,
                    fType: MFT_STRING,
                    wID: 42,
                    dwTypeData: text.as_ptr() as _,
                    cch: text.as_bytes().len() as _,
                    ..unsafe { mem::zeroed() }
                };

                // Insert the menu item.
                syscall!(nul InsertMenuItemA(self.menu.unwrap().get(), self.len as _, true as _, &info));
            }
        };

        // Increment the length.
        self.len += 1;

        Ok(())
    }

    /// Apply this menu to a raw window handle.
    pub fn apply(
        self,
        handle: impl raw_window_handle::HasRawWindowHandle
    ) -> Result<(), Error> {
        match handle.raw_window_handle() {
            raw_window_handle::RawWindowHandle::Win32(handle) => unsafe {
                if handle.hwnd.is_null() {
                    return Err(Error::unexpected_window_type());
                }

                self.apply_to_hwnd(handle.hwnd as _)
            },
            _ => Err(Error::unexpected_window_type()),
        }
    }

    /// Apply this menu to a window.
    unsafe fn apply_to_hwnd(mut self, hwnd: HWND) -> Result<(), Error> {
        // If the window already has a menu, error out. We don't want to step on any toes.
        let old_menu = GetMenu(hwnd);
        if old_menu != 0 {
            return Err(Error::menu_exists());
        }

        // Set the menu.
        SetMenu(hwnd, self.menu.take().unwrap().get());

        // Add a subclass to the window.
        let data = mem::take(&mut self.data);
        let data = Box::into_raw(Box::new(RefCell::new(data)));
        SetWindowSubclass(hwnd, Some(menu_subclass_proc), SUBCLASS_ID, data as _);

        Ok(())
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        if let Some(menu) = self.menu {
            unsafe {
                DestroyMenu(menu.get());
            }
        }
    }
}

/// Data associated with each menu item.
struct MenuItemData {
    /// The handler for the menu item.
    handler: Box<dyn FnMut()>,
}

/// A menu item.
pub struct MenuItem<'txt, 'hotkey, Handler = Empty> {
    inner: Inner<'txt, 'hotkey, Handler>,
}

enum Inner<'txt, 'hotkey, Handler> {
    /// This is a regular menu item.
    Item {
        /// The text of the menu item.
        text: &'txt str,

        /// Handler for the menu item.
        hotkey: Option<Hotkey<'hotkey>>,

        /// Handler for the menu item.
        handler: Handler,
    },

    /// This is a separator.
    Separator,

    /// This is a submenu.
    Submenu {
        /// The text of the menu item.
        text: &'txt str,

        /// The handle to the submenu.
        submenu: Menu,
    },
}

impl MenuItem<'static, 'static> {
    /// Create a new separator.
    pub fn separator() -> Self {
        MenuItem {
            inner: Inner::Separator,
        }
    }
}

impl<'txt> MenuItem<'txt, 'static> {
    /// Create a drop-down menu item.
    pub fn submenu(text: &'txt str, submenu: Menu) -> Self {
        MenuItem {
            inner: Inner::Submenu { text, submenu },
        }
    }
}

impl<'txt, 'hotkey, Handler: MenuItemHandler> MenuItem<'txt, 'hotkey, Handler> {
    /// Create a new menu item.
    pub fn new(text: &'txt str, hotkey: Option<Hotkey<'hotkey>>, handler: Handler) -> Self {
        MenuItem {
            inner: Inner::Item {
                text,
                hotkey,
                handler,
            },
        }
    }
}

/// Callback for invoking a menu item's functionality.
///
/// This is implemented for all `F` where `F: FnMut()`.
pub trait MenuItemHandler: 'static {
    fn invoke(&mut self);
}

impl MenuItemHandler for Empty {
    fn invoke(&mut self) {
        match *self {}
    }
}

impl<F> MenuItemHandler for F
where
    F: FnMut() + 'static,
{
    fn invoke(&mut self) {
        self();
    }
}

fn abort_on_panic<R>(f: impl FnOnce() -> R) -> R {
    struct Bomb;

    impl Drop for Bomb {
        fn drop(&mut self) {
            std::process::abort();
        }
    }

    let bomb = Bomb;
    let r = f();
    mem::forget(bomb);
    r
}
