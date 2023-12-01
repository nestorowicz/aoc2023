use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;

fn main() {
    
    let r = BufReader::new(stdin());
   

    let mut a: Option<u8> = None;
    let mut b: Option<u8> = None;
    let mut sum :u64= 0;

    for c in r.bytes() {
        if let Err(e) = c {
            eprintln!("error: {}", e);
            continue;
        }
        let ch = c.unwrap();
        if ch == b'\n' {
            if a.is_none() || b.is_none() {
                continue;
            }
            sum += (a.unwrap() as u64*10) + b.unwrap() as u64;
            a = None;
            b = None;
            continue;
        }
        if ch < b'0' || ch > b'9' {
            continue;
        }
        if a.is_none() {
            a = Some(ch-b'0');
        }
        b = Some(ch-b'0');
    }

    println!("{}", sum);
}



