use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split("\n").collect();

    part_1(&lines);
    part_2(&lines);
}

fn parse_input(lines: &Vec<&str>) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let split_index = lines.iter().position(|&line| line.is_empty()).unwrap();

    // Parse stacks
    let number_of_stacks = (lines[split_index - 1].chars().count() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    for i in (0..split_index - 1).rev() {
        let line = lines[i];
        for (c_index, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks[c_index].push(c);
            }
        }
    }

    // Parse instructions
    let mut instructions: Vec<Vec<usize>> = vec![];
    for i in split_index + 1..lines.len() {
        let line: Vec<usize> = lines[i]
            .split(' ')
            .filter(|item| item.chars().all(|c| c.is_numeric()))
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        instructions.push(line);
    }

    (stacks, instructions)
}

fn part_1(lines: &Vec<&str>) {
    let (mut stacks, instructions) = parse_input(lines);
    
    for instruction in instructions {
        let (amount, from, to) = (instruction[0], instruction[1] - 1, instruction[2] - 1);

        let from_length = stacks[from].len();
        // Move [amount] elemenents from [from] to [to]
        let moving: Vec<_> = stacks[from].drain(from_length - amount..).rev().collect();
        stacks[to].extend(moving);
    }
    let answer: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .into_iter()
        .collect::<String>();
    println!("Part 1: {}", answer);
}

fn part_2(lines: &Vec<&str>) {
    let (mut stacks, instructions) = parse_input(lines);
    
    for instruction in instructions {
        let (amount, from, to) = (instruction[0], instruction[1] - 1, instruction[2] - 1);

        let from_length = stacks[from].len();
        // Move [amount] elemenents from [from] to [to]
        let moving: Vec<_> = stacks[from].drain(from_length - amount..).collect();
        stacks[to].extend(moving);
    }
    let answer: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .into_iter()
        .collect::<String>();
    println!("Part 2: {}", answer);
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
