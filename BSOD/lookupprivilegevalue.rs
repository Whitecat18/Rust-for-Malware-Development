/*
    Program to invoke BSOD setting up privileges and provoking NtRaiseHardError. 
    @5mukx

*/

use std::ptr;
use ntapi::ntexapi::NtRaiseHardError;
use winapi::shared::ntstatus::STATUS_ASSERTION_FAILURE;
use winapi::shared::wtypesbase::ULONG;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winnt::{LUID, SE_PRIVILEGE_ENABLED, SE_SHUTDOWN_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY};
use winapi::um::winbase::LookupPrivilegeValueA;
use winapi::um::errhandlingapi::GetLastError;
use std::ffi::CString;

fn main() {
    println!("Press any key to trigger a BSOD.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    unsafe {
        let mut token_handle: winapi::um::winnt::HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token_handle) == 0 {
            println!("Failed to open process token.");
            return;
        }

        let mut luid: LUID = LUID { LowPart: 0, HighPart: 0 };
        let shutdown_privilege = CString::new(SE_SHUTDOWN_NAME).unwrap();
        if LookupPrivilegeValueA(ptr::null(), shutdown_privilege.as_ptr(), &mut luid) == 0 {
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

        if GetLastError() != 0 {
            println!("Failed to adjust token privileges. Error: {}", GetLastError());
            return;
        }

        // Raise hard error
        let mut response: ULONG = 0;
        let status = NtRaiseHardError(
            STATUS_ASSERTION_FAILURE,
            0,
            0,
            ptr::null_mut(),
            6,
            &mut response
        );

        if status != 0 {
            println!("Failed to raise hard error. Status: {}", status);
        }
    }
}

