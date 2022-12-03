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
    

    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<&str>) {
   println!("Part 2: {}", sum);
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
