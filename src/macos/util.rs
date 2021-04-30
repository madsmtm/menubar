use core::ptr;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi;
use std::os::raw;

pub type Id = *mut Object;

#[allow(non_upper_case_globals)]
pub const nil: Id = ptr::null_mut();

// https://developer.apple.com/documentation/objectivec/nsinteger?language=objc

#[cfg(target_pointer_width = "32")]
pub type NSInteger = raw::c_int;
#[cfg(target_pointer_width = "64")]
pub type NSInteger = raw::c_long;

// https://developer.apple.com/documentation/objectivec/nsuinteger?language=objc

#[cfg(target_pointer_width = "32")]
pub type NSUInteger = raw::c_uint;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = raw::c_ulong;

// https://developer.apple.com/documentation/foundation/1497293-string_encodings/nsutf8stringencoding?language=objc
const NS_UTF8_STRING_ENCODING: NSUInteger = 4;

pub fn to_nsstring(string: &str) -> Id {
    unsafe {
        let id: Id = msg_send![class!(NSString), alloc];
        msg_send![
            id,
            initWithBytes: string.as_ptr() as *const ffi::c_void
            length: string.len() as NSUInteger
            encoding: NS_UTF8_STRING_ENCODING
        ]
    }
}

pub unsafe fn from_nsstring<'a>(nsstring: Id) -> &'a str {
    let res = msg_send![nsstring, UTF8String];
    let cstring = ffi::CStr::from_ptr(res);
    cstring.to_str().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::STRINGS;

    #[test]
    fn test_nsstring() {
        STRINGS.iter().for_each(|&s| {
            let id = to_nsstring(s);
            let s2 = unsafe { from_nsstring(id) };
            assert_eq!(s, s2);
        })
    }
}
