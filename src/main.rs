use sysinfo::System;
use sysinfo::Components;
use sysinfo::Networks;
use sysinfo::Disks;
use std::process::{Command, Child};
use std::env::consts::OS;
use std::thread::sleep;
use std::time::Duration;

/*
Based off:
- https://docs.rs/sysinfo/latest/sysinfo/
- https://doc.rust-lang.org/std/process/struct.Command.html
- https://doc.rust-lang.org/std/env/index.html
*/

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: jumpstarter <port>");
        std::process::exit(1);
    }

    let port_arg = args[1].clone();

    let mut sys = System::new_all();
    sys.refresh_all();

    let executable_path = match OS {
        "windows" => r"C:\Users\Bruker\Documents\GitHub\sanntidslab\target\debug\driver-rust.exe",
        "linux" => r"/home/student/Dokumenter/GitHub/sanntidslab/target/debug/driver-rust",
        _ => panic!("OS not supported")
    };

    let mut child: Option<Child> = None;
    
    loop {
        match &mut child {
            Some(c) => {
                match c.try_wait() {
                    Ok(Some(status)) => {
                        println!("driver-rust exited with {:?}", status);
                        child = None; // Process exited -> clear it and restart
                    },
                    Ok(None) => {
                        println!("driver-rust is running!");
                        sleep(Duration::from_secs(1));
                    },
                    Err(e) => {
                        eprintln!("Error when checking process: {:?}", e);
                        child = None; // Couldn't check on the process -> clear it and restart
                    }
                }
            },
            None => {
                println!("Starting driver-rust...");
                match Command::new(executable_path)
                    .arg(&port_arg)
                    .spawn() {
                        Ok(new_child) => {
                            println!("Process started!");
                            child = Some(new_child);
                        },
                        Err(e) => {
                            println!("Couldn't start process: {:?}, retrying...", e);
                            sleep(Duration::from_secs(2));
                        }
                    }
            }
        }

/*        let mut program_found = false;
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            //println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
            //println!("{:?}", process.name());
            if process.name() == "driver-rust" || process.name() == "driver-rust.exe" { // for windows "driver-rust.exe", fro linux "driver-rust"
                if process.status() != sysinfo::ProcessStatus::Zombie &&
                process.status() != sysinfo::ProcessStatus::Dead {
                    println!("It runs!");
                    program_found = true;
                }
            }
        }

        if program_found {
            println!("Program found");
            std::thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        println!("Will try to run.");

        let output = Command::new(executable_path)
            .arg(&port_arg)
            .spawn()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });
        
        std::thread::sleep(std::time::Duration::from_secs(2)); */
    }
}