use std::cmp::max;
const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

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

fn valid_round(round: &Round) -> bool {
    round.red <= NUM_RED && round.green <= NUM_GREEN && round.blue <= NUM_BLUE
}

fn get_game_power(game: &Game) -> u32 {
    let (red, green, blue) = game.rounds.iter().fold((0, 0, 0), |(red_acc, green_acc, blue_acc), round| {
        (max(red_acc, round.red), max(green_acc, round.green), max(blue_acc, round.blue))
    });
    red * green * blue
}

fn main() {
    let input = include_str!("../input1.txt");
    let answer: i32 = input
        .lines()
        .map(|input|{
            let game = parse_game(input);
            if game.rounds.iter().all(valid_round) {
                game.id
            } else {
                0
            }
        })
        .sum();
    println!("The answer to part 1 = {}", answer);
    let answer2: i32 = input
        .lines()
        .map(|input| {
            let game = parse_game(input);
            get_game_power(&game) as i32
        })
        .sum();
    println!("The answer to part 2 = {}", answer2)
    
    
    
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
