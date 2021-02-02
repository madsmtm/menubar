use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CStr;

pub fn to_nsstring(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string) }
}

pub unsafe fn from_nsstring<'a>(nsstring: id) -> &'a str {
    let res = msg_send![nsstring, UTF8String];
    let cstring = CStr::from_ptr(res);
    cstring.to_str().unwrap()
}
