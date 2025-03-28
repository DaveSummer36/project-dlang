use std::ffi::{CString, CStr};
use std::os::raw::c_char;

// Dlang string konvertálása Rust stringgé
pub unsafe fn dlang_to_rust(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

// Rust string konvertálása Dlang stringgé
pub fn rust_to_dlang(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}