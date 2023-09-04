mod config_helper;
mod net_helper;
mod serial_helper;

use config_helper::Config;
use net_helper::Net;
use serial_helper::Serial;

fn main() {
    let serial = Serial::new();
    let net = Net::new();
    let mut config = Config::default();

    println!("Initialize config");
    config.init_config().expect("config init fail");

    let client_port = config.get_port();
    let client_baud = config.get_baud();

    let ports = serial.get_avail_port_name();
    let net_ip = net.get_local_addr();

    let mut port_search_fail = 0;

    println!("Search for port");
    for p in ports.to_owned() {
        if client_port == p {
            break;
        }
        port_search_fail += 1;
    }

    if port_search_fail == ports.len() {
        panic!("Search port fail: \"{} not found\"", client_port);
    }

    println!("Connecting to client device");
    let mut target_port = serial.connection(client_port, client_baud);
    println!("Device connected");
    loop {
        let mut str_read = String::new();
        let mut _err = target_port.read_to_string(&mut str_read).unwrap_err();

        let read_in = str_read.trim();

        if read_in == "WHEREIP" {
            let str_name = "IP ".to_owned() + &net_ip;
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");
            target_port.flush().expect("Error flush");
        }

        if read_in == "MOTD" {
            let str_name = "MOTD ".to_owned()+"This is the best day of my life\n";
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");
            target_port.flush().expect("Error flush");
        }

        if read_in == "STATUS" {
            let str_name = "ALIVE".to_owned();
            target_port
                .write(str_name.trim().as_bytes())
                .expect("Write failed!");
            target_port.flush().expect("Error flush");
        }
    }
}
