use core::ptr;
use objc::rc::{AutoreleasePool, Owned};
use objc::runtime::Object;
use objc::{class, msg_send, sel};
use std::ffi;
use std::os::raw;

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

#[repr(C)]
pub struct NSString {
    _priv: [u8; 0],
}

unsafe impl<'a> objc::Encode for &'a NSString {
    const ENCODING: objc::Encoding<'static> = objc::Encoding::Object;
}

unsafe impl<'a> objc::Encode for &'a mut NSString {
    const ENCODING: objc::Encoding<'static> = objc::Encoding::Object;
}

unsafe impl objc::Message for NSString {}

unsafe impl Send for NSString {}
unsafe impl Sync for NSString {}

impl NSString {
    pub fn from_str(s: &str) -> Owned<Self> {
        unsafe {
            let id: *mut Self = msg_send![class!(NSString), alloc];
            Owned::new(msg_send![
                id,
                initWithBytes: s.as_ptr() as *const ffi::c_void
                length: s.len() as NSUInteger
                encoding: NS_UTF8_STRING_ENCODING
            ])
        }
    }

    pub fn to_str<'p>(&self, pool: &'p AutoreleasePool) -> &'p str {
        unsafe {
            let res = msg_send![self, UTF8String];
            let cstring = ffi::CStr::from_ptr(res);
            cstring.to_str().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NSString;
    use crate::test_util::STRINGS;
    use objc::rc::autoreleasepool;

    #[test]
    fn test_nsstring() {
        autoreleasepool(|pool| {
            STRINGS.iter().for_each(|&s| {
                let nsstring = NSString::from_str(s);
                let s2 = nsstring.to_str(pool);
                assert_eq!(s, s2);
            });
        });
    }
}
