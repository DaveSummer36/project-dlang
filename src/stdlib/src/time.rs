use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, NaiveDateTime, TimeZone, Local};
use chrono_tz::Tz;

// --- Globális időzóna (thread-local storage) ---
thread_local! {
    static GLOBAL_TZ: Mutex<Option<Tz>> = Mutex::new(None);
}

/// Időreprezentáció (ms pontosságú, mint JavaScript-ben)
#[repl(C)]
pub struct DlangTime {
    timestamp: i64 // Unix epoch milliszekundumban
}

/// Magas felbontású időmérő (HRT)
pub struct DlangPerformanceTimer {
    start_time: Duration
}

// --- Alapvető időmérés ---

/// Visszaadja az aktuális időt Unix epoch óta eltelt milliszekundumban (mint Date.now())
#[no_mangle]
pub extern "C" fn dlang_time_now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}

/// Vár a megadott milliszekundumig (mint setTimeout(), de blokkoló)
#[no_mangle]
pub extern "C" fn dlang_time_sleep_ms(ms: i64) {
    thread::sleep(Duration::from_millis(ms as u64));
}

/// Létrehoz egy új DlangTime objektumot (mint new Date())
#[no_mangle]
pub extern "C" fn dlang_time_create() -> *mut DlangTime {
    Box::into_raw(Box::new(DlangTime {
        timestamp: dlang_time_now()
    }))
}

/// Szöveges reprezentáció (min Date.toString())
#[no_mangle]
pub unsafe extern "C" fn dlang_time_to_string(time: *const DlangTime) -> *mut c_char {
    let time = &*time;
    let dt = chrono::NaiveDateTime::from_timestamp_millis(time.timestamp).unwrap();
    CString::new(dt.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap().into_raw()
}

// --- Magas felbontású időmérés (performance API) ---

/// Elindít egy új performance timer-t (mint performance.now())
#[no_mangle]
pub extern "C" fn dlang_performance_now() -> f64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    now.as_secs_f64() * 1000.0 // Másodperc -> ms
}

/// Létrehoz egy új PerformanceTimer objektumot
#[no_mangle]
pub extern "C" fn dlang_performance_timer_start() -> *mut DlangPerformanceTimer {
    Box::into_raw(Box::new(DlangPerformanceTimer {
        start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }))
}

/// Megállítja a timert és visszaadja az eltelt időt ms-ban
#[no_mangle]
pub unsafe extern "C" fn dlang_performance_timer_stop(timer: *mut DlangPerformanceTimer) -> u64 {
    let timer = Box::from_raw(timer);
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = end_time - timer.start_time;
    duration.as_secs_f64() * 1000.0
}

// --- Időformázás ---

/// Formázott idő stringgé (mint Date.toLocaleString())
#[no_mangle]
pub unsafe extern "C" fn dlang_time_format(
    time: *const DlangTime,
    format: *const c_char
) -> *mut c_char {
    let time = &*time;
    let fmt = CStr::from_ptr(format).to_str().unwrap();
    let dt = chrono::NaiveDateTime::from_timestamp_millis(time.timestamp).unwrap();

    let formatted = match fmt {
        "ISO" => dt.to_string(),
        _ => dt.format(fmt).to_string()
    };

    CString::new(formatted).unwrap().into_raw()
}

// --- Időzítők ---

/// Időzítő callbackkel (mint setTimeout(), nem blokkoló)
#[no_mangle]
pub unsafe extern "C" fn dlang_time_set_timeout(
    callback: extern "C" fn(*mut c_char),
    ms: i64,
    data: *mut c_char
) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(ms as u64));
        callback(data);
    });
}

// --- Időzóna kezelés ---
#[no_mangle]
pub unsafe extern "C" fn dlang_time_set_timezone(tz_name: *const char) -> bool {
    let tz_str = match CStr::from_str(tz_name).to_str() {
        Ok(s) => s,
        Err(_) => return false
    };

    match tz_str.parse::<Tz>() {
        Ok(tz) => {
            GLOBAL_TZ.with(|g| *g.lock().unwrap() = Some(tz));
            true
        }
        Err(_) => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn dlang_time_local_now(format: *const c_char) -> *mut c_char {
    let fmt = CStr::from_ptr(format).to_str().unwrap();
    let now = GLOBAL_TZ.with(|g| {
        if let Some(tz) = &*g.lock().unwrap() {
            tz.from_utc_datetime(
                &NaiveDateTime::from_timestamp_millis(dlang_time_now()).unwrap()
            ).format(fmt).to_string()
        } else {
            Local::now().format(fmt).to_string()
        }
    });
    CString::new(now).unwrap().into_raw()
}

// --- Periodikus időzítők ---
#[no_mangle]
pub unsafe extern "C" fn dlang_time_set_interval(
    callback: extern "C" fn(*mut c_char),
    ms: u64,
    data: *mut c_char
) -> *mut c_void {
    let callback_ptr = Arc::new(Mutex::new(callback));
    let data_ptr = Arc::new(Mutex::new(data));

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(ms));
        let cb = callback_ptr.lock().unwrap();
        let data = data_ptr.lock().unwrap();
        cb(*data);
    });

    Box::into_raw(Box::new((callback_ptr, data_ptr))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn dlang_time_clear_interval(handle: *mut c_void) {
    let _ = Box::from_raw(handle as *mut (Arc<Mutex<...>>, Arc<Mutex<...>>));
}

#[no_mangle]
