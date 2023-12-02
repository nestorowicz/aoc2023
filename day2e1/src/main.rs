use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;

const RED_LIMIT :u32 = 12;
const GREEN_LIMIT :u32 = 13;
const BLUE_LIMIT :u32 = 14;

fn main() {
    let r = BufReader::new(stdin());
   
    let mut sum :u32 = 0;

    for game in r.lines() {
        if game.is_err() {
            continue;
        }

        let (id, game) = parse_game_id(game.unwrap());
        if !game_possible(game) {
            continue;
        }

        sum += id;
    }

    println!("{}", sum);
}

fn parse_game_id(game: String) -> (u32, String) {
    let line_str :String =
        game
        .strip_prefix("Game ").expect("missing Game prefix")
        .to_string();
    let (id, line_str) = read_prefix_digits(line_str);
    let line_str = line_str.strip_prefix(": ").expect("missing : after Game").to_string();
    return (id, line_str);
}

fn read_prefix_digits(chs :String) -> (u32, String) {
    let digits :String = chs
        .chars()
        .take_while(|x| x.is_digit(10))
        .collect();
    let number = digits.parse().expect("conversion failed");
    return (number, chs.strip_prefix(&digits).expect("strip failed").to_string());
}

fn game_possible(game: String) -> bool {
    for set in game.split("; ") {
        for balls in set.split(", ") {
            let (balls_no, rest) = read_prefix_digits(balls.to_string());
            if rest == " red" && balls_no > RED_LIMIT {
                return false;
            }
            if rest == " green" && balls_no > GREEN_LIMIT {
                return false;
            }
            if rest == " blue" && balls_no > BLUE_LIMIT {
                return false;
            }
        } 
    }
    return true;
}




