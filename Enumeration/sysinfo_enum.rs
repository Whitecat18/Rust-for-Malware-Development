/*
    This is an POC that you can use sysinfo to gather 
        * System information
        * Get Process PID's // Kill Processes.
        * Enumerate Running Processess
        * Get Hardware Information such as CPU , E_NET Interfaces, etc..
        * Get User Information.

    For Code : https://github.com/Whitecat18/Rust-for-Malware-Development
*/

const POS: &str = "[+]";
const NEG: &str = "-";

// extern crate sysinfo;

extern crate sysinfo;

#[allow(unused_imports)]
// use sysinfo::{Disk, System};
use sysinfo::{
    Components, Disks, Networks, System, Users,CpuRefreshKind,RefreshKind,Pid
};

use std::{any::Any, process::Command};

fn main(){
    let mut system = sysinfo::System::new_all();

    system.refresh_all();

    println!("{} System Information", POS);
    println!("{} OS: {} {}", NEG ,System::name().unwrap_or("Unable to find system name".to_string()), 
        System::os_version().unwrap_or("Unknown".to_string()));
    // println!("{} Kernel Version: {}", NEG,System::kernel_version().unwrap_or("Unable to Find".to_string())); // This methods it for Linux!
    println!("{} Host name : {}", NEG, System::host_name().unwrap_or("Unable to find Host Name".to_string()));
    println!();

    println!("{} CPU Information:",POS);
    let s =  System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );

    println!("{} CPU Usage : {}%", NEG,s.global_cpu_info().cpu_usage());

    let cpu_info = system.global_cpu_info();
    println!("{} Model : {:?}", NEG, cpu_info.name());
    println!("{} Usage : {}", NEG ,cpu_info.cpu_usage());
    let cpu_cores = system.physical_core_count();
    if cpu_cores.is_some(){
        println!("{} Cores: {}", NEG, cpu_cores.unwrap());
        // println!("{} Total Cores: {}", NEG, )
    } else{
        println!("{} Unable to Retrieve CPU cores !",NEG);
    }
    println!();

    println!("{} Disk Information",POS);
    let disk_info = Disks::new_with_refreshed_list();
    for d in &disk_info{
        // println!("{:?}",disk);
        // println!("Name: {:?}",d.mount_point());
        print!("{} Name: {}",NEG, d.name().to_string_lossy());
        print!(" FS: {:?}",d.file_system().to_string_lossy());
        // print!(" Type: {:?}",d.type_id());
        print!(" Removeable Device: {}",d.is_removable());
        print!(" Mount Point: {:?}" ,d.mount_point());
        println!(" Disk Space : {:.2} GB / {:.2} GB ", 
            d.available_space() as f64 / (1024.0 * 1024.0 * 1024.0) ,
            d.total_space() as f64 / (1024.0 * 1024.0 * 1024.0));

        // d.type_id(), d.is_removable(), d.mount_point()
    }
    println!();

    println!();
    println!("{} User Information",POS);
    let user = Users::new_with_refreshed_list();
    for u in user.list(){
        // println!("{:?}",u);
        // println!("{:?}",u.id());
        println!("{} Users: {} : {:?}", NEG, u.name(), u.type_id());
        // let sid = u.id();
        // println!("{:?}",sid);
    
        // Tried to access the sid that was inside the PID , but i failed !

        // println!("{:?}", sid.sid.iter().map(|x| format!("{:02}", x)).collect::<Vec<_>>().join("-"));
        // println!("{} ")
    }
    // To view the full process
    // for (pid, process) in system.processes() {
    //     println!("{} [{}] {} {:?}", NEG,pid ,process.name(), process.disk_usage());
    // }

    println!("{} Network Adapters",POS);
    let net_lans = Networks::new_with_refreshed_list();

    for (inter_name, _) in &net_lans{
        println!("{} {}B", NEG,inter_name);
    }
    
    // Find Particualr Process iD's 
    println!();
    println!("{} Custom PID's",POS);
    let check_process_lists = ["explorer.exe","winlogon.exe","wininit.exe"];
    for process in check_process_lists{
        let pid = system
            .processes_by_name(process)
            .next()
            .expect("[-] No such Process")
            .pid()
            .as_u32();

        println!("{} PID: {} : {}",NEG,process,pid);
    }
    
    // Printing ARPS!
    println!("{} ARPS!",POS);
    let arp = Command::new("powershell.exe")
    .args(&["arp","-a"])
    .output()
    .expect("Failed to Receive the Information");

    if arp.status.success(){
        println!("{} Inferface IPs!: {}", NEG,String::from_utf8_lossy(&arp.stdout));
    }else {
        println!("Command not found Exit Status! {}",arp.status.code().unwrap());
    }

    
    // Kill Process !
    println!();
    println!("{} PID Termination",POS);
    let s = System::new_all();
    let pid_demo = system
        .processes_by_name("notepad.exe").next().expect("[-] No Such Process").pid();

    if let Some(pos) = s.process(Pid::from(pid_demo)){
        pos.kill();
        println!("{} PID {} Terminated Successfully",NEG,pid_demo);
    } else {
        println!("{} PID Not Found !",NEG);
    }
    
}


