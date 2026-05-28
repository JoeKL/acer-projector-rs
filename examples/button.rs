use acer_projector_rs::Projector;
use acer_projector_rs::commands::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example button -- <port_name>");
        eprintln!("Example: cargo run --example button -- /dev/ttyUSB0");
        process::exit(1);
    }

    let port_name = &args[1];
    let baud_rate = 9600;
    let timeout_ms = 500;

    println!(
        "Attempting to connect to projector on {} at {} baud...",
        port_name, baud_rate
    );

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

    println!("Sending command...");
    match projector.send_command(Command::Press(acer_projector_rs::enums::Key::Menu)) {
        Ok(_) => println!("Command sent successfully!"),
        Err(e) => eprintln!("Failed to send command: {:?}", e),
    } 

}
