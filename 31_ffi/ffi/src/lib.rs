use libc::c_char;
use std:: {
    ffi::{CStr, CString},
    panic::catch_unwind,
    ptr,
};

// 使用no_mangle 进制函数名改编
#[no_mangle]
pub extern "C" fn hello_world() -> *const c_char {
    "hello world!\\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn hello_bad(name: *const c_char) -> *const c_char {
    let s = unsafe {CStr::from_ptr(name).to_str().unwrap()};
    format!("hello {}!\\0", s).as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn hello(name: *const c_char) -> *const c_char {
    if name.is_null() {
        return ptr::null();
    }

    let result = catch_unwind(|| {
        if let Ok(s) = unsafe {CStr::from_ptr(name).to_str()} {
            let result = format!("hello {}!", s);
            let s = CString::new(result).unwrap();
            s.into_raw()
        } else {
            ptr::null()
        }
    });

    match result {
        Ok(s) => s,
        Err(_) => ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn free_str(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s)};
    }
}