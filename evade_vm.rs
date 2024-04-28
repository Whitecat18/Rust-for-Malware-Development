/*
    Anti-Virtualization / Full-System Emulation 
    For More Malware POC: https://github.com/Whitecat18/Rust-for-Malware-Development.git
    Resources Used: https://github.com/LordNoteworthy/al-khaser
    By @5mukx
*/


/*

Note: 
    [Dev Machine] -> Installed VM's Softwares and some development tools for malware testing. 
    I have comment out some code due to testing purpose. If you execute this code on development machines[Dev Machine] , ofcouse its gonna result out 
    {Machine Running in Vitrualmachine}. So to avoid testing, i have commented out some codes with // sus // tag.

    If you are executing this on normal machines such as schools and office computers, means you can uncomment codes that was tagged with -> // sus //

    [+] This is an All in one resource gathered together and coded..
            If you want to exec even more fast 1.1 to 0.2 secs.
                Reduce the content of the program or artifacts and keep up the main one for 
*/


use std::process::Command;
use std::fs;
use raw_cpuid::CpuId;

macro_rules! okey {
    ($msg:expr) => {
        
        println!("\n----[+]\\  {}  //[+]----\n",format!($msg));
    }
}

macro_rules! error {
    ($msg:expr) => {
        println!("\n----[-]\\  {}  //[-]----\n\n", format!($msg));
    }
}

fn main(){
    let vm_detect = check_vm();
    if vm_detect{
        error!("VM Detected. Malware Running in sandbox"); // bruh... ;(
    }else{
        okey!("Malware Runnung on main Machine"); // Yayy .. ;)
    }
}

