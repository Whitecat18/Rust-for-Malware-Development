// Temp.rs file. Dll 

use std::ffi::{c_ulong, CString, OsString};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{self, addr_of, null_mut};

use winapi::shared::bcrypt::NTSTATUS;
use winapi::shared::minwindef::{HMODULE, PULONG};
use winapi::shared::ntdef::{LPCSTR, PUNICODE_STRING, PVOID, PWSTR, UNICODE_STRING};
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::shared::windef::HWND;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE};

type PNewLdrLoadDll = unsafe extern "system" fn(
    DllPath: PWSTR,
    Dllcharacters: PULONG,
    DllName: PUNICODE_STRING,
    DLLHandle: *mut PVOID,
) -> NTSTATUS;

type PMyMessageBoxA = unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, u32) -> i32;

unsafe fn copy_memory(dest: *mut u8, source: *const u8, length: usize){
    let mut d = dest;
    let mut s = source;

    for _ in 0..length{
        *d = *s;
        d = d.add(1);
        s = s.add(1);
    }
}

fn main(){
    unsafe{
        let h_module = LoadLibraryA(
            b"ntdll.dll".as_ptr() as *const i8
        );
        let origin = GetProcAddress(
            h_module, 
            b"LdrLoadDll\0".as_ptr() as *const i8
        );

        let jmp_addr: *const () = (origin as usize) as *const();

        let orgin: [u8; 5] = [0x48, 0x89, 0x5c, 0x24, 0x10];
        let jmp_prelude: [u8; 2] = [0x49, 0xBB];
        let jump_epilogue: [u8; 4] = [0x41, 0xFF, 0xE3, 0xC3];
        let trampole = VirtualAlloc(
            null_mut(),
            19,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );

        // ADDRESS CHANGE STARTS HERE 

        let addr_ptr = addr_of!(jmp_addr) as *const u8;
        copy_memory(trampole as *mut u8, orgin.as_ptr(), 5);
        copy_memory(trampole.add(5) as *mut u8, jmp_prelude.as_ptr() , 2);
        copy_memory(trampole.add(5).add(2) as *mut u8, addr_ptr, 8);
        copy_memory(trampole.add(5).add(2).add(8) as *mut u8, jump_epilogue.as_ptr(), 4);

        let mut oldprotect:u32 = 0;
        VirtualProtect(
            trampole,
            30, 
            PAGE_EXECUTE_READ, 
            oldprotect as *mut u32,
        );
        
        let ldr_load_dll: PNewLdrLoadDll = std::mem::transmute(trampole);


        let user32_dll_name: Vec<u16> = OsString::from("user32.dll")
            .encode_wide().chain(std::iter::once(0)).collect();


        let mut user32_dll_unicode = UNICODE_STRING{
            Length: ((user32_dll_name.len() - 1) * 2) as u16,
            MaximumLength: (user32_dll_name.len() * 2) as u16,
            Buffer: user32_dll_name.as_ptr() as *mut _,
        }; 
        let mut user32handle = null_mut();

        ldr_load_dll(ptr::null_mut(), 0 as PULONG, &mut user32_dll_unicode, &mut user32handle);
        let user32handle: HMODULE  = std::mem::transmute(user32handle);

        let my_message_box_a_addr = GetProcAddress(user32handle, b"MessageBoxA\0".as_ptr() as *const i8);
        let MessageBox: PMyMessageBoxA = std::mem::transmute(my_message_box_a_addr);

        let text_cstring = CString::new("Hello, Workd!").expect("error converting into cstring");
        let caption_string = CString::new("Title").expect("Error");

        // let hwnd = HWND(0);

        MessageBox( null_mut(), 
            text_cstring.as_ptr(), 
            caption_string.as_ptr(), 
            0 as u32
        );
    }
}


#[repr(transparent)]
pub struct PageProtectionFlags(pub u32);
