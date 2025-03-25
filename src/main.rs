use sysinfo::System;
use sysinfo::Components;
use sysinfo::Networks;
use sysinfo::Disks;
use std::os::windows;
use std::process::Command;
use std::env::consts::OS;

/*
Based of:

- https://docs.rs/sysinfo/latest/sysinfo/
- https://doc.rust-lang.org/std/process/struct.Command.html
- https://doc.rust-lang.org/std/env/index.html
*/

fn main() {

    let mut sys = System::new_all();
    sys.refresh_all();

    let executable_path = match OS {
        windows => r"C:\Users\Bruker\Documents\GitHub\sanntidslab\target\debug\driver-rust.exe",
        linux => r"C:\Users\Bruker\Documents\GitHub\sanntidslab\target\debug\driver-rust",
        _ => panic!("OS not supported")
    }
    ;
    loop {
        for (pid, process) in sys.processes() {
            //println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
            //println!("{:?}", process.name());
            if process.name() == "driver-rust.exe" {
                println!("It runs!");
            }
            if process.name() != "driver-rust.exe" {
                println!("Will try to run.");
                let output = Command::new(executable_path).output().unwrap_or_else(|e| {
                    panic!("failed to execute process: {}", e)
                });
                if output.status.success() {
                        let s = String::from_utf8_lossy(&output.stdout);
                        print!("curl succeeded and stdout was:\n{}", s);
                } else {
                        let s = String::from_utf8_lossy(&output.stderr);
                        print!("curl failed and stderr was:\n{}", s);
                }
                }
        }
    }
}