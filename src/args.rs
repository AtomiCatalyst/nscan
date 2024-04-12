use std::{net::IpAddr, str::FromStr};
pub struct Arguments {
    pub ipaddr: IpAddr,
    pub threads: u16,
    pub verbose: bool
}

const TNUM_DEF: u16 = 4;



impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments")
        } else if args.len() > 9 {
            return Err("Too many arguments")
        }
        let mut i = 0;
        let mut ipaddr: IpAddr = IpAddr::from_str("0.0.0.0").unwrap();
        let mut threads: u16 = TNUM_DEF;
        let mut verbose: bool = false;
        let mut debug: bool = false;
        while i < args.len() {
            match args[i].as_str() {
                "-h" => {
                    println!("Usage: nscan [-t num_threads] [ip_addr]");
                    return Err("help")
                },
                "-t" => {
                    threads = match args[i+1].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("Failed to parse thread number")
                    };
                },
                "-v" => {
                    verbose = true;
                }, 
                "-a" => {
                    ipaddr = match IpAddr::from_str(&args[i+1]) {
                        Ok(s) => s,
                        Err(_) => return Err("Not a valid IP address")
                    };
                },
                "-d" => {
                    println!("Debug mode enabled\n");
                    debug = true;
                }

                _ => {}

            }
            i += 1;
        }

        if debug {
            println!("--Current Set Values--\nIP Address: {ipaddr}\nNumber of Threads: {threads}\nVerbose Mode: {verbose}\n")
        }

        if ipaddr == IpAddr::from_str("0.0.0.0").unwrap(){
            return Err("No IP address supplied")
        } else {
            return Ok(Arguments{threads, ipaddr, verbose});
        }

        



        /* 
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
        } */
        }
    }

