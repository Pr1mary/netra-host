mod net_helper;
mod serial_helper;

use net_helper::Net;
use serial_helper::Serial;

use toml;
use std::env;
use std::fs;
use std::io::Read;
use std::io::{self, Write};
use std::path;
// use std::{thread, time};

fn main() {
    let serial = Serial::new();
    let net = Net::new();

    let ports = serial.get_avail_port_name();
    let net_ip = net.get_local_addr();

    println!("Serial Port List:");
    for p in ports {
        println!("{}", p);
    }

    // let mut str_in = String::new();
    let mut comser_in = String::new();
    // let baudrate_in = String::new();

    println!("Choose serial port:");
    io::stdin()
        .read_line(&mut comser_in)
        .expect("Failed to read command");
    // println!("Insert baud rate:");
    // io::stdin()
    //     .read_line(&mut baudrate_in)
    //     .expect("Failed to read command");

    let comser = comser_in.trim().to_uppercase().to_string();

    // let baudrate: u32 = match baudrate_in.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         panic!("Cannot convert baudrate to integer");
    //     }
    // };

    let baudrate = 19200;

    let mut target_port = serial.connection(comser, baudrate);

    loop {
        let mut str_read = String::new();
        target_port.read_to_string(&mut str_read);

        let read_in = str_read.trim();

        if read_in == "UP" {
            let str_name = "IP ".to_owned() + &net_ip;
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");
            target_port.flush().expect("Error flush");
        }

        if read_in == "STATUS" {
            let str_name = "OK".to_owned();
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");
            target_port.flush().expect("Error flush");
        }

        // println!("{}", read_in);
        // thread::sleep(time::Duration::from_millis(100));
    }
}
