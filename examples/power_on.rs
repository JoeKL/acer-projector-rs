use acer_projector_rs::Projector;
use std::env;
use std::process;

fn main() {
    // 1. Grab the port name from the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example power_on -- <port_name>");
        eprintln!("Example: cargo run --example power_on -- /dev/ttyUSB0");
        process::exit(1);
    }

    let port_name = &args[1];
    let baud_rate = 9600;
    let timeout_ms = 500;

    println!(
        "Attempting to connect to projector on {} at {} baud...",
        port_name, baud_rate
    );

    // 2. Connect to the projector
    let mut projector = match Projector::connect(port_name, baud_rate, timeout_ms) {
        Ok(p) => {
            println!("Successfully opened serial port: {}", port_name);
            p
        }
        Err(e) => {
            eprintln!("Failed to connect: {:?}", e);
            process::exit(1);
        }
    };

    // 3. Send the power on command
    println!("Sending Power On command...");
    match projector.power_on() {
        Ok(_) => println!("Power On command sent successfully!"),
        Err(e) => eprintln!("Failed to send command: {:?}", e),
    }
}
