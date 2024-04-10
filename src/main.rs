/* Valud Inputs
    nscan -h
    nscan -t [num_threads] [ip_addr]
 */

use std::{env, net::{IpAddr, TcpStream}, io::{self, Write}, str::FromStr, process, sync::mpsc::{Sender, channel}, thread};

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

const TNUM_DEF: u16 = 4;
const MAX_PORT: u16 = 65535;

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments")
        } else if args.len() > 4 {
            return Err("Too many arguments")
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: TNUM_DEF })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: nscan [-t num_threads] [ip_addr]");
                Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too mant arguments")
            } else if flag.contains("-t") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP address")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("Invalid syntax");
            }
        }
        }
    }

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_thrds: u16){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if(MAX_PORT - port) < num_thrds {
            break;
        }
        port += num_thrds;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_thrds = arguments.threads;
    let addr = arguments.ipaddr;
    let(tx, rx) = channel();
    for i in 0..num_thrds {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_thrds);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
