

const NUM_RED: i32 = 12;
const NUM_GREEN: i32 = 13;
const NUM_BLUE: i32 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

enum Orbs {
    Red(i32),
    Green(i32),
    Blue(i32)
}

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>
}

fn parse_round(input: &str) -> Round {
    let balls = input.split(',');

    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for ball in balls {
        let ball_vec = ball.trim().split(' ').collect::<Vec<_>>();
        println!("{:?}", ball_vec);
        let color = ball_vec[1];
        let num = ball_vec[0];
        match color {
            RED => red = num.parse::<u32>().expect("failed to parse"),
            GREEN => green = num.parse::<u32>().expect("failed to parse"),
            BLUE => blue = num.parse::<u32>().expect("failed to parse"),
            &_ => panic!("AHHHHHH")
        }
    }
    return Round {
        red,
        green,
        blue
    }
}

fn parse_game(input: &str) -> Game {
    
    Game {
        id: 1,
        rounds: vec![],
    }
}

fn main() {
    let input = include_str!("../input1.txt");
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_round() {
        let expected_answer = Round {
            red: 5,
            blue: 1,
            green: 0
        };
        let input = " 5 red, 1 blue";
        assert_eq!(parse_round(input), expected_answer);
    }
}
