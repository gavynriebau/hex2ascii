
extern crate hex;
extern crate clap;

use std::io;
use std::io::{Write, BufRead};
use clap::{App, Arg};

fn main() {

    let matches = App::new("hex2ascii")
        .author("Gavyn Riebau")
        .version("0.1.0")
        .about("Converts hex values to ascii")
        .arg(
            Arg::with_name("verbose")
                .help("Include verbose output including warning messages written to stderr")
                .long("verbose")
                .short("v")
        )
        .get_matches();

    let mut stderr = io::stderr();
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let verbose = matches.is_present("verbose");

    for line in stdin.lock().lines() {

        match hex::FromHex::from_hex(line.unwrap().into_bytes()) {
            Ok(buffer) => {
                let unencoded = String::from_utf8(buffer).unwrap();
                println!("{}", unencoded);
                let _ = stdout.flush();
            },
            Err(e) => {
                if verbose {
                    let _ = stderr.write_fmt(format_args!("Error: {}\n", e));
                    let _ = stderr.flush();
                }
            }
        }

    }
}

