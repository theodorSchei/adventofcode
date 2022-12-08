use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<&str> = input.split("\n").collect();

    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<&str>) {
    // Current working directory
    let mut cwd: Vec<u32> = vec![];
    // File system
    let mut fs: Vec<u32> = vec![];
    for line in lines {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            // move up a directory, move whatever is in current directory to fs
            ["$", "cd", ".."] => fs.push(cwd.pop().unwrap()),
            // move into a directory, push an empty new dir
            ["$", "cd", _] => cwd.push(0),
            [other, _] => {
                // Will only run on lines that start with number
                if let Ok(size) = other.parse::<u32>() {
                    cwd.iter_mut().for_each(|dir| *dir += size);
                }
            }
            _ => panic!(),
        }
    }
    fs.extend(cwd.iter());

    let answer: u32 = fs.iter().filter(|size| **size < 100_000 as u32).sum();
    println!("Part 1: {answer}")
}

fn part_2(lines: &Vec<&str>) {
    // Current working directory
    let mut cwd: Vec<u32> = vec![];
    // File system
    let mut fs: Vec<u32> = vec![];
    for line in lines {
        match line.split(' ').collect::<Vec<&str>>().as_slice() {
            // move up a directory, move whatever is in current directory to fs
            ["$", "cd", ".."] => fs.push(cwd.pop().unwrap()),
            // move into a directory, push a dir with size 0 to indicate a new empty dir
            ["$", "cd", _] => cwd.push(0),
            [other, _] => {
                // Will only run on lines that start with number
                if let Ok(size) = other.parse::<u32>() {
                    // Add size of file to all dirs in cwd (parents).
                    cwd.iter_mut().for_each(|dir| *dir += size);
                }
            }
            _ => panic!(),
        }
    }
    // Add final dir to fs
    fs.extend(cwd.iter());

    let used: u32 = *fs.iter().max().unwrap();
    let free: u32 = 70_000_000 - used;
    let needed_to_update: u32 = 30_000_000 - free;
    let answer: u32 = *fs
        .iter()
        .filter(|size| **size > needed_to_update)
        .min()
        .unwrap();
    println!("Part 2: {answer}")
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
