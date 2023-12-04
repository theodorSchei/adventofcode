use std::fs;

fn main() {
    dbg!(part_1("input.txt"));
    dbg!(part_2("input.txt"));
}

#[derive(Debug)]
struct NumCoord {
    num: u32,
    y: usize,
    x: usize,
    end_x: usize,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    y: usize,
    x: usize,
}

impl NumCoord {
    fn is_neighbor_with_symbol(&self, symbol: &Symbol) -> bool {
        let mut xs_to_check = Vec::<usize>::new();
        for i in self.x.saturating_sub(1)..=self.end_x.saturating_add(1) {
            xs_to_check.push(i);
        }
        let ys_to_check = vec![self.y.saturating_sub(1), self.y, self.y.saturating_add(1)];
        xs_to_check.contains(&symbol.x) && ys_to_check.contains(&symbol.y)
    }

    fn has_neighboring_symbol(&self, symbols: &[Symbol]) -> bool {
        symbols
            .iter()
            .any(|symbol| self.is_neighbor_with_symbol(symbol))
    }
}

impl Symbol {
    fn gear_ratio(&self, num_coords: &[NumCoord]) -> Option<u32> {
        if self.symbol != '*' {
            return None;
        }
        let neighbors = num_coords
            .iter()
            .filter(|num_coord| num_coord.is_neighbor_with_symbol(self))
            .collect::<Vec<&NumCoord>>();

        if neighbors.len() == 2 {
            Some(neighbors.iter().map(|num_coord| num_coord.num).product())
        } else {
            None
        }
    }
}

fn part_1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let number_regex = regex::Regex::new(r"\d+").unwrap();

    let mut num_coords = Vec::<NumCoord>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (y, line) in input.lines().enumerate() {
        let numbers = number_regex.find_iter(line);
        for finding in numbers {
            let num = finding.as_str().parse::<u32>().unwrap();
            num_coords.push(NumCoord {
                num,
                y,
                x: finding.start(),
                end_x: finding.end() - 1,
            });
        }
        for (x, char) in line.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                symbols.push(Symbol { symbol: char, y, x });
            }
        }
    }

    num_coords
        .iter()
        .filter(|num_coord| num_coord.has_neighboring_symbol(&symbols))
        .map(|num_coord| num_coord.num)
        .sum()
}

fn part_2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let number_regex = regex::Regex::new(r"\d+").unwrap();

    let mut num_coords = Vec::<NumCoord>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (y, line) in input.lines().enumerate() {
        let numbers = number_regex.find_iter(line);
        for finding in numbers {
            let num = finding.as_str().parse::<u32>().unwrap();
            num_coords.push(NumCoord {
                num,
                y,
                x: finding.start(),
                end_x: finding.end() - 1,
            });
        }
        for (x, char) in line.chars().enumerate() {
            if char != '.' && !char.is_ascii_digit() {
                symbols.push(Symbol { symbol: char, y, x });
            }
        }
    }

    symbols
        .iter()
        .filter_map(|symbol| symbol.gear_ratio(&num_coords))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1("example.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("example.txt");
        assert_eq!(result, 467835);
    }
}
