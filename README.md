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

## Usage

Here is a simple project that uses the library to emulate the remote via RS232 (which definitely did not get lost which led me to write this whole library in the first place): https://github.com/JoeKL/acer_serial_remote

## Contribution & Testing

This library was developed and verified against an **Acer P1510**. Because Acer uses a highly standardized universal instruction set across its DLP lineup, it should work out-of-the-box for dozens of models (such as the H7850, P5230, X1527i, etc.).

## License
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
