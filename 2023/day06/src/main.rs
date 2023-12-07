use std::fs;

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

fn part_1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut iter = input.lines();

    let times = iter
        .next()
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = iter
        .next()
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            record_distance: *distance,
        })
        .collect();

    races
        .iter()
        .map(|race| {
            let mut winning_strats = 0;

            for holding_time in 0..race.time {
                let distance_traveled = (race.time - holding_time) * holding_time;
                if distance_traveled > race.record_distance {
                    winning_strats += 1;
                }
            }
            winning_strats
        })
        .product::<u32>()
}

fn part_2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut iter = input.lines();

    let time = iter
        .next()
        .unwrap()
        .split("Time:")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = iter
        .next()
        .unwrap()
        .split("Distance:")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let race = Race {
        time,
        record_distance: distance,
    };

    let mut min_hold_strat = 0;

    for holding_time in 0..race.time {
        let distance_traveled = (race.time - holding_time) * holding_time;
        if distance_traveled > race.record_distance {
            min_hold_strat = holding_time;
            break;
        }
    }

    let mut max_hold_strat = race.time;
    for holding_time in (0..race.time).rev() {
        let distance_traveled = (race.time - holding_time) * holding_time;
        if distance_traveled > race.record_distance {
            max_hold_strat = holding_time;
            break;
        }
    }

    max_hold_strat - min_hold_strat + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 71503);
    }
}
