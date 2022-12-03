use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split("\n").collect();

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<&str>) {
    let scores: HashMap<char, HashMap<char, u8>> = [
        (
            'X',
            [('A', 1 + 3), ('B', 1 + 0), ('C', 1 + 6)]
                .iter()
                .cloned()
                .collect(),
        ),
        (
            'Y',
            [('A', 2 + 6), ('B', 2 + 3), ('C', 2 + 0)]
                .iter()
                .cloned()
                .collect(),
        ),
        (
            'Z',
            [('A', 3 + 0), ('B', 3 + 6), ('C', 3 + 3)]
                .iter()
                .cloned()
                .collect(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let enemy: char = lines[i].chars().nth(0).unwrap();
        let me: char = lines[i].chars().nth(2).unwrap();
        sum += scores[&me][&enemy] as u32;
    }
    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<&str>) {
    let scores: HashMap<char, HashMap<char, u8>> = [
        (
            'X',
            [('A', 3), ('B', 1), ('C', 2)].iter().cloned().collect(),
        ),
        (
            'Y',
            [('A', 4), ('B', 5), ('C', 6)].iter().cloned().collect(),
        ),
        (
            'Z',
            [('A', 8), ('B', 9), ('C', 7)].iter().cloned().collect(),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let enemy: char = lines[i].chars().nth(0).unwrap();
        let me: char = lines[i].chars().nth(2).unwrap();
        sum += scores[&me][&enemy] as u32;
    }
    println!("Part 2: {}", sum);
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
