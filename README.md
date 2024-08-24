<div align="center">
  <img width="260px" src="https://github.com/Whitecat18/Rust-for-Malware-Development/assets/96696929/08dcf469-a502-450c-ab94-3915fd4b9968" />

  <a href = "https://github.com/Whitecat18/Rust-for-Malware-Development.git" ><h3>Rust for Malware Development</h3></a>

  <b>This repository contains source codes of various techniques used by real-world malware authors, red teamers, threat actors, state-sponsored hacking groups etc. These techniques are well-researched and implemented in Rust.</b>
  <br>
  <br>
    Repository managed by <a href="https://twitter.com/5mukx"> @5mukx</a></i></p>
  <br>
  
  <img src="https://img.shields.io/badge/Language-Rust-orange">
    <img src="https://img.shields.io/badge/OS-Windows%20%26%20Linux-blue">
  <img src="https://img.shields.io/badge/Maintained-Yes-Green">

-----------------

</div>

> Note: These are my own research and implementations, derived from the original authors' work. If you discover any errors in these codes, please [contact](https://x.com/5mukx) or contribute to this repository.

## Basics 

To Learn Rust -> [Rust Book](https://doc.rust-lang.org/book/)

Windows API [old]-(winapi)-> [WinAPI](https://docs.rs/winapi/latest/winapi/)

Windows API (by Official Microsoft) -> [WinAPI](https://docs.rs/crate/windows/latest) 

ntapi Crate -> [NtAPI](https://docs.rs/ntapi/latest/ntapi/)

Windows Internels -> [Link](https://learn.microsoft.com/en-us/sysinternals/resources/windows-internals)

RedTeam Notes -> [Link](https://www.ired.team/)

## Manifest dependencies for [winapi](https://docs.rs/winapi/latest/winapi/) to test and execute

**Copy the dependencics in Cargo.toml file**

```
[dependencies]
winapi = { version = "0.3.9", features = ["winuser","setupapi","dbghelp","wlanapi","winnls","fileapi","sysinfoapi", "fibersapi","debugapi","winerror", "wininet" , "winhttp" ,"synchapi","securitybaseapi","wincrypt","psapi", "tlhelp32", "heapapi","shellapi", "memoryapi", "processthreadsapi", "errhandlingapi", "winbase", "handleapi", "synchapi"] }

ntapi = "0.4.1"
user32-sys = "0.2.0"
```

> Tips for Rust Beginners: Copy and save the dependencies in Cargo.toml File. Versions may be different. Just copy the features when testing. 


## Rust Malware Blogs regarding this Repostitory

* [Malware Development Essentials Part 1](https://medium.com/system-weakness/malware-development-essentials-part-1-5f4626652ed9)

* [Rust for CyberSecurity and Red Teaming](https://infosecwriteups.com/rust-for-cyber-security-and-red-teaming-275595d3fdec)

* [DLL Injection using Rust](https://smukx.medium.com/dll-injection-using-rust-593b83734c90)

