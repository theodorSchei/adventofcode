use std::fs;

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

#[derive(Debug)]
struct Map {
    maps: Vec<MapEntry>,
}

#[derive(Debug)]
struct MapEntry {
    dst_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

impl MapEntry {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let dst_range_start = iter.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = iter.next().unwrap().parse::<u64>().unwrap();
        let range_length = iter.next().unwrap().parse::<u64>().unwrap();
        Self {
            dst_range_start,
            src_range_start,
            range_length,
        }
    }
}

impl Map {
    fn from_str(s: &str) -> Self {
        // Skip the first line and parse the rest
        let iter = s.lines().skip(1);
        let mut maps = Vec::new();
        for line in iter {
            maps.push(MapEntry::from_str(line));
        }
        Self { maps }
    }

    fn map(&self, src: u64) -> u64 {
        for map in &self.maps {
            if src >= map.src_range_start && src < map.src_range_start + map.range_length {
                return src - map.src_range_start + map.dst_range_start;
            }
        }
        src
    }

    fn reverse_map(&self, dst: u64) -> u64 {
        for map in &self.maps {
            if dst >= map.dst_range_start && dst < map.dst_range_start + map.range_length {
                return dst - map.dst_range_start + map.src_range_start;
            }
        }
        dst
    }
}

fn part_1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut iter = input.split("\n\n");
    let seeds = iter
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps: Vec<Map> = iter.map(Map::from_str).collect();

    seeds
        .iter()
        .map(|seed| {
            let mut src = *seed;
            for map in &maps {
                src = map.map(src);
            }
            src
        })
        .min()
        .unwrap_or(0)
}

fn part_2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut iter = input.split("\n\n");
    let seed_pairs: Vec<(u64, u64)> = iter
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            (start, start + length)
        })
        .collect();

    let maps: Vec<Map> = iter.map(Map::from_str).collect();

    // Reverse brute force
    let mut location = 0;
    loop {
        let mut current = location;
        for map in maps.iter().rev() {
            current = map.reverse_map(current);
        }
        for pair in &seed_pairs {
            if current >= pair.0 && current < pair.1 {
                return location;
            }
        }

        location += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 46);
    }
}
