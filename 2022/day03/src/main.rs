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

    if ascii < 95 { // Uppercase
        value = ascii - 38
    } else { // Lowercase
        value = ascii - 96
    }
    value
}

fn part_1(lines: &Vec<&str>) {
    let mut sum = 0;
    for i in 0..lines.len() {
        let len = lines[i].len();
        let first = &lines[i][..len / 2];
        let second = &lines[i][len / 2..];

        'outer: for first_c in 0..first.len() {
            for second_c in 0..second.len() {
                if first.chars().nth(first_c).unwrap() == second.chars().nth(second_c).unwrap() {
                    let matching_char = first.chars().nth(first_c).unwrap();
                    sum += get_char_value(matching_char);
                    break 'outer;
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<&str>) {
    let mut sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let group = &lines[i..i+3];
        'outer:for first_i in 0..group[0].len() {
            for second_i in 0..group[1].len() {
                for third_i in 0..group[2].len() {
                    let char_0 = group[0].chars().nth(first_i).unwrap();
                    let char_1 = group[1].chars().nth(second_i).unwrap();
                    let char_2 = group[2].chars().nth(third_i).unwrap();
                    if char_0 == char_1 && char_1 == char_2 {
                        sum += get_char_value(char_0);
                        break 'outer;
                    }
                }
            }
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
