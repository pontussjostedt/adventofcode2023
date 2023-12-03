const NUM_RED: i32 = 12;
const NUM_GREEN: i32 = 13;
const NUM_BLUE: i32 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

enum Orbs {
    Red(i32),
    Green(i32),
    Blue(i32),
}

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
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
            &_ => panic!("AHHHHHH"),
        }
    }
    return Round { red, green, blue };
}

fn parse_game(input: &str) -> Game {
    let mut splitted = input.split(':');
    let id: i32 = splitted
        .next()
        .expect("Faulty input couldnt read Game ID")
        .split(' ')
        .last()
        .expect("Game ID input is wrong")
        .parse::<i32>()
        .expect("Couldnt parse ID");
    let rounds = splitted
        .last()
        .expect("Wrong input string, no rounds")
        .split(';')
        .map(parse_round)
        .collect::<Vec<Round>>();
    Game { id, rounds }
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
            green: 0,
        };
        let input = " 5 red, 1 blue";
        assert_eq!(parse_round(input), expected_answer);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 65: 7 red, 7 blue; 3 blue, 1 red, 1 green; 3 red, 8 blue";
        let expected_answer = Game {
            id: 65,
            rounds: vec![
                Round {
                    red: 7,
                    blue: 7,
                    green: 0,
                },
                Round {
                    red: 1,
                    blue: 3,
                    green: 1,
                },
                Round {
                    red: 3,
                    green: 0,
                    blue: 8,
                },
            ],
        };

        assert_eq!(parse_game(&input), expected_answer);
    }
}
