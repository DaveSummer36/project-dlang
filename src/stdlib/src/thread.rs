use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

// --- Szálkezelés ---

/// Új szál indítása (mint `thread::spawn`)
#[no_mangle]
pub unsafe extern "C" fn dlang_thread_spawn(
    callback: extern "C" fn(*mut c_void),
    data: *mut c_void
) -> *mut c_void {
    let data_ptr = Arc::new(Mutex::new(data));
    let callback_ptr = Arc::new(Mutex::new(callback));
    
    let handle = thread::spawn(move || {
        let cb = callback_ptr.lock().unwrap();
        let data = data_ptr.lock().unwrap();
        cb(*data)
    });
    
    Box::into_raw(Box::new(handle)) as *mut c_void
}

/// Szál leállítása (nem blokkoló)
#[no_mangle]
pub unsafe extern "C" fn dlang_thread_join(handle: *mut c_void) {
    let handle = Box::from_raw(handle as *mut thread::JoinHandle<()>);
    let _ = handle.join();
}

// --- Mutex (Kölcsönös kizárás) ---

#[repr(C)]
pub struct DlangMutex {
    inner: Arc<Mutex<()>>
}

/// Új mutex hozzáadása
#[no_mangle]
pub extern "C" fn dlang_mutex_new() -> *mut DlangMutex {
    Box::into_raw(Box::new(DlangMutex {
        inner: Arc::new(Mutex::new(()))
    }))
}

/// Mutex lockolása (blokkoló)
#[no_mangle]
pub unsafe extern "C" fn dlang_mutex_lock(mutex: *mut DlangMutex) {
    let mutex = &*mutex;
    let _guard = mutex.inner.lock().unwrap();
}

/// Mutex feloldása
#[no_mangle]
pub unsafe extern "C" fn dlang_mutex_unlock(mutex: *mut DlangMutex) {
    // A _guard automatikusan felszabadul itt
}

// --- Cstornák (MPSC: Multiple Producer, Single Cosumer) ---

#[repr(C)]
pub struct DlangChannel {
    sender: mpsc::Sender<*mut c_void>,
    receiver: mpsc::Receiver<*mut c_void>
}

/// Új csatorna létrehozása
#[no_mangle]
pub extern "C" fn dlang_channel_new() -> *mut DlangChannel {
    let (sender, receiver) -> mpsc::channel();
    Box::into_raw(Box::new(DlangChannel { sender, receiver }))
}

/// Üzenet küldése a csatornára
#[no_mangle]
pub unsafe extern "C" fn dlang_channel_send(
    channel: *mut DlangChannel,
    message: *mut c_void
) -> bool {
    let channel = &*channel;
    channel.sender.send(message).is_ok()
}

/// Üzenet fogadása a csatornáról (blokkoló)
#[no_mangle]
pub unsafe extern "C" fn dlang_channel_recv(
    channel: *mut DlangChannel
) -> *mut c_void {
    let channel = &*channel;
    channel.receiver.recv().unwrap()
}

// ---Atomic műveletek (egyszerű példa) ---

use std::sync::atomic::{AtomicI32, Ordering};

#[repr(C)]
pub struct DlangAtomicI32 {
    inner: AtomicI32
}

#[no_mangle]
pub extern "C" fn dlang_atomic_i32_new(value: i32) -> *mut DlangAtomicI32 {
    Box::into_raw(Box::new(DlangAtomicI32 {
        inner: AtomicI32::new(value)
    }))
}

#[no_mangle]
pub unsafe extern "C" fn dlang_atomic_i32_load(atomic: *mut DlangAtomicI32) -> i32 {
    let atomic = &*atomic;
    atomic.inner.load(Ordering::SeqCst)
}

#[no_mangle]
pub unsafe extern "C" fn dlang_atomic_i32_store(atomic: *mut DlangAtomicI32, value: i32) {
    let atomic = &*atomic;
    atomic.inner.store(value, Ordering::SeqCst);
}