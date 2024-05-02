use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Console::{
    self, CONSOLE_MODE, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
    ENABLE_PROCESSED_OUTPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, STD_INPUT_HANDLE,
    STD_OUTPUT_HANDLE,
};

pub fn enable_raw_input_mode() -> CONSOLE_MODE {
    let handle = get_stdin_handle();

    let original_input_mode = get_mode(handle);
    let raw_input_mode = make_input_raw(original_input_mode);

    set_input_mode(raw_input_mode);

    original_input_mode
}

pub fn enable_output_virtual_terminal_processing() -> CONSOLE_MODE {
    let handle = get_stdout_handle();

    let original_output_mode = get_mode(handle);
    let vt_output_mode = make_output_virtual_terminal(original_output_mode);

    set_output_mode(vt_output_mode);

    original_output_mode
}

pub fn set_input_mode(mode: CONSOLE_MODE) {
    let handle = get_stdin_handle();

    let ret = unsafe { Console::SetConsoleMode(handle, mode) };
    if ret.is_err() {
        panic!("failed to set console input mode");
    }
}

pub fn set_output_mode(mode: CONSOLE_MODE) {
    let handle = get_stdout_handle();

    let ret = unsafe { Console::SetConsoleMode(handle, mode) };
    if ret.is_err() {
        panic!("failed to set console output mode");
    }
}

fn get_stdin_handle() -> HANDLE {
    let handle = unsafe { Console::GetStdHandle(STD_INPUT_HANDLE) };

    handle.expect("failed to get console stdin handle")
}

fn get_stdout_handle() -> HANDLE {
    let handle = unsafe { Console::GetStdHandle(STD_OUTPUT_HANDLE) };

    handle.expect("failed to get console stdout handle")
}

fn get_mode(handle: HANDLE) -> CONSOLE_MODE {
    let mut mode = CONSOLE_MODE(0);

    let ret = unsafe { Console::GetConsoleMode(handle, &mut mode) };
    if ret.is_err() {
        panic!("failed to get console mode");
    }

    mode
}

fn make_input_raw(mode: CONSOLE_MODE) -> CONSOLE_MODE {
    let mut raw_mode = mode;

    raw_mode &= !ENABLE_ECHO_INPUT;
    raw_mode &= !ENABLE_LINE_INPUT;
    raw_mode &= !ENABLE_PROCESSED_INPUT;

    raw_mode
}

fn make_output_virtual_terminal(mode: CONSOLE_MODE) -> CONSOLE_MODE {
    let mut vt_mode = mode;

    vt_mode |= ENABLE_PROCESSED_OUTPUT;
    vt_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    vt_mode
}
