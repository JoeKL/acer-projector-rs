# acer-projector-rs

A simple RS232 driver library written in Rust for Acer DLP projectors. 

## Hardware Requirements

To connect your computer to the projector, you will typically need:
1. A **USB-to-RS232 (DB9) adapter cable** (e.g., using an FTDI or CH340 chipset).
2. A straight-through or null-modem serial cable depending on your specific model's port layout (the Acer P1510 uses a standard DB9 female configuration).

### Default Serial Configurations:
* **Baud Rate:** 9600
* **Data Bits:** 8
* **Parity:** None
* **Stop Bits:** 1

## Quick Start

Here is how simple it is to initialize a connection and power on your projector:

```rust
use acer_projector::{Projector, Command, enums::{Source, Key}};
use std::time::Duration;

fn main() -> Result<(), acer_projector::error::ProjectorError> {
    // Port names: "/dev/ttyUSB0" on Linux
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let timeout_ms = 500;

    println!("Connecting to projector on {}...", port_name);
    let mut projector = Projector::connect(port_name, baud_rate, timeout_ms)?;

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

## Contribution & Testing

This library was developed and verified against an **Acer P1510**. Because Acer uses a highly standardized universal instruction set across its DLP lineup, it should work out-of-the-box for dozens of models (such as the H7850, P5230, X1527i, etc.).

## License
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
