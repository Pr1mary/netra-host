
mod net_helper;

use std::io::{self, Write};
use std::time::Duration;

use local_ip_address::local_ip;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    let net_ip = local_ip().unwrap();


    println!("Serial Port List:");
    for p in ports {
        println!("{}", p.port_name);
    }

    // let mut str_in = String::new();
    let mut comser_in = String::new();
    let mut baudrate_in = String::new();

    println!("Choose serial port:");
    io::stdin()
        .read_line(&mut comser_in)
        .expect("Failed to read command");
    println!("Insert baud rate:");
    io::stdin()
        .read_line(&mut baudrate_in)
        .expect("Failed to read command");

    let comser = comser_in.trim().to_uppercase().to_string();

    let baudrate: u32 = match baudrate_in.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Cannot convert baudrate to integer");
        }
    };

    let mut target_port = serialport::new(comser, baudrate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    loop {
        let mut str_read = String::new();
        target_port.read_to_string(&mut str_read);

        let read_in = str_read.trim();

        if read_in == "UP" {
            let str_name = "SEND ".to_owned() + &net_ip.to_string();
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");

            target_port.flush().expect("Error flush");
        }

        if read_in == "" {}
    }
}
