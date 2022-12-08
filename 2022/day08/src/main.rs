use std::env;
use std::fs;

fn main() {
    let input = read_file();
    let lines: Vec<Vec<u8>> = input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    part_1(&lines);
    part_2(&lines);
}

fn is_visible(h: usize, w: usize, grid: &Vec<Vec<u8>>) -> bool {
    let mut visible_left = true;
    let mut visible_right = true;
    let mut visible_up = true;
    let mut visible_down = true;
    let checking = grid[h][w];
    // left
    for i in 0..w {
        if grid[h][i] >= checking {
            visible_left = false;
            break;
        }
    }
    // right
    for i in (w + 1)..grid[h].len() {
        if grid[h][i] >= checking {
            visible_right = false;
            break;
        }
    }
    // up
    for i in 0..h {
        if grid[i][w] >= checking {
            visible_up = false;
            break;
        }
    }
    // down
    for i in (h + 1)..grid.len() {
        if grid[i][w] >= checking {
            visible_down = false;
            break;
        }
    }
    visible_left || visible_right || visible_up || visible_down
}

fn get_scenic_score(h: usize, w: usize, grid: &Vec<Vec<u8>>) -> u32 {
    let mut visible_left = 0;
    let mut visible_right = 0;
    let mut visible_up = 0;
    let mut visible_down = 0;
    let checking = grid[h][w];
    // left
    for i in (0..w).rev() {
        visible_left += 1;
        if grid[h][i] >= checking {
            break;
        }
    }
    // right
    for i in (w + 1)..grid[h].len() {
        visible_right += 1;
        if grid[h][i] >= checking {
            break;
        }
    }
    // up
    for i in (0..h).rev() {
        visible_up += 1;
        if grid[i][w] >= checking {
            break;
        }
    }
    // down
    for i in (h + 1)..grid.len() {
        visible_down += 1;
        if grid[i][w] >= checking {
            break;
        }
    }
    visible_left * visible_right * visible_up * visible_down
}

fn part_1(grid: &Vec<Vec<u8>>) {
    let mut answer: u32 = 0;
    for h in 0..grid.len() {
        for w in 0..grid[0].len() {
            if is_visible(h, w, grid) {
                answer += 1;
            }
        }
    }
    println!("Part 1: {answer}")
}

fn part_2(grid: &Vec<Vec<u8>>) {
    let mut scenic_scores: Vec<u32> = Vec::new();
    for h in 0..grid.len() {
        for w in 0..grid[0].len() {
            scenic_scores.push(get_scenic_score(h, w, grid));
        }
    }
    let answer: u32 = *scenic_scores.iter().max().unwrap();
    println!("Part 2: {answer}")
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}