fn check_vm() -> bool{
    
    //##=> Registry key value artifacts

    let registry_keys_value_artifacts = vec![
        // (r#"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion"#, "",""), // Example test case to see if this reg key attri wokrs ! Dont uncomment this !
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "VMWARE"),
        (r#"HKLM\SOFTWARE\VMware, Inc.\VMware Tools"#, "", ""),
        (r#"HKLM\HARDWARE\Description\System\SystemBiosVersion"#, "", "VMWARE"),
        (r#"HKLM\HARDWARE\Description\System\SystemBiosVersion"#, "", "VBOX"),
        (r#"HKLM\SOFTWARE\Oracle\VirtualBox Guest Additions"#, "", ""),
        (r#"HKLM\HARDWARE\ACPI\DSDT\VBOX__"#, "", ""),
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "VBOX"),
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "QEMU"),
        (r#"HKLM\HARDWARE\Description\System\SystemBiosVersion"#, "", "VBOX"),
        (r#"HKLM\HARDWARE\Description\System\SystemBiosVersion"#, "", "QEMU"),
        (r#"HKLM\HARDWARE\Description\System\VideoBiosVersion"#, "", "VIRTUALBOX"),
        (r#"HKLM\HARDWARE\Description\System\SystemBiosDate"#, "", "06/23/99"),
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "VMWARE"),
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 1\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "VMWARE"),
        (r#"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 2\Scsi Bus 0\Target Id 0\Logical Unit Id 0"#, "Identifier", "VMWARE"),
        (r#"HKLM\SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemManufacturer", "VMWARE"),
        (r#"HKLM\SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemProductName", "VMWARE"),
    ];

    let registry_keys_value_artifacts_value = registry_keys_value_artifacts.iter().any(|&(key, value_name, expected_value)| {
        let key_exists = registry_key_exists(key);
        let value_matches = registry_value_matches(key, value_name, expected_value);
        key_exists && value_matches
    });

    //##==> Registry Keys artifacts

    let registry_keys_artifacts = vec![
        r#"HKEY_LOCAL_MACHINE\HARDWARE\ACPI\DSDT\VBOX__"#,
        r#"HKEY_LOCAL_MACHINE\HARDWARE\ACPI\FADT\VBOX__"#,
        r#"HKEY_LOCAL_MACHINE\HARDWARE\ACPI\RSDT\VBOX__"#,
        r#"HKEY_LOCAL_MACHINE\SOFTWARE\Oracle\VirtualBox Guest Additions"#,
        r#"HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Services\VBoxGuest"#,
        r#"HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Services\VBoxMouse"#,
        r#"HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Services\VBoxService"#,
        r#"HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Services\VBoxSF"#,
        r#"HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Services\VBoxVideo"#,
        r#"HKEY_LOCAL_MACHINE\SOFTWARE\VMware, Inc.\VMware Tools"#,
        r#"HKEY_LOCAL_MACHINE\SOFTWARE\Wine"#,
        r#"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Virtual Machine\Guest\Parameters"#,

        //  // Main machines contains this reg key. So uncomment this !
        // If you are exec it on developer machine means ofcourse the reg contains in it ..  

        // r#"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Disk\Enum"#, // sus //
        // r#"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Enum\IDE"#, // sus //
        // r#"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Enum\SCSI"#, // sus //
    ];

    let registry_keys_artifacts_value = registry_keys_artifacts.iter().any(|&key| registry_key_exists(key));

    //##==> Checking File System artifacts !
    
    let file_system_artifacts = vec![
       r#"C:\Windows\system32\drivers\VBoxMouse.sys"#,
       r#"C:\Windows\system32\drivers\VBoxGuest.sys"#,
       r#"C:\Windows\system32\drivers\VBoxSF.sys"#,
       r#"C:\Windows\system32\drivers\VBoxVideo.sys"#,
       r#"C:\Windows\system32\vboxdisp.dll"#,
       r#"C:\Windows\system32\vboxhook.dll"#,
       r#"C:\Windows\system32\vboxmrxnp.dll"#,
       r#"C:\Windows\system32\vboxogl.dll"#,
       r#"C:\Windows\system32\vboxoglarrayspu.dll"#,
       r#"C:\Windows\system32\vboxoglcrutil.dll"#,
       r#"C:\Windows\system32\vboxoglerrorspu.dll"#,
       r#"C:\Windows\system32\vboxoglfeedbackspu.dll"#,
       r#"C:\Windows\system32\vboxoglpackspu.dll"#,
       r#"C:\Windows\system32\vboxoglpassthroughspu.dll"#,
       r#"C:\Windows\system32\vboxservice.exe"#,
       r#"C:\Windows\system32\vboxtray.exe"#,
       r#"C:\Windows\system32\VBoxControl.exe"#,
       r#"C:\Windows\system32\drivers\vmmouse.sys"#,
       r#"C:\Windows\system32\drivers\vmhgfs.sys"#,
       r#"C:\Windows\system32\drivers\vm3dmp.sys"#, 
       r#"C:\Windows\system32\drivers\vmhgfs.sys"#,
       r#"C:\Windows\system32\drivers\vmmemctl.sys"#,
       r#"C:\Windows\system32\drivers\vmmouse.sys"#,
       r#"C:\Windows\system32\drivers\vmrawdsk.sys"#,
       r#"C:\Windows\system32\drivers\vmusbmouse.sys"#,

    //  wtf is this -> VMCI.sys is the driver for the VMware Virtual Machine Communication Interface (VMCI). 
    //  It's responsible for communication between the host operating system and a virtual machine, 
    //  or between two or more virtual machines on the same host

    // So if you are testing with your development machine (vmware installed). This file artifact will contains in the main machine so i commented out it !
    // IF you did not installed vm's on you dev machine, then you can uncomment this !  

       // r#"C:\Windows\system32\drivers\vmci.sys"#, // sus //
    ];
    
    let file_system_artifacts_value = file_system_artifacts.iter().any(|&path| file_artifacts(path));

    //##=> Check running process !

    // Fastest Approach ever 0.3 secs

    let all_processes = get_running_processes();
    let target_processes = vec![
        "vboxservice.exe",
        "vboxtray.exe",
        "vmtoolsd.exe",
        "vmwaretray.exe",
        "vmwareuser.exe",
        "VGAuthService.exe",
        "vmacthlp.exe",
        "vmsrvc.exe",
        "vmusrvc.exe",
        "prl_cc.exe",
        "prl_tools.exe",
        "xenservice.exe",
        "qemu-ga.exe",
    ];

    let target_process_value = target_processes.iter()
        .any(|target_process| process_exists(&all_processes, target_process)); 

    

    //##==> Check Mac Address...!

    // let mac_address = get_mac_address();

    let mac_address = match get_mac_address(){
        Some(mac) => mac,
        None => return false,
    };

    let vm_mac_addresses = vec![
        vec![0x08, 0x00, 0x27], // VBOX
        vec![0x00, 0x05, 0x69], // VMWARE
        vec![0x00, 0x0C, 0x29], // VMWARE
        vec![0x00, 0x1C, 0x14], // VMWARE
        vec![0x00, 0x50, 0x56], // VMWARE
        vec![0x00, 0x1C, 0x42], // Parallels
        vec![0x00, 0x16, 0x3E], // Xen
        vec![0x0A, 0x00, 0x27], // Hybrid Analysis
    ]; 

    let mac_address_value = match find_matching_pattern(&mac_address, &vm_mac_addresses) {
        Some(_) => true,  
        None => false,   
    };

    //##==> Check CPU Instructions 

    let cpuid = CpuId::new();
    
    let vm_presence = cpuid.get_feature_info().map_or(false, |info| {
        info.has_hypervisor()
    });


    let vm_vendor = cpuid.get_vendor_info().map_or(false, |info| {
        info.as_str().contains("KVMKVMKVM")    || // KVM
        info.as_str().contains("Microsoft Hv") || // Microsoft Hyper-V or Windows Virtual PC
        info.as_str().contains("VMwareVMware") || // VMware
        info.as_str().contains("XenVMMXenVMM") || // Xen
        info.as_str().contains("prl hyperv")   || // Parallels
        info.as_str().contains("VBoxVBoxVBox")    // VirtualBox
    });

    let cpu_vendor_value = vm_presence || vm_vendor;

    //##=> WMI Quaries !! Soon ...!

    registry_keys_value_artifacts_value ||
    registry_keys_artifacts_value || 
    file_system_artifacts_value ||
    target_process_value || 
    mac_address_value || 
    cpu_vendor_value


}

fn registry_key_exists(key: &str) -> bool {
    let output = Command::new("reg")
        .args(&["query", &key])
        .output()
        .expect("Failed to execute reg query cmd");

    output.status.success()
}

// Program to check registry keys with artifacts ..!
fn registry_value_matches(key: &str, value_name: &str, expected_value: &str) -> bool {
    let output = Command::new("reg")
        .args(&["query", &key, "/v", value_name])
        .output()
        .expect("Failed to execute reg query cmd");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains(expected_value)
    } else {
        false
    }
}

// Progran to check if file exists ! 
fn file_artifacts(path: &str)-> bool{
    fs::metadata(path).is_ok()
}

// Programs to check for current running process !

fn get_running_processes() -> Vec<String>{
    let output = Command::new("wmic")
        .args(&["process","get","name"])
        .output()
        .expect("Failed to execute wmic cmd");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let processes: Vec<String> = output_str
        .lines()
        .skip(1)
        .map(|line| line.trim().to_lowercase())
        .collect();

    processes
}

fn process_exists(processes: &[String], target: &str) -> bool {
    processes.iter().any(|process| process.contains(target))
}

// Function to find mac addresses

fn get_mac_address() -> Option<Vec<u8>> {
    let output = Command::new("ipconfig")
        .args(&["/all"])
        .output()
        .expect("Failed to Exec ipconfing");

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        if line.contains("Physical Address") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let mac_address_str = parts[2].replace("-", ":");
                let mac_bytes: Vec<u8> = mac_address_str.split(":")
                    .map(|s| u8::from_str_radix(s, 16).unwrap_or_default())
                    .collect();
                return Some(mac_bytes);
            }
        }
    }
    None
}

fn find_matching_pattern<'a>(mac_address: &'a Vec<u8>, patterns: &'a Vec<Vec<u8>>) -> Option<&'a Vec<u8>> {
    for pattern in patterns {
        if mac_address.starts_with(pattern) {
            return Some(pattern);
        }
    }
    None
}


// Programs to find thr presence of Specific CPU Instructions !
// There is an create that will take care of it !!
// fn check_cpu_instruction(eax_value: u32) -> bool {
//     let eax_value_str = format!("{:#x}", eax_value);
//     let output = Command::new("cpuid")
//         .args(&["-l", &eax_value_str])
//         .output()
//         .expect("Failed to execute cpuid cmd");

//     let output_str = String::from_utf8_lossy(&output.stdout);
//     output_str.contains(&eax_value_str)
// }

// fn detect_vendor_string(vendor_string: &str) -> bool {
//     let output = Command::new("cpuid")
//         .args(&["-s", "0"])
//         .output()
//         .expect("Failed to execute cpuid cmd");

//     let output_str = String::from_utf8_lossy(&output.stdout);
//     output_str.contains(vendor_string)
// }

// Program to use WMI Quaries to retrieve sys info !

// System Firmware tables 
// Get Syetem frimwares => soon !
