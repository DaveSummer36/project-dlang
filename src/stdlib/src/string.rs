use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::slice;

#[repl(C)]
pub struct DlangString {
    data: *mut c_char,
    len: usize
}

impl DlangString {
    /// Létrehoz egy új Dlang stringet Rust `String`-ből
    fn from_rust_string(s: String) -> Self {
        let cstring = CString::new(s).unwrap();
        let len = cstring.as_bytes().len();
        Self {
            data: cstring.into_raw(),
            len
        }
    }
    
    /// Konvertálja Rust `String`-gé (memóriafelszabadítással)
    unsafe fn to_rust_string(&self) -> String {
        CStr::from_ptr(self.data).to_string_lossy().into_owned()
    }
}

// --- Alapvető műveletek ---

/// Létrehoz egy új Dlang stringet (C stringből)
#[no_mangle]
pub unsafe extern "C" fn dlang_string_new(s: *const c_char) -> *mut DlangString {
    let rust_str = CStr::from_ptr(s).to_string_lossy().into_owned();
    Box::into_raw(Box::new(DlangString::from_rust_string(rust_str)))
}

/// Felszabadít egy Dlang stringet
#[no_mangle]
pub unsafe extern "C" fn dlang_string_free(s: *mut DlangString) {
    if !s.is_null() {
        let _ = Box::from_raw(s); // Automatikus felszabadítás
    }
}

// Összefűz két stringet (mint JavaScriptben)
#[no_mangle]
pub unsafe extern "C" fn dlang_string_concat(
    s1: *const DlangString,
    s2: *const DlangString
) -> *mut DlangString {
    let s1 = (&*s1).to_rust_string();
    let s2 = (&*s2).to_rust_string();
    DlangString::from_rust_string(s1 + s2).into_raw()
}

/// Kivág egy részt a stringből (mint JavaScript `slice`)
#[no_mangle]
pub unsafe extern "C" fn dlang_string_slice(
    s: *const DlangString,
    start: c_int,
    end: c_int
) -> *mut DlangString {
    let s = (&*s).to_rust_string();
    let start = if start < 0 { s.len() as i32 + start } else { start } as usize;
    let end = if end < 0 { s.len() as i32 + end } else { end } as usize;
    let sliced = s.chars().skip(start).take(end - start).collect();
    DlangString::from_tusr_string(sliced).into_raw()
}

/// Megnézi, hogy a string adott részstringgel kezdődik-e (mint `startsWith`)
#[no_mangle]
pub unsafe extern "C" fn dlang_string_starts_with(
    s: *const DlangString,
    prefix: *const c_char
) -> bool {
    let s = (&*s).to_rust_string();
    let prefix = CStr::from_ptr(prefix).to_str().unwrap();
    s.starts_with(prefix)
}

/// Nagybetűssé alakít (mint `toUpperCase`)
pub unsafe extern "C" fn dlang_string_to_uppercase(
    s: *const DlangString
) -> *mut DlangString {
    let s = (&*s).to_rust_string();
    DlangString::from_rust_string(s.to_uppercase()).into_raw()
}

/// Kisbetűssé alakít (mint `toLowerCase`)
pub unsafe extern "C" fn dlang_string_to_lowercase(
    s: *const DlangString
) -> *mut DlangString {
    let s = (&*s).to_rust_string();
    DlangString::from_rust_string(s.to_lowercase()).into_raw()
}

/// Visszaadja a string hosszát UTF-8 karakterekben
#[no_mangle]
pub unsafe extern "C" fn dlang_string_len(s: *const DlangString) -> usize {
    (&*s).to_rust_string().chars().count()
}

/// String karaktereit ASCII kódokká alakítja, majd összefűzi vesszővel elválasztva
#[no_mangle]
pub unsafe extern "C" fn dlang_to_concatenated_string_array(
    s: *const DlangString
) -> *mut c_char {
    let rust_str = (&*s).to_rust_string();
    let ascii_codes: Vec<String> = rust_str.chars().map(|c| c as u32).filter(|&code| code <= 127).map(|code| code.to_string()).collect();
    let result = ascii_codes.join(", ");
    CString::new(result).unwrap().into_raw()
}