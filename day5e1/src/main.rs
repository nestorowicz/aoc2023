use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let _ = calculate_lowest_location().map(|res| println!("{res}"));
}

struct Map {
    translations: Vec<Translation>
}

impl Map {
    fn translate(&self, i :u64) -> u64 {
      self.translations.iter()
          .find(|t| t.src <= i && i < (t.src + t.range))
          .map(|t| t.dest + (i - t.src))
          .unwrap_or(i)
    }
}

struct Translation {
    dest: u64,
    src: u64,
    range: u64
}

fn calculate_lowest_location() -> Option<u64> {
    let mut lines = stdin().lock()
        .lines()
        .filter_map(|l| l.map_err(|e| { eprintln!("{e}"); }).ok() );

    let mut seeds = parse_seeds(lines.next().expect("missing header line"));
    _ = lines.next();

    loop {
        let map = parse_map(&mut lines);
        if map.is_none() {
            return seeds.iter().min().copied();
        }
        let map = map?;
        seeds = seeds.iter().map(|s| map.translate(*s)).collect();
    }
}

fn parse_seeds(line :String) -> Vec<u64> {
    line.trim_start_matches("seeds: ")
        .split(' ')
        .filter_map(|i| i.parse().ok())
        .collect()
}

fn parse_map<'a, I>(lines :&mut I) -> Option<Map>
where
    I: Iterator<Item = String>
{
    _ = lines.next()?;
    let translations :Vec<Translation> = lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(" ").map(|s| s.to_string()).collect())
        .filter_map(parse_translations) 
        .collect();
    Some(Map{translations})
}


fn parse_translations(nums :Vec<String>) -> Option<Translation>
{
    let mut nums = nums.iter()
       .filter_map(|num| num.parse().ok());
    Some(Translation {
        dest: nums.next()?,
        src: nums.next()?,
        range: nums.next()?
    })
}


