use std::ffi::CString;
use std::ptr::{null,null_mut};
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{STARTUPINFOA, PROCESS_INFORMATION, CreateProcessA};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winbase::{CREATE_NEW_CONSOLE, INFINITE};
use winapi::um::libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleA};


#[no_mangle]

pub extern "stdcall" fn DllMain(_module: *mut u8, reason: DWORD, _reserved: *mut u8) -> bool {
    match reason {
        1 => {
            unsafe {
                let mut startup_info: STARTUPINFOA = std::mem::zeroed();
                startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

                let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

                let application_name = CString::new("C:\\Windows\\System32\\calc.exe").expect("CString::new failed");

                let success = CreateProcessA(
                    null(),
                    application_name.as_ptr() as *mut i8,
                    null_mut(),
                    null_mut(),
                    false as i32,
                    CREATE_NEW_CONSOLE,
                    null_mut(),
                    null(),
                    &mut startup_info,
                    &mut process_info,
                );

                if success == 0 {
                    println!("Failed to open Calculator !");
                    return false;
                }

                WaitForSingleObject(process_info.hProcess, INFINITE);

                CloseHandle(process_info.hProcess);
                CloseHandle(process_info.hThread);

                let module_handle = GetModuleHandleA(null());
                FreeLibraryAndExitThread(module_handle, 0);

            }
        },
        0 => {
            return false;
        },
        _ => {},
    }
    true
}
