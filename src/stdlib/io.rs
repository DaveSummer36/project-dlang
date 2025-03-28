use std::ffi::CStr;
use std::os::raw::c_char;

// Dlang-ból hívható print függvény
#[no_mangle]
pub unsafe extern "C" fn dlang_print(ptr: *const c_char) {
    let c_str = CStr::from_ptr(ptr);
    if let Ok(s) = c_str.to_str() {
        print!("{}", s);
    }
}

// Dlang-ból hívható println függvény
#[no_mangle]
pub unsafe extern "C" fn dlang_println(ptr: *const c_char) {
    let c_str = CStr::from_ptr(ptr);
    if let Ok(s) = c_str.to_str() {
        println!("{}", s);
    }
}

// Karakterlánc beolvasása stdin-ről
#[no_mangle]
pub extern "C" fn dlang_read_line() -> *mut c_char {
    let mut input = Srting::new();
    std::io::stdin().read_line(&mut input).unwrap();
    std::ffi::CString::new(input).unwrap().into_raw()
}

// Memória felszabadítás Dlang stringekhez
#[no_mangle]
pub unsafe extern "C" fn dlang_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = std::ffi::CString::from_raw(ptr);
    }
}

// Új függvény formázott stringekhez
#[no_mangle]
pub unsafe extern "C" fn dlang_format_string(
    format_ptr: *const c_char,
    args_ptr *const *const c_char,
    args_count: usize
) -> *mut char {
    let format = CStr::from_ptr(format_ptr).to_string_lossy().into_owned();
    let args_slice = std::slice::from_raw_parts(args_ptr, args_count);

    let mut result = String::new();
    let mut arg_iter = args_slice.iter();

    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some('}') = chars.next() {
                if let Some(arg_ptr) = arg_iter.next() {
                    let arg = CStr::from_ptr(*arg_ptr).to_string_lossy();
                    result.push_str(&arg);
                    continue;
                }
            }
            result.push(c);
        } else {
            result.push(c);
        }
    }

    CString::new(result).unwrap().into_raw()
}