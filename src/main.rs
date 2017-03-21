
extern crate hex;

use std::io;
use std::io::{Read, Write};


fn main() {

    let mut stderr = io::stderr();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer : [u8; 2] = [0; 2];

    loop {
        let bytes_read = stdin.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        match hex::FromHex::from_hex(buffer) {
            Ok(buffer) => {
                let unencoded = String::from_utf8(buffer).unwrap();
                print!("{}", unencoded);
                let _ = stdout.flush();
            },
            Err(e) => {
                let _ = stderr.write_fmt(format_args!("Error was: {}", e));
                let _ = stderr.flush();
            }
        }

    }
}
