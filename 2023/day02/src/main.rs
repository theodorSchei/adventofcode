use std::{collections::HashMap, fs};

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

fn part_1(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let bag: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .map(|line| -> i32 {
            let mut parts = line.split(':');

            let game_num = parts
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let mut rounds = parts.next().unwrap().split(';');

            let possible = rounds.all(|round| {
                let mut pull = round.split(',');
                pull.all(|pull| {
                    let mut iter = pull.split_whitespace();
                    let num = iter.next().unwrap().parse::<i32>().unwrap();
                    let color = iter.next().unwrap();

                    bag.get(color).unwrap() >= &num
                })
            });

            if possible {
                game_num
            } else {
                0
            }
        })
        .sum()
}

fn part_2(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    input
        .lines()
        .map(|line| -> i32 {
            let mut parts = line.split(':');

            let rounds = parts.nth(1).unwrap().split(';');

            let mut bag = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            rounds.for_each(|round| {
                let pull = round.split(',');
                pull.for_each(|pull| {
                    let mut iter = pull.split_whitespace();
                    let num = iter.next().unwrap().parse::<i32>().unwrap();
                    let color = iter.next().unwrap();
                    // Insert in bag if bigger
                    if bag.get(color).unwrap() < &num {
                        bag.insert(color, num);
                    }
                });
            });

            bag.values().product()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 2286);
    }
}
