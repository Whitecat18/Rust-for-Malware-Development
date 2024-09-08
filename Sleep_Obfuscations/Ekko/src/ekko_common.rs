extern crate winapi;

use std::mem;
use std::ptr::null;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::PVOID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::libloaderapi::LoadLibraryA;
// use winapi::um::memoryapi::VirtualProtect;
use winapi::um::synchapi::CreateEventW;
// use winapi::um::synchapi::SetEvent;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::threadpoollegacyapiset::CreateTimerQueue;
use winapi::um::threadpoollegacyapiset::CreateTimerQueueTimer;
use winapi::um::winbase::DeleteTimerQueue;
// use winapi::um::winnt::RtlCaptureContext;
use winapi::um::winnt::CONTEXT;
// use winapi::shared::ntdef::NTSTATUS;
use winapi::um::winnt::WAITORTIMERCALLBACK;
use winapi::um::winnt::WT_EXECUTEINTIMERTHREAD;
use winapi::um::winnt::{IMAGE_DOS_HEADER,IMAGE_NT_HEADERS64};


#[repr(C)]
pub struct UString{
    length: u32,
    max_length: u32,
    buffer: *mut u16,
}

pub fn ekko(sleep_time: u32) {
    unsafe {
        
        let mut ctx_thread: CONTEXT = mem::zeroed();
        let mut rop_prot_rw: CONTEXT = mem::zeroed();
        let mut rop_mem_enc: CONTEXT = mem::zeroed();
        let mut rop_delay: CONTEXT = mem::zeroed();
        let mut rop_mem_dec: CONTEXT = mem::zeroed();
        let mut rop_prot_rx: CONTEXT = mem::zeroed();
        let mut rop_set_evt: CONTEXT = mem::zeroed();

        let mut h_new_timer: *mut c_void = null_mut();

        let h_timer_queue = CreateTimerQueue();

        let h_event = CreateEventW(
            null_mut(),
            0,
            0, 
            null()
        );

        if h_event.is_null(){
            println!("CreateEventW is Failed: {:?}", GetLastError());
            std::process::exit(0);
        }

        let mut old_protect: u32 = 0;

        let image_base = GetModuleHandleA(null_mut());

        let dos_header = image_base as *mut IMAGE_DOS_HEADER;
        // let e_lfanew = *dos_header.offset(0x3C);
        // let nt_headers = image_base.add(e_lfanew as usize) as *const u8;
        let nt_headers = (dos_header as u64 + (*dos_header).e_lfanew as u64) as *mut IMAGE_NT_HEADERS64;
        // let optional_header = nt_headers.add(24) as *const u32; // Assuming PE32+ structure
        let image_size = (*nt_headers).OptionalHeader.SizeOfImage;  // SizeOfImage

        // let key_buf: [u8; 16] = [0x55; 16];
        let mut key_buf: [u8; 16] = [49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 65, 66, 67, 68, 69, 70];    

        let key = UString{
            length: 16, 
            max_length: 16,
            buffer: key_buf.as_mut_ptr() as *mut u16,
        };

        let mut img = UString {
            length: image_size,
            max_length: image_size,
            buffer: image_base as *mut u16,
        };

        let rtl_capture_context = GetProcAddress(
            LoadLibraryA("ntdll\0".as_ptr() as *const i8), 
            "RtlCaptureContext\0".as_ptr() as *const i8) as u64;

        let rtl_capture_context_ptr = std::mem::transmute::<_, WAITORTIMERCALLBACK>(rtl_capture_context);

        let nt_continue = GetProcAddress(
            GetModuleHandleA("ntdll\0".as_ptr() as *const i8),
            "NtContinue\0".as_ptr() as *const i8) as u64;

        let nt_continue_ptr = std::mem::transmute::<_, WAITORTIMERCALLBACK>(nt_continue);

        let system_function032 = GetProcAddress(
            LoadLibraryA("Advapi32\0".as_ptr() as *const i8), 
            "SystemFunction032\0".as_ptr() as *const i8) as u64;
        let virtual_protect = GetProcAddress(
            LoadLibraryA("kernel32.dll\0".as_ptr() as *const i8 ),
            "VirtualProtect\0".as_ptr() as *const i8) as u64;
        let wait_for_single_object = GetProcAddress(
            LoadLibraryA("kernel32.dll\0".as_ptr() as *const i8 ), 
            "WaitForSingleObject\0".as_ptr() as *const i8) as u64;

        let set_event = GetProcAddress(
            LoadLibraryA("kernel32.dll\0".as_ptr() as *const i8 ), 
            "SetEvent\0".as_ptr() as *const i8)  as u64;

        // let sys_func_032 = GetProcAddress(LoadLibraryA("Advapi32.dll".as_ptr() as *const i8), "SystemFunction032".as_ptr() as *const i8);


        // let ctx_thread = std::mem::zeroed::<ProperlyAlignedContext>();


        let createtimerqueue = CreateTimerQueueTimer(
            &mut null_mut(), 
            h_timer_queue, 
            rtl_capture_context_ptr,
            &mut ctx_thread as *mut _ as *mut c_void,
            0,
            0,
            WT_EXECUTEINTIMERTHREAD,
        );

        if createtimerqueue != 0 {
            WaitForSingleObject(h_event, 0x32);

            rop_prot_rw.Rsp -= 8;
            rop_prot_rw.Rip = virtual_protect as u64;
            rop_prot_rw.Rcx = image_base as *const c_void as u64;
            rop_prot_rw.Rdx = image_size as u64;
            rop_prot_rw.R8 = 0x04 as u64; // PAGE_READWRITE
            rop_prot_rw.R9 = &mut old_protect as *mut _ as u64;

            rop_mem_enc.Rsp -= 8;
            rop_mem_enc.Rip = system_function032 as u64;
            rop_mem_enc.Rcx = &mut img as *mut _ as u64;
            rop_mem_enc.Rdx = &key as *const _ as u64;

            rop_delay.Rsp -= 8;
            rop_delay.Rip = wait_for_single_object as u64;
            rop_delay.Rcx = -1 as isize as u64;
            rop_delay.Rdx = sleep_time as u64;
    

            rop_mem_dec.Rsp -= 8;
            rop_mem_dec.Rip = system_function032 as u64;
            rop_mem_dec.Rcx = &mut img as *mut _ as u64;
            rop_mem_dec.Rdx = &key as *const _ as u64;
            
            rop_prot_rx.Rsp -= 8;
            rop_prot_rx.Rip = virtual_protect as u64;
            rop_prot_rx.Rcx = image_base as *const c_void as u64;
            rop_prot_rx.Rdx = image_size as u64;
            rop_prot_rx.R8 = 0x40;
            rop_prot_rx.R9 = &mut old_protect as *mut _ as u64;
    
            rop_set_evt.Rsp -= 8;
            rop_set_evt.Rip = set_event as u64;
            rop_set_evt.Rcx = h_event as u64;

            println!("[+] Queue times: ");
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_prot_rw as *const _ as PVOID, 100, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_mem_enc as *const _ as PVOID, 200, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_delay as *const _ as PVOID, 300, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_mem_dec as *const _ as PVOID, 400, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_prot_rx as *const _ as PVOID, 500, 0, WT_EXECUTEINTIMERTHREAD);
            
            CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, nt_continue_ptr, &rop_set_evt as *const _ as PVOID, 600, 0, WT_EXECUTEINTIMERTHREAD);

            println!("[+] h_event waitings");

            WaitForSingleObject(h_event, 0xFFFFFFFF);

            println!("[+] Waiting Time Finished");

            // for ctx in [&rop_prot_rw, &rop_mem_enc, &rop_delay, &rop_mem_dec, &rop_prot_rx, &rop_set_evt] {
            //     if CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, Some(nt_continue_wrapper), ctx as *const _ as PVOID, delay, 0, WT_EXECUTEINTIMERTHREAD) != 0 {
            //         delay += 100;
            //     } else {
            //         println!("Failed to create timer");
            //         break;
            //     }
            // }
        }

        DeleteTimerQueue(h_timer_queue);
    }
}

// extern "system" fn timer_callback(lp_parameter: *mut winapi::ctypes::c_void, _dw_timer_low_value: u8) {
//     let context = lp_parameter as *mut CONTEXT;
//     unsafe {
//         RtlCaptureContext(context);
//     }
// }

// extern "system" fn nt_continue_wrapper(lp_parameter: *mut winapi::ctypes::c_void, _dw_timer_low_value: u8) {
//     let context = lp_parameter as *mut CONTEXT;
//     unsafe {
//         let nt_continue: unsafe extern "system" fn(*mut CONTEXT) -> NTSTATUS = std::mem::transmute(GetProcAddress(GetModuleHandleA("Ntdll\0".as_ptr() as *const _), "NtContinue\0".as_ptr() as *const _));
//         nt_continue(context);
//     }
// }


