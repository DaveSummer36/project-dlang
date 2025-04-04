use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::{self, Write};

/// Kiír egy karakterláncot a standard kimenetre (nincs sortörés)
/// # Safety
/// A 'message' null-terminált C stringre mutat.
#[no_mangle]
pub unsafe extern "C" fn dlang_print(message: *const c_char) {
    let msg = CStr::from_ptr(message).to_string_lossy();
    print!("{}", msg);
    io::stdout().flush().unwrap(); // Azonnali kiírás
}

/// Kiír egy karakterláncot a standard kimenetre sortöréssel.
/// # Safety
/// A 'message' null-terminált C stringre mutat.
#[no_mangle]
pub unsafe extern "C" fn dlang_println(message: *const c_char) {
    let msg = CStr::from_ptr(message).to_string_lossy();
    println!("{}", msg);
}

/// Formázott kiírás ('printf' stílusú, egyszerűsítve).
/// Példa: 'dlang_printf("{} + {} = {}\0", 2, 3, 5)'
/// # Safety
/// A 'format' null-terminált C stringre mutat, és a változók száma egyezik a helyőrzőkkel.
#[no_mangle]
pub unsafe extern "C" fn dlang_printf(format: *const c_char) -> *mut c_char {
    use std::fmt::Write;
    let mut output = String::new();
    let fmt_str = CStr::from_ptr(format).to_string_lossy();

    // Egyszerűsített helyőrző-feldolgozás (csak '{}')
    let parts: Vec<&str> = fmt_str.split("{}").collect();
    let mut args = std::ffi::VaList::new();

    for (i, part) in parts.iter().enumerate() {
        write!(output, "{}", part).unwrap();
        if i < parts.len() - 1 {
            match args.arg::<i32>() {
                // Egyszerűsített típuskezelés (csak i32)
                n => write!(output, "{}", n).unwrap()
            }
        }
    }

    CString::new(output).unwrap().into_raw();
}

/// Beolvas egy sort a standard bemenetről.
/// # Return
/// A beolvasott string (null-terminált), vagy 'null' hiba esetén.
#[no_mangle]
pub extern "C" fn dlang_readline() -> *mut c_char {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return std::ptr::null_mut();
    }
    CString::new(input.trim()).unwrap().into_raw()
}

/// Felszabadít egy dinamikusan lefoglalt stringet
/// # Safety
/// A 'ptr' érvényes, előzőleg lefoglalt CString-re mutat.
#[no_mangle]
pub unsafe extern "C" fn dlang_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

/// Fájl tartalmának beolvasása
/// # Safety
/// A 'path' null-terminált C stringre mutat.
/// # Return
/// A fájl tartalma (null-terminált), vagy 'null' hiba esetén.
#[no_mangle]
pub unsafe extern "C" fn dlang_readfile(path: *const c_char) -> *mut c_char {
    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut()
    };

    match std::fs::read_to_string(path_str) {
        Ok(content) => CString::new(content).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut()
    }
}

/// Fájlba írás
/// # Safety
/// A 'path' és 'content' null-terminált C stringekre mutatnak.
#[no_mangle]
pub unsafe extern "C" fn dlang_writefile(path: *const c_char, content: *const c_char) -> bool {
    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => return false
    };

    let content_str = match CStr::from_ptr(content).to_str() {
        Ok(s) => s,
        Err(_) => return false
    };

    std::fs::write(path_str, content_str).is_ok()
}