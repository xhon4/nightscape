Nightscape

Crates.io

A terminal animation of a night sky, featuring a moon, stars, and random events.
Installation
Quick Option (Recommended)

cargo install nightscape

Other Options
Option 1: Using Cargo (Recommended, Cross-platform)

cargo install nightscape

    If it hasn't been published yet, you can install it directly from GitHub:

cargo install --git https://github.com/xhon4/nightscape.git

Option 2: Download Precompiled Binaries

Go to the Releases section and download the binary for your operating system.
Option 3: Build Manually
1. Install Rust

    Linux/macOS:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    Windows:
    Download and install from rustup.rs.

2. Clone the Repository

git clone https://github.com/xhon4/nightscape.git
cd nightscape

3. Build and Run

cargo run

Or to build the release binary:

cargo build --release
./target/release/nightscape

Compatibility

Works on Linux, macOS, and Windows (in terminals that support Unicode and ANSI).
License

MIT
