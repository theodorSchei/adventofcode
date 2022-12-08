use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let line = read_file();

    part_1(&line);
    part_2(&line);
}

fn part_1(line: &String) {
    let mut answer = 0;
    let marker_len = 4;
    for i in 0..line.len() - marker_len {
        let chars: &str = &line[i..i + marker_len];
        let marker: HashSet<char> = HashSet::from_iter(chars.chars());

        if marker.len() == marker_len {
            answer = i + marker_len;
            break;
        }
    }
    println!("Part 1: {}", answer);
}

fn part_2(line: &String) {
    let mut answer = 0;
    let marker_len = 14;
    for i in 0..line.len() - marker_len {
        let chars: &str = &line[i..i + marker_len];
        let marker: HashSet<char> = HashSet::from_iter(chars.chars());

        if marker.len() == marker_len {
            answer = i + marker_len;
            break;
        }
    }
    println!("Part 2: {}", answer);
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
