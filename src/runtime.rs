use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::process::Command;

#[no_mangle]
pub extern "C" fn spaceship_run_process(command: *const c_char, args: *mut *mut c_char) -> c_int {
    if command.is_null() {
        return -1;
    }

    let command = unsafe { CStr::from_ptr(command) };
    let command = match command.to_str() {
        Ok(text) => text,
        Err(_) => return -1,
    };

    let mut cmd = Command::new(command);

    if !args.is_null() {
        let mut idx = 0;
        while unsafe { !(*args.add(idx)).is_null() } {
            let arg_ptr = unsafe { *args.add(idx) };
            if arg_ptr.is_null() {
                break;
            }

            let value = unsafe { CStr::from_ptr(arg_ptr) };
            if let Ok(value) = value.to_str() {
                cmd.arg(value);
            }
            idx += 1;
        }
    }

    match cmd.status() {
        Ok(status) => status.code().unwrap_or(-1),
        Err(_) => -1,
    }
}
