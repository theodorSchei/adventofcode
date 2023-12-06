use std::fs;

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

#[derive(Debug)]
struct Game {
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

impl Game {
    fn num_winning_numbers(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|num| self.our_numbers.contains(num))
            .count()
            .try_into()
            .unwrap()
    }

    fn worth(&self) -> Option<u32> {
        if self.num_winning_numbers() != 0 {
            Some(2_u32.pow(self.num_winning_numbers() - 1))
        } else {
            None
        }
    }
}

fn parse_game(line: &str) -> Game {
    let re = regex::Regex::new(r"Card +\d+: ([\d ]+) \| ([\d ]+)").unwrap();
    let caps = re.captures(line).unwrap();
    let winning_numbers = caps
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let your_numbers = caps
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Game {
        winning_numbers,
        our_numbers: your_numbers,
    }
}

fn part_1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    input.lines().filter_map(|x| parse_game(x).worth()).sum()
}

fn part_2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let games = input.lines().map(parse_game).collect::<Vec<Game>>();

    let mut counts = vec![1; games.len()];
    for i in 0..games.len() {
        for j in 0..games[i].num_winning_numbers() as usize {
            counts[i + j + 1] += counts[i];
        }
    }
    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 30);
    }
}
