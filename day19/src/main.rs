use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

use self::Direction::*;

fn grid_at(grid: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    if y >= grid.len() || x >= grid[y].len() {
        ' '
    } else {
        grid[y][x]
    }
}

fn find_in(line: &Vec<char>, needle: char) -> usize {
    for i in 0..line.len() {
        if line[i] == needle {
            return i;
        }
    }

    0
}

fn main() {
    let input = "day19.in";
    let mut file = File::open(input).expect("Unable to open file day19.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let grid: Vec<Vec<char>> = data.split('\n')
        .filter(|x| x.len() > 1)
        .map(|x| x.chars().collect())
        .collect();

    let mut x = find_in(&grid[0], '|');
    let mut y = 0;

    let mut direction = Down;
    let mut steps = 0;

    loop {
        steps += 1;

        if grid[y][x] == '+' {
            // change direction
            match direction {
                Down | Up => {
                    if grid_at(&grid, x + 1, y) != ' ' {
                        x = x + 1;
                        direction = Right;
                    } else if grid_at(&grid, x - 1, y) != ' ' {
                        x = x - 1;
                        direction = Left;
                    }
                }
                Left | Right => {
                    if grid_at(&grid, x, y + 1) != ' ' {
                        y = y + 1;
                        direction = Down;
                    } else if grid_at(&grid, x, y - 1) != ' ' {
                        y = y - 1;
                        direction = Up;
                    }
                }
            }
        } else {
            if !(grid[y][x] == '|' || grid[y][x] == '-') {
                if grid[y][x].is_alphabetic() {
                    print!("{}", grid[y][x]);
                } else {
                    break;
                }
            }

            match direction {
                Down => y = y + 1,
                Up => y = y - 1,
                Left => x = x - 1,
                Right => x = x + 1,
            }
        }
    }

    println!();
    println!("{} steps", steps - 1);
}
