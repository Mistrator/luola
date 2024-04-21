use libc::{termios, STDIN_FILENO, TCSANOW};

pub fn enable_raw_mode() -> termios {
    let mut mode = init_termios();

    get_mode(&mut mode);
    let original_mode = mode.clone();

    unsafe { libc::cfmakeraw(&mut mode) };
    set_mode(&mode);

    original_mode
}

pub fn set_mode(mode: &termios) {
    // Type coercion from &T to *const T
    let ret = unsafe { libc::tcsetattr(STDIN_FILENO, TCSANOW, mode) };
    if ret != 0 {
        panic!("failed to set terminal mode");
    }

    let mut actual_mode = init_termios();
    get_mode(&mut actual_mode);

    // libc::tcsetattr() returns success if any of the requested changes was successful,
    // not if all of them were successful. Thus, check that all changes were actually
    // applied.
    if *mode != actual_mode {
        panic!("failed to set terminal mode: not all changes were applied");
    }
}

fn get_mode(mode: &mut termios) {
    // Type coercion from &mut T to *mut T
    let ret = unsafe { libc::tcgetattr(STDIN_FILENO, mode) };
    if ret != 0 {
        panic!("failed to get terminal mode");
    }
}

fn init_termios() -> termios {
    // We could use std::mem::MaybeUninit here but we initialize this manually
    // to keep it simple: it seems easy to accidentally misuse MaybeUninit
    // and cause UB.
    termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    }
}
