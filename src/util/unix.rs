use std::{fs::File, os::unix::io::IntoRawFd};
use libc::{dup2, STDERR_FILENO};

pub fn suppress_stderr() {
    let fd = match File::create("/dev/null") {
        Ok(file) => file.into_raw_fd(),
        Err(_) => return
    };
    unsafe { dup2(fd, STDERR_FILENO) };
}
