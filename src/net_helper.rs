pub mod net {

    use local_ip_address::list_afinet_netifas;
    use std::io::{self, Write};
    use local_ip_address::local_ip;

    pub fn getAllAddr() {
        let ports = serialport::available_ports().expect("No ports found!");
        let net_intf = list_afinet_netifas().unwrap();

        let mut conn_name: String = String::from("");
        let mut conn_ip: String = String::from("");

        println!("IP List:");
        for (name, ip) in net_intf.iter() {
            println!("{}:\t{:?}", name, ip);
            conn_name = name.to_string();
            conn_ip = ip.to_string();
        }

        println!("Serial Port List:");
        for p in ports {
            println!("{}", p.port_name);
        }
    }

    pub fn getLocalAddr(){
        let net_ip = local_ip().unwrap();
    }
}
