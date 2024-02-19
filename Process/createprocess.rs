extern crate winapi;

#[allow(unused_imports)]
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, process::exit, ptr::{self, null_mut}
};
use winapi::{
  shared::minwindef::FALSE,
  um::{
  processthreadsapi::{PROCESS_INFORMATION,STARTUPINFOW,{CreateProcessW,GetProcessId,GetThreadId}},
  winbase::INFINITE,
  // handleapi::CloseHandle,
  synchapi::WaitForSingleObject,
  errhandlingapi::GetLastError,
}};

fn main(){
  let mut path = OsStr::new("C:\\Windows\\System32\\notepad.exe").encode_wide().collect::<Vec<_>>();
  let cmd = path.as_mut_ptr() as *mut u16 ;
  let mut startup: STARTUPINFOW = unsafe{ std::mem::zeroed()};
  let mut process_info: PROCESS_INFORMATION = unsafe{std::mem::zeroed()};

  unsafe{
  /* 0s and 1s BOOL
    BOOL CreateProcessW(
      [in, optional]      LPCWSTR               lpApplicationName,
      [in, out, optional] LPWSTR                lpCommandLine,
      [in, optional]      LPSECURITY_ATTRIBUTES lpProcessAttributes,
      [in, optional]      LPSECURITY_ATTRIBUTES lpThreadAttributes,
      [in]                BOOL                  bInheritHandles,
      [in]                DWORD                 dwCreationFlags,
      [in, optional]      LPVOID                lpEnvironment,
      [in, optional]      LPCWSTR               lpCurrentDirectory,
      [in]                LPSTARTUPINFOW        lpStartupInfo,
      [out]               LPPROCESS_INFORMATION lpProcessInformation
    );
  */

    if CreateProcessW(
      ptr::null(),
      cmd,
      ptr::null_mut(),
      ptr::null_mut(),
      FALSE,
      0,
      ptr::null_mut(),
      ptr::null(),
      &mut startup,
      &mut process_info,
    ) == 0{
      println!("(-) Failed to create Process, Error: {}",GetLastError());
      exit(1);
    }

    let pid = GetProcessId(process_info.hProcess);
    let tid = GetThreadId(process_info.hThread);

    println!("(+) got handle to process");
    println!("(+) process started! pid: {}",pid);
    println!("\t(+) pid:{} | handle: {:?}",pid,pid);
    println!("\t(+) tid:{} | handle: {:?}",tid,tid);

    WaitForSingleObject(process_info.hProcess, INFINITE);
    println!("(+) Finish Exiting...");

    // In Rust we dont need to free up its allocated memory because when going out of scops rust automatically cleans up the memory due to its ownership and resource management system.
    // If you need so you can clean by yourself !...
    
    // CloseHandle(process_info.hThread); 
    // CloseHandle(process_info.hProcess);
  }
}
