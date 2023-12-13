use std::collections::HashSet;
use std::collections::LinkedList;
use std::io::prelude::*;
use std::io::stdin;

const BUF_LEN :u32 = 10;

fn main() {
    let mut copies :LinkedList<u32> = LinkedList::new();
    (0..BUF_LEN).for_each(|_| copies.push_back(1));
    let sum = stdin().lock().lines()
        .filter_map(|line| line.map_err(|e| eprintln!("failed to read line: {e:?}")).ok())
        .map(|line| parse_scratchcard(line))
        .filter_map(|matches| matches.map_err(|e| eprintln!("failed to process scratchcard: {e:?}")).ok())
        .fold(0, |sum, matches| sum + update_copies(&mut copies, matches));
    println!("{sum}");
}

fn parse_scratchcard(card: String) -> Result<u32, String> {
    let mut iter = card.split(": ");
    _ = iter.next().ok_or(format!("missing scratchcard header: {card}"))?;
    let content = iter.next().ok_or(format!("missing scratchard content: {card}"))?;
    let mut iter = content.split(" | ");
    let winning :HashSet<u32, _> = iter.next().ok_or(format!("missing winning numbers: {card}"))?
        .split(" ")
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<u32>().map_err(|e| format!("failed to parse num {e}: {card}")))
        .collect::<Result<Vec<u32>,_>>()
        .map(|nums| -> HashSet<u32> { HashSet::from_iter(nums) })?;
    let nums :u32 = iter.next().ok_or(format!("missing numbers: {card}"))?
        .split(" ")
        .filter(|num| num.len() > 0)
        .map(|num| num.parse::<u32>().map_err(|e| format!("failed to parse num {e}: {card}")))
        .collect::<Result<Vec<_>,_>>()?
        .iter()
        .filter(|num| winning.contains(num))
        .count().try_into().map_err(|e| format!("failed to convert points to u32 {e}"))?;
    return Ok(nums);
}

fn update_copies(copies :&mut LinkedList<u32>, matches :u32) -> u32 {
    let current = copies.pop_front().expect("missing copies entry");
    copies.push_back(1);
    let mut iter = copies.iter_mut();
    (0..matches)
        .map(|_| iter.next().expect("missing iter next"))
        .for_each(|n| *n += current);
    return current;
}

