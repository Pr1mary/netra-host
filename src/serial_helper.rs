use std::io::stdout;
use std::io::{self, Write};
use std::str;
use std::time::Duration;

use serialport::SerialPort;

mod serial_process {
    pub struct serstruct {}

    impl serproc for serstruct {
        fn connection(portname: String, baudrate: u32) -> Box<dyn SerialPort> {
            let open_port = serialport::new(portname, baudrate)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open port");
            return open_port;
        }
    }
}
