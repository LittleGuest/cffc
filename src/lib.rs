use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int},
};

use crate::convert::Data;

pub mod convert;

/// # Safety
///
/// 转换
#[no_mangle]
pub unsafe extern "C" fn convert(
    from: *const c_char,
    to: *const c_char,
    text: *const c_char,
) -> *mut c_char {
    let data =
        Data {
            from: Option::from(CStr::from_ptr(from).to_str().unwrap().to_string()),
            to: Option::from(CStr::from_ptr(to).to_str().unwrap().to_string()),
            text: Option::from(CStr::from_ptr(text).to_str().unwrap().to_string()),
        };

    CString::new(data.convert()).unwrap().into_raw()
}

/// # Safety
///
/// 校验
#[no_mangle]
pub unsafe extern "C" fn check(from: *const c_char, text: *const c_char) -> c_int {
    let data = Data {
        from: Option::from(CStr::from_ptr(from).to_str().unwrap().to_string()),
        to: None,
        text: Option::from(CStr::from_ptr(text).to_str().unwrap().to_string()),
    };
    if data.check() {
        1_i32
    } else {
        0
    }
}
