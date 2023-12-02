use std::fs;

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

fn part_1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    input
        .lines()
        .map(|line| {
            let mut first_digit: char = ' ';
            let mut last_digit: char = ' ';
            for char in line.chars() {
                if char.is_ascii_digit() {
                    if first_digit == ' ' {
                        first_digit = char;
                        last_digit = char;
                    } else {
                        last_digit = char;
                    }
                }
            }
            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
}

fn part_2(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let spelled_numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|raw_line| {
            let new_line = {
                let mut new_line = String::from(raw_line);
                for (index, spelled_number) in spelled_numbers.iter().enumerate() {
                    new_line = new_line.replace(
                        spelled_number,
                        format!("{}{}{}", spelled_number, index, spelled_number).as_str(),
                    );
                }
                new_line
            };

            let mut first_digit: char = ' ';
            let mut last_digit: char = ' ';
            for char in new_line.chars() {
                if char.is_ascii_digit() {
                    if first_digit == ' ' {
                        first_digit = char;
                        last_digit = char;
                    } else {
                        last_digit = char;
                    }
                }
            }
            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example2.txt");
        assert_eq!(result, 281);
    }
}
