// ==========================================================================//
//                                  BUILD.RS                                 //
// ==========================================================================//

// ======================= IMPORTS  ======================
use passwords::PasswordGenerator;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// ======================= FUNCTIONS ======================
// Generate a key for encryption in a .rs file that gets included in encryption.rs
fn generate_key() {
    if !Path::new("src/generated_key.rs").exists() {
        // AES Header for BLOB -> 8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0
        let mut blob: Vec<u8> = vec![8u8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0];

        // The key must be at least 25 characters long
        // A key can be declared manually here, otherwise, a 120-character one will be automatically generated
        let key: &str = "";

        if key == "" {
            let generator: PasswordGenerator = PasswordGenerator {
                length: 120,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: true,
                symbols: true,
                spaces: true,
                exclude_similar_characters: false,
                strict: true,
            };

            let generated_key = generator.generate_one().unwrap();
            let key_bytes: &[u8] = generated_key.as_bytes();
            blob.extend(key_bytes);
        } else {
            blob.extend(key.as_bytes());
        }

        // Write the key to the file
        let mut file = File::create("src/generated_key.rs").unwrap();
        writeln!(
            &mut file,
            "pub static GENERATED_KEY: [u8; {}] = {:?};",
            blob.len(),
            blob
        )
        .unwrap();
    }
}

// ======================= MAIN ======================
fn main() {
    // Generate the encryption key
    generate_key();

    // Add the icon to the generated executable
    let _res_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    embed_resource::compile("resources.rc");
}
