use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;

fn main() {
    let r = BufReader::new(stdin());
   
    let mut sum :u32 = 0;

    for game in r.lines() {
        if game.is_err() {
            continue;
        }

        let (_, game) = parse_game_id(game.unwrap());
        let (reds, greens, blues) = find_min_balls(game);

        sum += reds * greens * blues;
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

fn find_min_balls(game: String) -> (u32, u32, u32) {
    let mut reds :u32 = 0;
    let mut greens :u32 = 0;
    let mut blues :u32 = 0;
    for set in game.split("; ") {
        for balls in set.split(", ") {
            let (balls_no, rest) = read_prefix_digits(balls.to_string());
            if rest == " red" {
                if balls_no > reds {
                    reds = balls_no;
                }
            }
            if rest == " green" {
                if balls_no > greens {
                    greens = balls_no;
                }
            }
            if rest == " blue" {
                if balls_no > blues {
                    blues = balls_no;
                }
            }
        } 
    }
    return (reds, greens, blues);
}




