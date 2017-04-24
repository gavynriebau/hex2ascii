
extern crate hex;
extern crate clap;

use std::io;
use std::io::{Write, BufRead};
use clap::{App, Arg};
use std::error::Error;
use hex::{ToHex};

fn from_hex(s : String) -> Result<Vec<u8>, Box<Error>> {
    match hex::FromHex::from_hex(s) {
        Ok(raw) => Ok(raw),
        Err(e) => Err(Box::new(e))
    }
}

fn to_hex(s : String) -> Result<Vec<u8>, Box<Error>> {
    let hex = s.to_hex();
    Ok(hex.into_bytes())
}

fn main() {

    let matches = App::new("hex2ascii")
        .author("Gavyn Riebau")
        .version("1.0.1")
        .about("Converts hex values to ascii")
        .arg(
            Arg::with_name("verbose")
                .help("Include verbose output including warning messages written to stderr")
                .long("verbose")
                .short("v")
        )
        .arg(
            Arg::with_name("reverse")
                .help("Converts from ascii to hex rather than the other way around")
                .long("reverse")
                .short("r")
        )
        .get_matches();

    let mut stderr = io::stderr();
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        // If the "reverse" flag is supplied we convert to hex rather than from hex.
        let process = if matches.is_present("reverse") {
            to_hex
        } else {
            from_hex
        };

        match process(line.unwrap()) {
            Ok(buffer) => {
                let unencoded = String::from_utf8(buffer).unwrap();
                println!("{}", unencoded);
                let _ = stdout.flush();
            },
            Err(e) => {
                if matches.is_present("verbose") {
                    let _ = stderr.write_fmt(format_args!("Error: {}\n", e));
                    let _ = stderr.flush();
                }
            }
        }

    }
}

