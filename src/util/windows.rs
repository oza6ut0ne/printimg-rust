use std::ffi::CString;
use libc::{dup2, fileno, fopen};

const STDERR_FILENO: i32 = 2;

pub fn suppress_stderr() {
    let c_str_nul = CString::new("nul").unwrap();
    let c_str_w = CString::new("w").unwrap();

    let fnul = unsafe { fopen(c_str_nul.as_ptr(), c_str_w.as_ptr()) };
    if fnul.is_null() {
        return
    }

    // FIXME: Not working in PowerShell.
    unsafe { dup2(fileno(fnul), STDERR_FILENO) };
}
