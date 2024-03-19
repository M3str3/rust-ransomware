# Educational Ransomware in Rust

## Disclaimer

This ransomware project is created solely for educational purposes to explore and understand the complexities of ransomware mechanics and to learn Rust programming language intricacies. It is strictly forbidden to use this project or any part of its code for illegal activities. The creator assumes no responsibility for misuse or damages caused by this software.

## Introduction

This project is a personal journey into simulating the behavior of ransomware within a controlled and safe environment. It serves as a hands-on experiment for me to dive into file encryption and decryption, explore anti-reversing techniques, and understand how to interact with the Windows API, all while learning the Rust programming language. This initiative is driven by a curiosity about cybersecurity, malware analysis, and a desire to deepen my programming skills.

## Features

- **Encryption & Decryption**: Utilizes AES encryption to encrypt and decrypt files.
- **Anti-Reversing Techniques**: Implements basic methods to detect debugging and virtual machine environments.
- **Wallpaper Change**: A harmless demonstration of how ransomware might alter system settings to signal its presence.
- **Educational Ransom Note**: Includes a mock ransom note that emphasizes the project's educational purpose.
- **Build Script**: Features a `build.rs` script for generating encryption keys and preparing the build environment.

## Getting Started

1. **Clone the Repository**

```bash
git clone https://github.com/M3str3/rust-ransomware.git
cd rust-ransomware
```

2. **Compile the encrypter**

```bash
cargo build --release --features "ransomw"
```

3. **Compile the decryptor**

```bash
cargo build --release --features "decryptor"
```

4. **Enjoy**

The executables should be on `target/release/`