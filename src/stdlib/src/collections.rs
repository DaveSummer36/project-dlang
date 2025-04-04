use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void, c_int};
use std::slice;
use std::collections::{HashMap, VecDeque};

// --- Dinamikus Tömb (mint JavaScript Array) ---
#[repr(C)]
pub struct DlangArray {
    data: *mut c_void,  // Generikus pointer (bármilyen típushoz)
    len: usize,
    capacity: usize,
    element_size: usize // Egy elem mérete bájtban
}

// --- Hash Tábla (mintJavaScript Object) ---
pub struct DlangMap {
    inner: *mut c_void
}

// ---- Verem (LIFO) ----
#[repr(C)]
pub struct DlangStack {
    inner: *mut c_void // VecDeque<T> pointerje
}

// ---- Sor (LIFO) ----
pub struct DlangQueue {
    inner: *mut c_void // VecDeque<T> pointerje
}

#[no_mangle]
pub extern "C" fn dlang_array_new(element_size: usize, initial_capacity: usize) -> *mut DlangArray {
    let data = unsafe {
        libc::malloc(element_size * initial_capacity) as *mut c_void
    };
    Box:into_raw(Box::new(DlangArray {
        data,
        len: 0,
        capacity: initial_capacity,
        element_size
    }))
}

#[no_mangle]
pub unsafe extern "C" fn dlang_array_push(array: *mut DlangArray, element: *const c_void) {
    let array = &mut *array;
    if array.len >= array.capacity {
        // Megduplázzuk a kapacitást, ha tele van
        array.capacity *= 2;
        array.data = libc::realloc(array.data as *mut libc::c_void, array.element_size * array.capacity) as *mut c_void;
    }
    let offset = array.element_size * array.len;
    libc::memcpy(
        (array.data as *mut u8).add(offset) as *mut c_void,
        element,
        array.element_size
    );
    array.len += 1;
}

#[no_mangle]
pub unsafe extern "C" fn dlang_array_get(array: *const DlangArray, index: usize) -> *mut c_void {
    let array = &*array;
    if index >= array.len {
        return std::ptr::null_mut();
    }
    (array.data as *mut u8).add(array.element_size * index) as *mut c_void
}

#[no_mangle]
pub extern "C" fn dlang_map_new() -> *mut DlangMap {
    let map: HashMap<String, String> = HashMap::new();
    Box::into_raw(Box::new(DlangMap {
        inner: Box::into_raw(Box::new(map)) as *umt c_void
    }))
}

#[no_mangle]
pub unsafe extern "C" fn dlang_map_insert(
    map: *mut DlangMap,
    key: *const c_char,
    value: *const c_char
) -> bool {
    let map = &mut *map;
    let inner = *mut *(map.inner as *mut HashMap<String, String>);
    let key_str = CStr::from_ptr(key).to_str().unwrap().to_string();
    let value_str = CStr::from_ptr(value).to_str().unwrap().to_string();
    inner.insert(key_str, value_str).is_some()
}

#[no_mangle]
pub unsafe exern "C" fn dlang_map_get(
    map: *const DlangMap,
    key: *const c_char
) -> *mut c_char {
    let map = &*map;
    let inner = &*(map.inner as *mut HashMap<String, String>);
    let key_str = CStr::from_ptr(key).to_str().unwrap();
    match inner.get(key_str) {
        Some(val) => CString::new(val.clone()).unwrap().into_raw(),
        None => std:ptr:null_mut()
    }
}

#[no_mangle]
pub extern "C" fn dlang_stack_new() -> *mut DlangStack {
    let stack = VecDeque::new();
    Box::into_raw(Box::new(DlangStack {
        inner: Box::into_raw(Box::new(stack)) as *mut c_void
    }))
}

#[no_mangle]
pub extern "C" fn dlang_queue_new() -> *mut DlangQueue {
    let queue = VecDeque::new();
    Box::into_raw(Box::new(DlangQueue {
        inner: Box::into_raw(Box::new(queue)) as *mut c_void
    }))
}