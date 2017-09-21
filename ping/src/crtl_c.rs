#[cfg(unix)]
mod inner {
    use std::mem;
    use std::ptr;

    extern crate libc;
    use self::libc::{
        c_int,
        sighandler_t,
        SIGINT,
        SIGTERM,
        sigaction,
        SA_RESETHAND,
        raise
    };

    unsafe extern "C" fn handler(signum: c_int) {
        let stats = ::STATS.read().unwrap();
        println!("{}", *stats);
        raise(signum);
    }

    #[inline]
    pub fn set_handler() {
        unsafe {
            let mut act: sigaction = mem::zeroed();
            act.sa_sigaction = handler as sighandler_t;
            act.sa_flags = SA_RESETHAND;

            sigaction(SIGINT, &act, ptr::null_mut());
            sigaction(SIGTERM, &act, ptr::null_mut());
        }
    }
}

#[cfg(windows)]
mod inner {
    use std::os::raw::*;

    type DWORD = c_ulong;
    type BOOL = c_int;
    #[allow(bad_style)]
    type PHANDLER_ROUTINE = Option<unsafe  extern "system" fn(crtl_type: DWORD) -> BOOL>;

    extern "system" {
        pub fn SetConsoleCtrlHandler(HandlerRoutine: PHANDLER_ROUTINE, Add: BOOL) -> BOOL;
    }

    unsafe extern "system" fn handler(_: DWORD) -> BOOL {
        let stats = ::STATS.read().unwrap();
        println!("Crtl-C\n{}", *stats);
        0
    }

    #[inline]
    pub fn set_handler() {
        unsafe {
            SetConsoleCtrlHandler(Some(handler), 1);
        }
    }
}


pub use self::inner::*;
