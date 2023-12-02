
fn line_to_number(line: &str) -> i32 {
    let mut numbers = line.chars().filter(|ch| ch.is_digit(10));
    let head = numbers.next().expect("No num? :)");
    let last = numbers.last().unwrap_or(head); // If no "last" digit it means that there only was once hence its head and last
    format!("{}{}", head, last).parse::<i32>().unwrap()
}

fn sum_hidden_code(input: &str) -> i32 {
    input.lines().map(line_to_number).sum::<i32>()
}

fn main() {
    let input = include_str!("../input1.txt");
    println!("output day1 part1 = {}", sum_hidden_code(input))
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
}
