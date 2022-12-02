use std::env;
use std::fs;

fn main() {
    day_1();
    day_2();
}

fn day_1() {
    let input = read_file();
    let mut split: Vec<&str> = input.split("\n").collect();
    let mut elfs_total_calories: Vec<i32> = Vec::<i32>::new();
    let mut sum: i32 = 0;
    for i in 0..split.len() {
        if split[i] == "" {
            elfs_total_calories.push(sum);
            sum = 0;
        } else {
            let mut calories: i32 = split[i].parse().unwrap();
            sum += calories;
        }
    }
    let maxValue = elfs_total_calories.iter().max();
    match maxValue {
        Some(max) => println!("Max value: {}", max),
        None => println!("Vector is empty"),
    }
}

fn day_2() {
    let input = read_file();
    let mut split: Vec<&str> = input.split("\n").collect();
    let mut elfs_total_calories: Vec<i32> = Vec::<i32>::new();
    let mut sum: i32 = 0;

    for i in 0..split.len() {
        if split[i] == "" {
            elfs_total_calories.push(sum);
            sum = 0;
        } else {
            let mut calories: i32 = split[i].parse().unwrap();
            sum += calories;
        }
    }

    elfs_total_calories.sort();
    let last3 = &elfs_total_calories[elfs_total_calories.len() - 3..elfs_total_calories.len()];
    let sum: i32 = last3.iter().sum();
    println!("{}", sum);

}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
