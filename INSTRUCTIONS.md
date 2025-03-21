# Raspberry Pi Zero U2F Key Implementation Guide

This guide explains how to use this repository to turn a Raspberry Pi Zero (W/2W) into a physical U2F security key using USB gadget mode.

## Overview

This project adapts the rust-u2f software-based U2F implementation to run on a Raspberry Pi Zero, making it function as a hardware U2F key that can be plugged into any computer.

### How It Works

1. The Pi Zero is configured in USB gadget mode to appear as a U2F HID device
2. We use the U2F protocol implementation from rust-u2f
3. Instead of creating a virtual device with UHID, we write directly to the USB gadget HID interface

## Hardware Requirements

- Raspberry Pi Zero, Zero W, or Zero 2 W
- MicroSD card (8GB+ recommended)
- USB data cable for connecting to host computer
- Optional: Case for the Pi Zero

## Setup Instructions

### 1. Prepare the Raspberry Pi

1. Flash Raspberry Pi OS Lite to a microSD card
2. Set up headless access (SSH) if needed
3. Boot the Pi Zero and connect via SSH

### 2. Install Dependencies

```bash
sudo apt update
sudo apt install -y git curl build-essential pkg-config libssl-dev
```

### 3. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 4. Configure USB Gadget Mode

1. Enable USB OTG mode by editing `/boot/config.txt`:

```bash
sudo nano /boot/config.txt
```

2. Add this line at the end:

```
dtoverlay=dwc2
```

3. Enable the dwc2 module by editing `/etc/modules`:

```bash
sudo nano /etc/modules
```

4. Add this line at the end:

```
dwc2
```

5. Save the `fido_gadget_setup.sh` script from this repository to `/usr/local/bin/`:

```bash
sudo cp fido_gadget_setup.sh /usr/local/bin/
sudo chmod +x /usr/local/bin/fido_gadget_setup.sh
```

6. Create a systemd service to run it at boot:

```bash
sudo nano /etc/systemd/system/fido-gadget.service
```

7. Add the following content:

```
[Unit]
Description=FIDO U2F USB Gadget Setup
After=network.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/fido_gadget_setup.sh
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
```

8. Enable and start the service:

```bash
sudo systemctl enable fido-gadget
sudo systemctl start fido-gadget
```

### 5. Clone and Modify the rust-u2f Repository

1. Clone this repository:

```bash
git clone https://github.com/yourusername/rust-u2f.git
cd rust-u2f
```

2. Create the HID device adapter for writing to `/dev/hidg0` (implementation details below)

3. Compile the modified U2F daemon:

```bash
cargo build --release
```

4. Create a systemd service to run the U2F daemon:

```bash
sudo nano /etc/systemd/system/u2f-daemon.service
```

5. Add the following content:

```
[Unit]
Description=U2F Security Key Daemon
After=fido-gadget.service

[Service]
ExecStart=/home/pi/rust-u2f/target/release/pi-u2f-daemon
Restart=always
User=root

[Install]
WantedBy=multi-user.target
```

6. Enable and start the service:

```bash
sudo systemctl enable u2f-daemon
sudo systemctl start u2f-daemon
```

## Code Modifications Required

To adapt the rust-u2f codebase for the Pi Zero, we need to make the following changes:

### 1. Create a New HID Device Implementation

Create a new file at `linux/pi-hid/src/lib.rs` that implements a character device interface for `/dev/hidg0`:

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct PiHidDevice {
    file: File,
}

impl PiHidDevice {
    pub fn open() -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/hidg0")?;
            
        Ok(Self { file })
    }
    
    pub fn write_report(&mut self, data: &[u8]) -> io::Result<usize> {
        self.file.write(data)
    }
    
    pub fn read_report(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}
```

### 2. Create a U2FHID Adapter for the Pi

Create a new file to adapt the U2FHID protocol to work with the Pi HID device:

```rust
use pi_hid::PiHidDevice;
use u2fhid_protocol::{Packet, Response, Server};

pub struct PiU2fAdapter {
    device: PiHidDevice,
    server: Server,
}

impl PiU2fAdapter {
    pub fn new(device: PiHidDevice, server: Server) -> Self {
        Self { device, server }
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        let mut input_buf = [0u8; 64];
        
        loop {
            // Read HID report from the host
            let read_size = self.device.read_report(&mut input_buf)?;
            
            if read_size > 0 {
                // Process the incoming packet
                if let Ok(packet) = Packet::from_bytes(&input_buf) {
                    // Let the U2F server process the packet
                    let responses = self.server.process_packet(packet);
                    
                    // Send any response packets back to the host
                    for response_packet in responses {
                        let response_bytes = response_packet.to_bytes();
                        self.device.write_report(&response_bytes)?;
                    }
                }
            }
        }
    }
}
```

### 3. Create a Pi U2F Daemon

Create a new binary at `linux/pi-u2f-daemon/src/main.rs`:

```rust
use pi_hid::PiHidDevice;
use u2f_core::{self, SecretStore, U2fService};
use u2fhid_protocol::Server;

fn main() -> std::io::Result<()> {
    // Initialize the U2F service with appropriate storage and crypto
    let secret_store = // ... initialize your secret store
    let crypto_ops = // ... initialize your crypto operations
    let user_presence = // ... initialize your user presence check
    
    let u2f_service = U2fService::new(secret_store, crypto_ops, user_presence);
    
    // Create the U2FHID protocol server
    let server = Server::new(u2f_service);
    
    // Open the HID device
    let hid_device = PiHidDevice::open()?;
    
    // Create and run the adapter
    let mut adapter = PiU2fAdapter::new(hid_device, server);
    adapter.run()
}
```

## Testing

1. Plug the Pi Zero into a computer using the USB port (not the power port)
2. The computer should recognize a new U2F security key
3. Test registration at https://webauthn.io or https://demo.yubico.com/webauthn-technical/registration

## Troubleshooting

- **USB device not detected**: Check that the gadget mode is properly configured
- **USB device detected but not working as U2F**: Verify the HID descriptor and report sizes
- **Registration/Authentication failures**: Check the application logs for errors

## Security Considerations

This implementation provides similar phishing protection as hardware U2F keys but does not provide the same level of hardware-backed security. The private keys are stored on the Pi's filesystem and could potentially be extracted if an attacker gains physical access to the device or compromises it remotely.

For production use, consider additional security measures:
- Use encrypted storage for keys
- Set up the Pi with minimal attack surface
- Consider a read-only filesystem for improved security

## Further Improvements

- Add a physical button for user presence verification
- Add LED indication for authentication/registration events
- Implement FIDO2 functionality for passwordless authentication
- Add additional attestation options