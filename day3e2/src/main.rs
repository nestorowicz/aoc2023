use std::io::prelude::*;
use std::io::BufReader;
use std::io;

fn main() {
    let mut lines = BufReader::new(io::stdin()).lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| l.into_bytes());

    if let Some(sum) = sum_gear_ratios(&mut lines) {
        println!("Sum of part numbers: {}", sum);
    }
}

fn sum_gear_ratios<'a, I>(lines :&mut I) -> Option<u64>
where
    I: Iterator<Item = Vec<u8>>
{
    let mut sum :u64 = 0;
    let mut prev_line :Option<Vec<u8>> = None;
    let mut current_line :Option<Vec<u8>> = lines.next();
    let mut next :Option<Vec<u8>> = lines.next();

    while current_line.is_some() {
        sum += sum_gear_ratios_in_line(current_line.as_deref(), prev_line.as_deref(), next.as_deref());

        prev_line = current_line;
        current_line = next;
        next = lines.next();
    }
    
    return Some(sum);
}

fn sum_gear_ratios_in_line(line :Option<&[u8]>, prev :Option<&[u8]>, next :Option<&[u8]>) -> u64 {
    let line = line.unwrap_or(&[]);
    let mut sum = 0;
    for i in 0..line.len() {
        if line[i] != b'*' {
            continue;
        }
        sum += get_gear_ratio(line, prev, next, i);
    }
    return sum;
}

fn get_gear_ratio(line :&[u8], prev :Option<&[u8]>, next :Option<&[u8]>, i :usize) -> u64 {
    let prev = prev.unwrap_or(&[]);
    let next = next.unwrap_or(&[]);
    let mut n = 0;
    let mut sum = 1;
    let prev_i = i.checked_sub(1).unwrap_or(0);
    if let Some(num) = parse_number(prev, i) {
        n += 1;
        sum *= num;
    } else {
        if let Some(num) = parse_number(prev, prev_i) {
            n += 1;
            sum *= num;
        }
        if let Some(num) = parse_number(prev, i+1) {
            n += 1;
            sum *= num;
        }
    }
    if let Some(num) = parse_number(line, prev_i) {
        n += 1;
        sum *= num;
    }
    if let Some(num) = parse_number(line, i+1) {
        n += 1;
        sum *= num;
    }
    if let Some(num) = parse_number(next, i) {
        n += 1;
        sum *= num;
    } else {
        if let Some(num) = parse_number(next, prev_i) {
            n += 1;
            sum *= num;
        }
        if let Some(num) = parse_number(next, i+1) {
            n += 1;
            sum *= num;
        }
    }
    if n == 2 {
        return sum;
    }
    return 0;
}

fn parse_number(line: &[u8], i :usize) -> Option<u64> {
    if !line.get(i)?.is_ascii_digit() {
        return None;
    }
    let start_idx = (0..i)
        .rev()
        .find(|i| 
              !line.get(*i).unwrap_or(&b'.').is_ascii_digit())
        .map(|i| i+1)
        .unwrap_or(0);
    let end_idx = (i..line.len())
        .find(|i| 
              !line.get(*i).unwrap_or(&b'.').is_ascii_digit())
        .unwrap_or(line.len());
    let num_str = String::from_utf8(line[start_idx..end_idx].to_vec()).expect("failed to create a string");
    let num = num_str.parse::<u64>().expect("failed to parse the number");
    return Some(num);
}

