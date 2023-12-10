use std::io::prelude::*;
use std::io::BufReader;
use std::io;

fn main() {
    let mut lines = BufReader::new(io::stdin()).lines()
        .map(|l| l.expect("failed to read line"))
        .map(|l| l.into_bytes());

    if let Some(sum) = sum_parts_numbers(&mut lines) {
        println!("Sum of part numbers: {}", sum);
    }
}

fn sum_parts_numbers<'a, I>(lines :&mut I) -> Option<u32>
where
    I: Iterator<Item = Vec<u8>>
{
    let mut sum = 0;
    let mut prev_line :Option<Vec<u8>> = None;
    let mut current_line :Option<Vec<u8>> = lines.next();
    let mut next_line :Option<Vec<u8>> = lines.next();

    while current_line.is_some() {
        sum += scan_line(current_line.as_deref(), prev_line.as_deref(), next_line.as_deref());

        prev_line = current_line;
        current_line = next_line;
        next_line = lines.next();
    }
    
    return Some(sum);
}

fn scan_line(line :Option<&[u8]>, previous_line :Option<&[u8]>, next_line :Option<&[u8]>) -> u32 {
    let line = line.unwrap_or(&[]);
    let mut sum = 0;
    let mut i :usize = 0;
    loop {
        let num_start = (i..line.len())
            .find(|i| line[*i].is_ascii_digit());
        if num_start.is_none() {
            return sum;
        }
        let num_start = num_start.unwrap();
        let mut is_part = vec!(Some(line), previous_line, next_line)
            .iter().any(|l| has_symbol_at(*l, prev(num_start)));
        let num_end = (num_start..line.len())
            .find(|i| !line[*i].is_ascii_digit())
            .unwrap_or(line.len());
        for i in num_start..num_end {
            is_part = is_part ||
                vec!(previous_line, next_line).iter().any(|l| has_symbol_at(*l, i));
        }
        is_part = is_part ||
            vec!(Some(line), previous_line, next_line).iter().any(|l| has_symbol_at(*l, num_end));
        if is_part {
            sum += parse_number(line, num_start, num_end);
        }
        i = num_end;
        if i == line.len() {
            return sum;
        }
    }
}

fn prev(i :usize) -> usize {
    return if i == 0 { 0 } else {i-1};
}

fn has_symbol_at(line :Option<&[u8]>, index :usize) -> bool {
    let symbol = line.unwrap_or(&[] as &[u8]).get(index).unwrap_or(&b'.');
    return !symbol.is_ascii_digit() && symbol != &b'.';
}

fn parse_number(line: &[u8], i :usize, j :usize) -> u32 {
    return String::from_utf8(line[i..j].to_vec()).unwrap_or(String::from("")).parse::<u32>().unwrap_or(0);
}

