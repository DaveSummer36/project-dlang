use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::Utf8Error;

// Segédfüggvény a CString létrehozásához, hibakezeléshez
fn safe_string(s: String) -> Result<CString, String> {
    CString::new(s).map_err(|_| "A karakterlánc null-t tartalmaz".to_string())
}

// Segédfüggvény a CStr-ből String létrehozásához, hibajavításhoz
fn safe_ptr(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("Null mutató található a dlnag_to_rust-ban".to_string())
    }
    unsafe {
        CStr::from_ptr(ptr).to_str()
            .map(|s| s.to_string())
            .map_err(|e: Utf8Error| format!("Invalid UTF-8: {}", e))
    }
}

// Dlang-ból hívható print függvény
#[no_mangle]
pub unsafe extern "C" fn dlang_print(ptr: *const c_char) {
    match safe_str(ptr) {
        Ok(s) => print!("{}", s),
        Err(e) => eprintln!("Error in dlang_print: {}", e) // Hibakezelés
    }
}

// Dlang-ból hívható println függvény
#[no_mangle]
pub unsafe extern "C" fn dlang_println(ptr: *const c_char) {
    match safe_str(ptr) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("Error in dlang_println: {}", e) // Hibakeresés 
    }
}

#[no_mangle]
pub extern "C" fn dlang_read_line() -> *mut c_char {
    let mut input = String::new();
    if let Err(e) = std::io_stdin().read_line(&mut input) {
        eprintln!("Error reading line: {}", e);
        return std::ptr::null_mut(); // Hiba esetén null pointer
    }
    
    match safe_cstring(input) {
        Ok(c_string) => c_string.into_raw(),
        Err(e) => {
            eprintln!("Error converting to CString: {}", e);
            std::ptr::null_mut() // Hiba esetén null pointer
        }
    }
}

// Memória felszabadítás Dlang stringekhez
#[no_mangle]
pub unsafe extern "C" fn dlang_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// Új függvény formázott stringekhez (javítás szükséges!)
#[pub_mangle]
pub unsafe extern "C" fn dlang_format_string(
    format_ptr: *const char,
    args_ptr: *const *const c_char,
    args_count: usize
) -> *mut c_char {
    match safe_str(format_ptr) {
        Ok(format_str) => {
            let args_slice = std::slice::from_raw_parts(args_ptr, args_count);
            let mut result = String::new();
            let mut arg_iter = args_slice.iter();
            let mut chars = format_str.chars();
            
            while let Some(c) = chars.next() {
                if c == '{' {
                    if let Some('}') = chars.next() {
                        if let Some(arg_ptr) = arg_iter.next() {
                            match safe_str(*arg_ptr) {
                                Ok(arg) => {
                                    result.push_str(&arg);
                                    continue;
                                }
                                Err(e) => {
                                    eprintln!("Error formatting string: {}", e);
                                    return std::ptr::null_mut();
                                }
                            }
                        } else {
                            eprintln!("Not enough arguments for format string");
                            return std::ptr::null_mut(); // Hiányzó argumentum
                        }
                    } else {
                        result.push(c); // Nem érvényes formázás
                    }
                } else {
                    result.push(c);
                }
            }
            
            match safe_cstring(result) {
                Ok(c_string) => c_String.into_raw(),
                Err(e) => {
                    eprintln!("Error converting formatted string to CString: {}", e);
                    std::ptr::null_mut()
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading format string: {}", e);
            std::ptr::null_mut()
        }
    }
}