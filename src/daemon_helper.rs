extern crate daemonize;

use daemonize::{Daemonize, Error};
use std::fs::File;

pub struct Daemon {
    workdir: String,
}

impl Daemon {
    pub fn new() -> Daemon {
        return Daemon {
            workdir: "/tmp".to_owned(),
        };
    }

    pub fn daemonize(&self, proc: &dyn Fn()) -> Result<String, Error> {
        let stdout = File::create(self.workdir.to_owned() + "/netrad.out").unwrap();
        let stderr = File::create(self.workdir.to_owned() + "/netrad.err").unwrap();

        let daemon = Daemonize::new()
            .pid_file(self.workdir.to_owned() + "/netrad.pid")
            .chown_pid_file(true)
            .working_directory(self.workdir.to_owned())
            // .user("rafli")
            // .group("daemon")
            // .group(2)
            .umask(0o777)
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        match daemon.start() {
            Ok(_) => {
                proc();
                return Ok("Success".to_owned());
            }
            Err(e) => return Err(e),
        }
    }
}
