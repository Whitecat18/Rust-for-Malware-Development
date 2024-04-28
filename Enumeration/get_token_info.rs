
/*
    Windows process token manipulation, privilege checking, and elevation.
    For More codes : https://github.com/Whitecat18/Rust-for-Malware-Development.git
    Resouces Used: https://medium.com/@s12deff/process-token-manipulation-8983e92a824
    @5mukx
*/

// WInAPI's That you need to import 
// * securitybaseapi

#[allow(unused_assignments)]

use winapi::ctypes::c_void;// use std::mem;
use std::mem;
use std::ptr::{null,null_mut};
use std::io::Error;
use winapi::um::winnt::{TokenElevation, TokenPrivileges, PRIVILEGE_SET_ALL_NECESSARY, PTOKEN_PRIVILEGES, TOKEN_ELEVATION};
use winapi::um::{handleapi::CloseHandle, processthreadsapi::{GetCurrentProcess, OpenProcessToken}, winbase::LookupPrivilegeValueA, winnt::{LUID, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY}};
// use windows::Win32::Security::AdjustTokenPrivileges;
use winapi::shared::ntdef::HANDLE;
use winapi::um::securitybaseapi::{AdjustTokenPrivileges, GetTokenInformation, PrivilegeCheck};
use winapi::um::winnt::PRIVILEGE_SET;
// use windows::Win32::Security::{PRIVILEGE_SET, TOKEN_PRIVILEGES_ATTRIBUTES};
use winapi::um::winbase::LookupPrivilegeNameA;
use std::ffi::CString;

fn set_debug_token() -> Result<(), Error>{
    unsafe{
        let mut h_token:HANDLE = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut h_token) != 0{
            let mut luid: LUID = std::mem::zeroed();
                                                        // or b"SeDebugPrivilege\0".as_ptr()
            if LookupPrivilegeValueA(null(), SE_DEBUG_NAME.as_ptr() as *const i8, &mut luid) != 0 {
                let mut token_privilages: TOKEN_PRIVILEGES = std::mem::zeroed();
                token_privilages.PrivilegeCount = 1;
                token_privilages.Privileges[0].Luid = luid;
                token_privilages.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
                if AdjustTokenPrivileges(h_token, 0,  &mut token_privilages, mem::size_of::<TOKEN_PRIVILEGES>() as u32, null_mut(), null_mut()) != 0{
                    CloseHandle(h_token);
                    return Ok(())
                }
            }
            CloseHandle(h_token);
        } 
        Err(Error::last_os_error())
    }
}

fn check_debug_privileges() -> Result<bool, Error>{
    unsafe{
        let mut luid: LUID = mem::zeroed();
        if LookupPrivilegeValueA(null(), SE_DEBUG_NAME.as_ptr() as *const i8,&mut luid) != 0{
            let mut privs: PRIVILEGE_SET = std::mem::zeroed();
            let mut b_result: i32 = 0;
            let mut h_token: *mut c_void = null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token) != 0{
                privs.PrivilegeCount = 1;
                privs.Control = PRIVILEGE_SET_ALL_NECESSARY; // u32
                privs.Privilege[0].Luid = luid;
                privs.Privilege[0].Attributes = SE_PRIVILEGE_ENABLED; //TOKEN_PRIVILEGES_ATTRIBUTES
                PrivilegeCheck(h_token, &mut privs, &mut b_result);
                CloseHandle(h_token); 
                return Ok(b_result != 0);
            }
        }
        Err(Error::last_os_error())
    }
}

fn get_privileges() -> Result<(), Error>{
    unsafe{
        let mut h_token: HANDLE = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token) != 0{
            let mut token_elevation: TOKEN_ELEVATION = std::mem::zeroed(); 
            let mut cb_size: u32 = mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut tp_size: u32 = 0;
            #[allow(unused_assignments)]
            let mut length: u32 = 0;
            let mut name: [i8; 256] = [0;256];

            GetTokenInformation(h_token, TokenPrivileges, null_mut(), 0, &mut tp_size);
            // let mut
            let ptoken_privileges: PTOKEN_PRIVILEGES = Vec::<c_void>::with_capacity(tp_size as usize).as_mut_ptr() as PTOKEN_PRIVILEGES;

            if GetTokenInformation(h_token, TokenPrivileges, ptoken_privileges as *mut c_void, tp_size, &mut tp_size) != 0{
                for i in 0..(*ptoken_privileges).PrivilegeCount{
                    length = 256;
                    LookupPrivilegeNameA(null(), &mut (*ptoken_privileges).Privileges[i as usize].Luid, name.as_mut_ptr(), &mut length);
                    if (*ptoken_privileges).Privileges[i as usize].Attributes == 3 {
                        println!("[+] {:<50} Enabled (Default)", CString::from_raw(name.as_mut_ptr()).into_string().unwrap());
                    } else if (*ptoken_privileges).Privileges[i as usize].Attributes == 2 {
                        println!("[+] {:<50} Enabled (Adjusted)", CString::from_raw(name.as_mut_ptr()).into_string().unwrap());
                    } else if (*ptoken_privileges).Privileges[i as usize].Attributes == 0 {
                        println!("[+] {:<50} Disabled", CString::from_raw(name.as_mut_ptr()).into_string().unwrap());
                    }
                }
            }

            if GetTokenInformation(h_token, TokenElevation, &mut token_elevation as *mut TOKEN_ELEVATION as *mut c_void, mem::size_of::<TOKEN_ELEVATION>() as u32 , &mut cb_size) != 0{
                if token_elevation.TokenIsElevated != 0{
                    println!("[+] Elevated");
                } else {
                    println!("[-] Restricted");
                }
            }
            CloseHandle(h_token);
            return Ok(());
        } else {
            Err(Error::last_os_error())
        }
    }
}

fn main() -> Result<(), Error>{
    get_privileges()?;
    if let Err(err) = set_debug_token(){
        return Err(err);
    }

    if let Ok(true) = check_debug_privileges(){
        println!("--- PRIVILEGES MODIFIED ---");
        get_privileges()?;
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}
