mod config_helper;
mod net_helper;
mod serial_helper;

use config_helper::Config;
use net_helper::Net;
use serial_helper::Serial;

use toml;
// use serde;
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::io::{self, Write};
use std::path;
// use std::{thread, time};

fn main() {
    let serial = Serial::new();
    let net = Net::new();
    let mut config = Config::default();

    println!("Initialize config");
    config.init_config();

    let ports = serial.get_avail_port_name();
    let net_ip = net.get_local_addr();

    let mut port_search_fail = 0;

    println!("Search for port");
    for p in ports.to_owned() {
        if config.get_port() == p {
            break;
        }
        port_search_fail += 1;
    }

    if port_search_fail == ports.len() {
        println!("Search port fail, port not found!");
        return;
    }

    println!("Connecting to client device");
    let client_port = config.get_port();
    let client_baud = config.get_baud();

    let mut target_port = serial.connection(client_port, client_baud);
    println!("Device connected");
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
