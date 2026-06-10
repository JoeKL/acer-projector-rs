use acer_projector_rs::Projector;
use acer_projector_rs::enums::Query;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example queries -- <port_name>");
        eprintln!("Example: cargo run --example queries -- /dev/ttyUSB0");
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

    match projector.is_powered_on() {
        Ok(true) => println!("Projector is powered on!"),
        Ok(false) => println!("Projector is not powered on."),
        Err(e) => eprintln!("Failed to check powerstate: {:?}", e),
    }

    match projector.send_query(Query::ModelName) {
        Ok(model_name) => println!("Projector model name: {}", model_name),
        Err(e) => eprintln!("Failed to query model name: {:?}", e),
    }

    match projector.send_query(Query::Lamp1Hours) {
        Ok(lamp_hours) => println!("Lamp 1 hours: {}", lamp_hours),
        Err(e) => eprintln!("Failed to query lamp hours: {:?}", e),
    }
}
