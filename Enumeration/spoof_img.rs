/*
Image Spoofer using NtOpenFile, NtCreateSection, and NtMapViewOfSection.
For more maldev codes -> https://github.com/Whitecat18/Rust-for-Malware-Development.git
@5mukx
*/

#![allow(unused_imports)]
use std::{ffi::OsStr, io::{self, Write}, os::windows::ffi::OsStrExt, ptr::{self, null_mut}};
use ntapi::{ntioapi::{NtCreateFile, NtOpenFile, FILE_OPEN, FILE_SYNCHRONOUS_IO_NONALERT, IO_STATUS_BLOCK}, ntmmapi::{NtCreateSection, NtMapViewOfSection}};
use winapi::{ctypes::c_void, shared::{bcrypt::NTSTATUS, minwindef::USHORT, ntdef::{OBJECT_ATTRIBUTES, OBJ_CASE_INSENSITIVE, PLARGE_INTEGER, UNICODE_STRING}}, um::{handleapi::CloseHandle, memoryapi::VirtualFree, winnt::{FILE_GENERIC_READ, FILE_SHARE_READ, MEM_RELEASE, PVOID, SECTION_MAP_EXECUTE, SECTION_MAP_READ, SECTION_MAP_WRITE, SEC_IMAGE}}};
// use windows::Win32::Foundation::NTSTATUS;

fn main(){
    if !spoof_image_loading(){
        eprintln!("[-] Failed to spoof image loading");
    }
}

fn spoof_image_loading() -> bool{
    unsafe{
        let module_name = OsStr::new(r"\\??\\C:\\windows\\system32\\advapi32.dll")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();
        
        let mut file_handle: *mut c_void = ptr::null_mut();
        let mut section_handle: *mut c_void = ptr::null_mut();
        let mut base_addr: PVOID = null_mut();
        let mut io_status_block: IO_STATUS_BLOCK = std::mem::zeroed();
        let mut object_attributes: OBJECT_ATTRIBUTES = std::mem::zeroed();

        let mut unicode_string = UNICODE_STRING{
            Length: (module_name.len() * 2) as USHORT,
            MaximumLength: ((module_name.len() + 1) * 2) as USHORT,
            Buffer: module_name.as_ptr() as *mut _,
        };

        object_attributes.Length = std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32;
        object_attributes.ObjectName = &mut unicode_string;
        object_attributes.Attributes = OBJ_CASE_INSENSITIVE;

        // let status: NTSTATUS = NtCreateFile(
        //     &mut file_handle,
        //     FILE_GENERIC_READ, 
        //     &mut object_attributes,
        //     &mut io_status_block,
        //     null_mut() as PLARGE_INTEGER,
        //     0,
        //     FILE_SHARE_READ,
        //     FILE_OPEN,
        //     FILE_SYNCHRONOUS_IO_NONALERT,
        //     null_mut(),
        //     0,
        // );

        //    NTSTATUS ntStatus = NtOpenFile(&hFile, 0x100021, &objectAttributes, &ioStatusBlock, 5, 0x60);

        let status: NTSTATUS = NtOpenFile(
                &mut file_handle, 0x100021, &mut object_attributes, &mut io_status_block, 5, 0x60);

        println!("[+] NtOpenFile -> {:#?}",status);

        // let status: NTSTATUS = NtOpenFile(
        //     &mut file_handle, FILE_GENERIC_READ, &mut object_attributes, &mut io_status_block, FILE_SHARE_READ, FILE_OPEN);

        if status !=0{
            return false;
            // println!("[-] False")
        }

        // let status = NtCreateSection(
        //     &mut section_handle, 
        //     SECTION_MAP_READ | SECTION_MAP_WRITE | SECTION_MAP_EXECUTE,
        //     null_mut(),
        //     null_mut(),
        //     0x10,
        //     SEC_IMAGE,
        //     file_handle
        // );

        let status = NtCreateSection(
            &mut section_handle, 
            0xd,
            null_mut(),
            null_mut(),
            0x10,
            SEC_IMAGE,
            file_handle
        );
    
        println!("[+] NtCreateSection -> {:#?}",status);


        if status != 0 {
            CloseHandle(file_handle);
            return false;
        }

        // http://undocumented.ntinternals.net/index.html?page=UserMode%2FUndocumented%20Functions%2FNT%20Objects%2FSection%2FNtMapViewOfSection.html
        // let status = NtMapViewOfSection(
        //     section_handle, 
        //     null_mut(),
        //     &mut base_addr,
        //     0, 
        //     0,
        //     null_mut(),
        //     null_mut(), 
        //     0, 
        //     0x800000, 
        //     0x80,
        // );

        let status = NtMapViewOfSection(
            section_handle, 
            0xFFFFFFFFFFFFFFFF as *mut c_void,
            &mut base_addr,
            0, 
            0,
            null_mut(),
            null_mut(), 
            0x1, 
            0x800000, 
            0x80,
        );

        if status != 0{
            CloseHandle(section_handle);
            CloseHandle(file_handle);
            return false;
        }

        println!("[+] Image Loaded at {:#?}",base_addr);
        println!("Press any key to Exit");

        let _ = std::io::stdout().flush();
        let _ = io::stdin().read_line(&mut String::new());

        // Closing All handles...

        CloseHandle(section_handle);
        CloseHandle(file_handle);
        VirtualFree(base_addr, 0, MEM_RELEASE);
    }
    true
}
