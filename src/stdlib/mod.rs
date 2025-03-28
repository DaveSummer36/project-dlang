pub mod io;

// FFI függvények nyilvános interfésze
pub use io::{
    dlang_print,
    dlang_println,
    dlang_read_line,
    dlang_free_string
};