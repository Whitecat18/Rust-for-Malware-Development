/*
    Windows API Hooking
    For more Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    Resources used: https://www.ired.team/offensive-security/code-injection-process-injection/how-to-hook-windows-api-using-c++
                    https://forums.codeguru.com/showthread.php?468963-sample-source-MessageBox-API-Hook&p=1806470#post1806470
    @5mukx
*/

use std::ptr::null_mut;
use user32::MessageBoxA;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::UINT;
use winapi::shared::ntdef::{LPCSTR, LPSTR};
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory};
use winapi::um::processthreadsapi::GetCurrentProcess;
use std::ffi::{CStr, CString};
use std::ptr::addr_of_mut;

type FarProc = *mut c_void;
type SizeT = usize;

static mut MESSAGE_BOX_ADDRESS: FarProc = null_mut();
static mut MESSAGE_BOX_ORIGINAL_BYTES: [u8; 6] = [0; 6];
static mut BYTES_WRITTEN: SizeT = 0;

unsafe extern "system" fn hooked_message_box(
    hwnd: HWND,
    lp_text: LPSTR,
    lp_caption: LPCSTR,
    u_type: UINT,
) -> i32 {
    let text = CStr::from_ptr(lp_text).to_str().unwrap_or("INVALID UTF-8");
    let caption = CStr::from_ptr(lp_caption).to_str().unwrap_or("INVALID UTF-8");
    println!("Nerdy from Hooded Function");
    println!("Text : {}",text);
    println!("Caption: {}",caption);

    WriteProcessMemory(
        hwnd as *mut c_void,
        MESSAGE_BOX_ADDRESS as *mut _,
        MESSAGE_BOX_ORIGINAL_BYTES.as_ptr() as *const _,
        MESSAGE_BOX_ORIGINAL_BYTES.len(),
        addr_of_mut!(BYTES_WRITTEN),
    );

    MessageBoxA(hwnd as _, 
        lp_text, 
        lp_caption, 
        u_type
    )
}

fn main(){
    let box_title = CString::new("NerdyX...").expect("error");
    let message = CString::new("Hello Nerds. Lets Hook some API's").expect("error");

    unsafe{
        MessageBoxA(
            null_mut(),
            message.as_ptr() as LPCSTR,
            box_title.as_ptr() as LPCSTR, // *const CHAR
            0x00000000, // MB_OK
        );

        let library = LoadLibraryA("user32.dll\0".as_ptr() as *const i8);
        let mut bytes_read: SizeT = 0;

        MESSAGE_BOX_ADDRESS = GetProcAddress(library, "MessageBoxA\0".as_ptr() as *const i8) as FarProc;

        ReadProcessMemory(
            GetCurrentProcess(),
            MESSAGE_BOX_ADDRESS,
            MESSAGE_BOX_ORIGINAL_BYTES.as_mut_ptr() as *mut _,
            MESSAGE_BOX_ORIGINAL_BYTES.len(),
            &mut bytes_read,
        );

        let hook_text = CString::new("Nerdys.. im from Hook func").expect("error");
        let hook_heading = CString::new("Hooked").expect("error");
        
        let hook_msg_box_addr: FarProc = hooked_message_box(
            null_mut(),
            hook_text.as_ptr() as _,
            hook_heading.as_ptr() as _,
            0x00000000,
        ) as *mut c_void;

        let patch: [u8; 6] = [0x68, 0, 0, 0, 0, 0xC3];
        *(patch.as_ptr().offset(1) as *mut FarProc) = hook_msg_box_addr;

        WriteProcessMemory(
            GetCurrentProcess(),            
            MESSAGE_BOX_ADDRESS as *mut _,
            patch.as_ptr() as *const _,
            patch.len(),
            addr_of_mut!(BYTES_WRITTEN),
        );

        let after_hook_title = CString::new("After Hooked").expect("error");
        let after_hook_text = CString::new("Nerdies .. This is after Hooked").expect("error"); 
        
        MessageBoxA(
            null_mut(),
            after_hook_text.as_ptr() as LPCSTR,
            after_hook_title.as_ptr() as LPCSTR,
            0x00000000,
        );
    }
}
