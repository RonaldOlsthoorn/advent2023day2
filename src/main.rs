
use std::{io::{BufReader, BufRead}, fs::File, cmp};

struct Pick {
    red: u16,
    blue: u16,
    green: u16
}

struct Game {
    id: u32,
    picks: Vec<Pick>
}

impl From<&String> for Game {
    fn from(value: &String) -> Self {
        let (id_part, picks_part): (&str, &str) = parse_game(value);
        let id: u32 = id_part[5..].parse().unwrap();

        let picks: Vec<Pick> = parse_picks(picks_part);
        Game{id, picks}
    }
}

fn parse_game(input_line: &String) -> (&str, &str) {
    let split: Vec<&str> = input_line.split(":").collect();
    (split[0], split[1])
}

fn parse_picks(pick_line: &str) -> Vec<Pick> {
    pick_line.split(";").map(|l| l.trim()).map(|l| parse_pick(l)).collect()
}

fn parse_pick(pick: &str) -> Pick {
    let color_specifications: Vec<&str> = pick.split(",").map(|l| l.trim()).collect();

    let mut res = Pick{red: 0, blue: 0, green: 0};

    for color_specification in color_specifications {

        let splits: Vec<&str> = color_specification.split(" ").collect();
        let quantity: u16 = splits[0].parse().unwrap();
        let color: &str = splits[1];

        match color {
            "red" => res.red = quantity,
            "green" => res.green = quantity,
            "blue" => res.blue = quantity,
            _ => panic!("Parsing error. Unknown color {}", color)
        }
    }

    res
}

fn check_game(game: &Game) -> bool {

    for pick in game.picks.iter() {
        if pick.red > 12 || pick.green > 13 || pick.blue > 14 {
            return false;
        }
    }
    return true;
}

fn minimum_pick(game: &Game) -> Pick {
    let mut res = Pick{red: 0, blue: 0, green: 0};

    for pick in game.picks.iter() {
        res.red = cmp::max(res.red, pick.red);
        res.blue = cmp::max(res.blue, pick.blue);
        res.green = cmp::max(res.green, pick.green);
    }

    res
}

fn power(pick: &Pick) -> u32 {
    (pick.red as u32) * (pick.blue as u32) * (pick.green as u32) 
} 

fn part1(games: &Vec<Game>) -> u32 {
    games.iter().filter(|&game| check_game(game)).map(|game| game.id).sum()
}

fn part2(games: &Vec<Game>) -> u32 {
    games.iter().map(|g| power(&minimum_pick(g))).sum()
}

fn main() {
   
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let games: Vec<Game> = lines.iter().map(|line| line.into()).collect();

    println!("part 1 {}", part1(&games));
    println!("part 2 {}", part2(&games));
}