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
