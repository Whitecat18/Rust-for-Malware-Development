
const EDR_LIST: [&str; 125] = [
    "activeconsole", 
    "ADA-PreCheck", 
    "ahnlab", 
    "amsi.dll", 
    "anti malware", 
    "anti-malware", 
    "antimalware",
    "anti virus",
    "anti-virus",
    "antivirus",
    "appsense",
    "attivo networks",
    "attivonetworks",
    "authtap",
    "avast",
    "avecto",
    "bitdefender",
    "blackberry",
    "canary",
    "carbonblack",
    "carbon black",
    "cb.exe",
    "check point",
    "ciscoamp",
    "cisco amp",
    "countercept",
    "countertack",
    "cramtray",
    "crssvc",
    "crowdstrike",
    "csagent",
    "csfalcon",
    "csshell",
    "cybereason",
    "cyclorama",
    "cylance",
    "cynet",
    "cyoptics",
    "cyupdate",
    "cyvera",
    "cyserver",
    "cytray",
    "darktrace",
    "deep instinct",
    "defendpoint",
    "defender",
    "eectrl",
    "elastic",
    "endgame",
    "f-secure",
    "forcepoint",
    "fortinet",
    "fireeye",
    "groundling",
    "GRRservic",
    "harfanglab",
    "inspector",
    "ivanti",
    "juniper networks",
    "kaspersky",
    "lacuna",
    "logrhythm",
    "malware",
    "malwarebytes",
    "mandiant",
    "mcafee",
    "morphisec",
    "msascuil",
    "msmpeng",
    "nissrv",
    "omni",
    "omniagent",
    "osquery",
    "Palo Alto Networks",
    "pgeposervice",
    "pgsystemtray",
    "privilegeguard",
    "procwall",
    "protectorservic",
    "qianxin",
    "qradar",
    "qualys",
    "rapid7",
    "redcloak",
    "red canary",
    "SanerNow",
    "sangfor",
    "secureworks",
    "securityhealthservice",
    "semlaunchsv",
    "sentinel",
    "sentinelone",
    "sepliveupdat",
    "sisidsservice",
    "sisipsservice",
    "sisipsutil",
    "smc.exe",
    "smcgui",
    "snac64",
    "somma",
    "sophos",
    "splunk",
    "srtsp",
    "symantec",
    "symcorpu",
    "symefasi",
    "sysinternal",
    "sysmon",
    "tanium",
    "tda.exe",
    "tdawork",
    "tehtris",
    "threat",
    "trellix",
    "tpython",
    "trend micro",
    "uptycs",
    "vectra",
    "watchguard",
    "wincollect",
    "windowssensor",
    "wireshark",
    "withsecure",
    "xagt.exe",
    "xagtnotif.exe"
];

use std::process::Command;
use std::fs;
use regex::Regex;

fn check_edr() -> Result<(), Box<dyn std::error::Error>> {
    let edr_regex = Regex::new(&format!("(?i)({})", EDR_LIST.join("|")))?;

    let output = Command::new("wmic").args(&["process", "get", "name"]).output()?;
    if let Ok(processes) = String::from_utf8(output.stdout) {
        for line in processes.lines() {
            if edr_regex.is_match(line) {
                println!("[-] Suspicious process found: {}", line);
            }
        }
    }
    
    let output = Command::new("wmic").args(&["service", "get", "name"]).output()?;
    if let Ok(services) = String::from_utf8(output.stdout) {
        for line in services.lines() {
            if edr_regex.is_match(line) {
                println!("[-] Suspicious service found: {}", line);
            }
        }
    }

    for dir in &["C:\\Program Files", "C:\\Program Files (x86)", "C:\\ProgramData"] {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if edr_regex.is_match(name) {
                            println!("[-] Suspicious file found in {}: {}", dir, name);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    check_edr()?;
    Ok(())
}