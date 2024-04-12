use std::{net::{IpAddr, TcpStream}, io::{self, Write}, sync::mpsc::Sender};

const MAX_PORT: u16 = 65535;

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_thrds: u16, verb: bool){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                if verb {
                    print!("---------------------------------------------Heard back from port: {port}\n")
                } else {
                    print!(".");
                }
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {
                if verb {
                    print!("Nothing back from port: {port}\n");
                    io::stdout().flush().unwrap();
                }
            }
        }
        if(MAX_PORT - port) < num_thrds {
            break;
        }
        port += num_thrds;
    }
}