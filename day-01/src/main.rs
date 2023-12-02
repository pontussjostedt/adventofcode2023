const SPELLED_OUT_NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn line_to_number(line: &str) -> i32 {
    let mut numbers = line.chars().filter(|ch| ch.is_digit(10));
    let head = numbers.next().expect("No num? :)");
    let last = numbers.last().unwrap_or(head); // If no "last" digit it means that there only was once hence its head and last
    format!("{}{}", head, last).parse::<i32>().unwrap()
}

fn sum_hidden_code(input: &str) -> i32 {
    input.lines().map(line_to_number).sum::<i32>()
}

fn line_to_number2(line: &str) -> i32 {
    let mut num_chars = (0..line.len()).filter_map(|index| {
        let left: &str = &line[index..];
        parse_digit(&left).or_else(|| parse_string_digit(&left))
    });

    let head = num_chars.next().expect("THERE WAS NO NUMBER AHHHH");
    let last = num_chars.last().unwrap_or(head);
    format!("{}{}", head, last).parse::<i32>().unwrap()

}

fn parse_digit(input: &str) -> Option<char> {
    input.chars().next().filter(|c| c.is_digit(10))
}

fn parse_string_digit(input: &str) -> Option<char> {
    for (index, str_num) in SPELLED_OUT_NUMBERS.iter().enumerate() {
        if input.starts_with(str_num) {
            return char::from_digit((index+1) as u32, 10)
        }
    }
    None
}

fn main() {
    let input = include_str!("../input1.txt");
    println!("output day1 part1 = {}", sum_hidden_code(&input));
    println!("output day2 part2 = {}", input.lines().map(|line| line_to_number2(line)).sum::<i32>())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_day_one() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(super::sum_hidden_code(input), 142);
    }

    #[test]
    fn test_day_two() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(input.lines().map(|line| super::line_to_number2(line)).sum::<i32>(), 281);
    }
}
