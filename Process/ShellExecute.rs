extern crate winapi;

// MSDN For ShellExecute : https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutea
use winapi::um::{winuser::SW_SHOWNORMAL,
    shellapi::ShellExecuteA};
use std::ptr;
use std::ffi::CString;
use winapi::shared::ntdef::NULL;

fn main() {
    let command = CString::new("calc.exe").unwrap();
    let verb = CString::new("runas").unwrap();

    // For Folder .. 
    // let parameters = CString::new("").unwrap();
    // let directory = CString::new("").unwrap();
    let dirc = NULL as _;

    unsafe {
        ShellExecuteA(
            ptr::null_mut(),
            verb.as_ptr(),
            command.as_ptr(),
            dirc,
            dirc,
            SW_SHOWNORMAL,
        );
    }
}
