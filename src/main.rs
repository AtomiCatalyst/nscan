/* Valud Inputs
    nscan -h
    nscan -t [num_threads] [ip_addr]
 */
mod scan; 
mod args;
use std::{env, process, sync::mpsc::{channel, Receiver}, thread};
use ctrlc;

use crate::args::Arguments;

fn main() {

    ctrlc::set_handler(move || {
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let args: Vec<String> = env::args().collect();
    let program:String = args[0].clone();
    let arguments: Arguments = args::Arguments::new(&args).unwrap_or_else(
        |err: &str| {
            if err.contains("help"){
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_thrds: u16 = arguments.threads;
    let addr = arguments.ipaddr;
    let verb: bool = arguments.verbose;
    let(tx, rx) = channel();
    for i in 0..num_thrds {
        let tx = tx.clone();
        thread::spawn(move || {
            scan::scan(tx, i, addr, num_thrds, verb);
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
