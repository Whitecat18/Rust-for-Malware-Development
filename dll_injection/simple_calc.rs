/*
Executing calc.exe by using WINAPI'S 
*/

use std::ptr;
use std::ffi::CString;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{STARTUPINFOA, PROCESS_INFORMATION, CreateProcessA};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::{CREATE_NEW_CONSOLE, INFINITE};

#[no_mangle]
pub extern "stdcall" fn DllMain() {
    unsafe {
        let mut startup_info: STARTUPINFOA = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

        let application_name = CString::new("C:\\Windows\\System32\\calc.exe").expect("CString::new failed");

        let success = CreateProcessA(
            ptr::null(),
            application_name.as_ptr() as *mut i8,
            ptr::null_mut(),
            ptr::null_mut(),
            false as i32,
            CREATE_NEW_CONSOLE,
            ptr::null_mut(),
            ptr::null(),
            &mut startup_info,
            &mut process_info,
        );

        if success == 0 {
            println!("Failed to open Calculator !");
            return
        }
        WaitForSingleObject(success as *mut winapi::ctypes::c_void, INFINITE);
        CloseHandle(success as *mut winapi::ctypes::c_void);
    }
}
