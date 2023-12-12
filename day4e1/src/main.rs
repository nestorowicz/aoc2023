use std::collections::HashSet;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let lines = stdin().lock().lines()
        .filter_map(|line| line.map_err(|e| eprintln!("failed to read line: {e:?}")).ok())
        .map(|line| process_scratchcard(line))
        .filter_map(|points| points.map_err(|e| eprintln!("failed to process scratchcard: {e:?}")).ok())
        .reduce(|sum, points| sum + points)
        .unwrap_or(0);
    println!("{lines}");
}

fn process_scratchcard(card: String) -> Result<u64, String> {
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
    if nums == 0 {
        return Ok(0);
    }
    return Ok(2u64.pow(nums.checked_sub(1).unwrap_or(0)));
        
}

