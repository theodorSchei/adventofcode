use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split("\n").collect();

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<&str>) {
    let mut sum = 0;

    for i in 0..lines.len() {
        let elfs: Vec<&str> = lines[i].split(',').collect();

        let a: Vec<i32> = elfs[0].split('-').map(|e| e.parse().unwrap()).collect();
        let b: Vec<i32> = elfs[1].split('-').map(|e| e.parse().unwrap()).collect();

        let a_set: HashSet<i32> = (a[0]..=a[1]).collect();
        let b_set: HashSet<i32> = (b[0]..=b[1]).collect();

        sum += (a_set.is_subset(&b_set) || a_set.is_superset(&b_set)) as i32;
    }
    println!("Part 1: {}", sum);
}

fn part_2(lines: &Vec<&str>) {
    let mut sum = 0;

    for i in 0..lines.len() {
        let elfs: Vec<&str> = lines[i].split(',').collect();

        let a: Vec<i32> = elfs[0].split('-').map(|e| e.parse().unwrap()).collect();
        let b: Vec<i32> = elfs[1].split('-').map(|e| e.parse().unwrap()).collect();

        let a_set: HashSet<i32> = (a[0]..=a[1]).collect();
        let b_set: HashSet<i32> = (b[0]..=b[1]).collect();

        sum += !a_set.is_disjoint(&b_set) as i32;
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
