use num::integer::lcm;
use std::{collections::HashMap, fs};

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

fn part_1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines = input.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();

    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.skip(1).for_each(|line| {
        //AAA = (BBB, CCC)
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        maps.insert(name, (left, right));
    });

    let direction = directions.iter().cycle();
    let mut current_map = "AAA";
    let jumps = direction
        .take_while(|&&dir| {
            if current_map == "ZZZ" {
                return false;
            }
            if dir == 'L' {
                current_map = maps.get(current_map).unwrap().0;
            } else {
                current_map = maps.get(current_map).unwrap().1;
            }
            true
        })
        .count();

    // dbg!(&directions, &maps);

    jumps as u32
}

fn part_2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut lines = input.lines();
    let directions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();

    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.skip(1).for_each(|line| {
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        maps.insert(name, (left, right));
    });

    // Filter for nodes that end with "A".
    let starts: Vec<&str> = maps
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect();

    starts
        .iter()
        .map(|&start| {
            // Find the cyce length for each starting node.
            let mut current = start;
            for (i, &dir) in directions.iter().cycle().enumerate() {
                if current.ends_with('Z') {
                    return i;
                }

                current = if dir == 'L' {
                    maps.get(current).unwrap().0
                } else {
                    maps.get(current).unwrap().1
                };
            }
            panic!("No cycle found for {}", start);
        })
        // Calculate the least common multiple of all cycle lengths.
        .fold(1, lcm) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let result = part_1("example1.txt");
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_1_example_2() {
        let result = part_1("example2.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example3.txt");
        assert_eq!(result, 6);
    }
}
