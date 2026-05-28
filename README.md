# acer-projector-rs

A modular, type-safe, and stateless RS232 driver library written in Rust for Acer DLP projectors. 

This crate wraps Acer's universal serial protocol into clean, idiomatic Rust enums, eliminating the need to deal with raw ASCII or Hex strings when automating your home theater or commercial setup.

## Features

* **100% Type-Safe:** Leverage Rust's type system with nested enums (e.g., `Command::SetSource(Source::Hdmi1)`) to guarantee you never send an invalid instruction.
* **Stateless & Reliable:** Queries the hardware directly to verify configuration, preventing out-of-sync "ghost states".
* **Robust Error Handling:** Built on top of `thiserror` to safely bubble up OS-level serial timeouts or hardware connection drops.
* **Universal Protocol Support:** Implements all 70+ control and query actions defined in standard Acer DLP protocol sheets.

## Hardware Requirements

To connect your computer to the projector, you will typically need:
1. A **USB-to-RS232 (DB9) adapter cable** (e.g., using an FTDI or CH340 chipset).
2. A straight-through or null-modem serial cable depending on your specific model's port layout (the Acer P1510 uses a standard DB9 female configuration).

### Default Serial Configurations:
* **Baud Rate:** 9600
* **Data Bits:** 8
* **Parity:** None
* **Stop Bits:** 1

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
acer-projector = "0.1.0"
```

    let timeout = 500;
## Quick Start

Here is how simple it is to initialize a connection and power on your projector:

```rust
use acer_projector::{Projector, Command, enums::{Source, Key}};
use std::time::Duration;

fn main() -> Result<(), acer_projector::error::ProjectorError> {
    // Port names: "COM3" on Windows, "/dev/ttyUSB0" on Linux
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let timeout = 500;

    println!("Connecting to projector on {}...", port_name);
    let mut projector = Projector::connect(port_name, baud_rate, timeout)?;

    // Power on the projector using convenience methods
    projector.power_on()?;
    println!("Power command sent!");

    // Switch to HDMI 1 using the modular type system
    projector.send_command(Command::SetSource(Source::Hdmi1))?;

    // Query hardware
    match projector.is_powered_on() {
        Ok(true) => println!("Hardware status confirmed: Lamp is burning."),
        Ok(false) => println!("Hardware status confirmed: Projector is in Standby."),
        Err(e) => eprintln!("Failed to fetch hardware status: {:?}", e),
    }

    Ok(())
}
```

## Operating System Notes

### Linux / macOS
You usually need permission to read and write to serial ports. You can grant your user access by adding them to the `dialout` or `tty` group:

```bash
sudo usermod -a -G dialout $USER
```
*(Note: You will need to log out and back in for these changes to take effect).*

### Windows
Ensure that no other terminal programs (like PuTTY or Serial Monitor) are holding the target `COM` port open, or the driver will return an `Access Denied` error.

## Contribution & Testing

This library was developed and verified against an **Acer P1510**. Because Acer uses a highly standardized universal instruction set across its DLP lineup, it should work out-of-the-box for dozens of models (such as the H7850, P5230, X1527i, etc.).

If you test this crate on a different Acer projector model, please open an Issue or pull request to help us build a comprehensive "Verified Models" list in this README!

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
