use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;

fn main() {
    let r = BufReader::new(stdin());
   
    let mut sum :u32 = 0;

    for line in r.lines() {
        if let Err(e) = line {
            eprintln!("error: {}", e);
            break;
        }

        let mut a: Option<u32> = None;
        let mut b: Option<u32> = None;

        let mut sub_line = &line.unwrap()[0..];
        while sub_line.len() > 0 {
          if let Some(n) = try_parse(sub_line) {
              if a.is_none() {
                  a = Some(n);
              }
              b = Some(n);
          }

          sub_line = &sub_line[1..];
        }
        if a.is_some() && b.is_some() {
            sum += (a.unwrap()*10) + b.unwrap();
        }
    }

    println!("{}", sum);
}

fn try_parse(chs :&str) -> Option<u32> {
    if let Some(ch) = chs.chars().nth(0) {
        if ch.is_digit(10) {
            return ch.to_digit(10);
        }
    }

    if chs.starts_with("one") {
        return Some(1);
    } else if chs.starts_with("two") {
        return Some(2);
    } else if chs.starts_with("three") {
        return Some(3);
    } else if chs.starts_with("four") {
        return Some(4);
    } else if chs.starts_with("five") {
        return Some(5);
    } else if chs.starts_with("six") {
        return Some(6);
    } else if chs.starts_with("seven") {
        return Some(7);
    } else if chs.starts_with("eight") {
        return Some(8);
    } else if chs.starts_with("nine") {
        return Some(9);
    } else if chs.starts_with("ten") {
        return Some(10);
    }
    return None;
}



