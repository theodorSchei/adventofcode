use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split("\n").collect();

    part_1(&lines);
    part_2(&lines);
}

fn get_char_value(c: char) -> u32 {
    let ascii: u32 = c as u32;
    let value: u32;

    if ascii < 95 {
        // Uppercase
        value = ascii - 38
    } else {
        // Lowercase
        value = ascii - 96
    }
    value
}

fn part_1(lines: &Vec<&str>) {
    let mut sum: u32 = 0;
    for i in 0..lines.len() {
        let len = lines[i].len();

        let a = &lines[i][..len / 2];
        let b = &lines[i][len / 2..];

        let a_set: HashSet<char> = a.chars().collect();
        let b_set: HashSet<char> = b.chars().collect();
        
        if let Some(c) = a_set.intersection(&b_set).next() {
            sum += get_char_value(*c);
        }
    }

    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<&str>) {
    let mut sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let group = &lines[i..i + 3];

        let a_set: HashSet<char> = group[0].chars().collect();
        let b_set: HashSet<char> = group[1].chars().collect();
        let c_set: HashSet<char> = group[2].chars().collect();

        let a_b: HashSet<char> = a_set.intersection(&b_set).cloned().collect();

        if let Some(c) = a_b.intersection(&c_set).next() {
            sum += get_char_value(*c);
        }
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
