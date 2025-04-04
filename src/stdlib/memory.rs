#[cfg(unix)]
mod unix {
    use libc::{mmap, munmap, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE};

    #[no_mangle]
    pub unsafe extern "C" fn sys_allocate(size: usize) -> *mut u8 {
        mmap(std::ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
        if ptr == MAP_FAILED {
            std::ptr::null_mut()
        } else {
            ptr as *mut u8
        }
    }
}

#[cfg(windows)]
mod windows {
    use winapi::um::memoryapi::VirtualAlloc;
    use::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

    #[no_mangle]
    pub unsafe extern "C" fn sys_allocate(size: usize) -> *mut u8 {
        VirtualAlloc(std::ptr::null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) as *mut u8
    }
}