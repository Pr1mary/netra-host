use local_ip_address::list_afinet_netifas;
use local_ip_address::local_ip;

pub struct Net{}

impl Net {
    pub fn new() -> Net{
        return Net{};
    }

    pub fn get_local_addr(&self) -> String {
        let local = local_ip().unwrap();
        return local.to_string();
    }

    pub fn get_all_addr(&self) {
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
}
