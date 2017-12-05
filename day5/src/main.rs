use std::fs::File;
use std::io::prelude::*;

fn part1(start: &Vec<i32>) -> u32 {
    let mut jumps = start.to_vec();
    let mut position: i32 = 0;
    let mut count = 0;

    while position >= 0 && position < jumps.len() as i32 {
        let i = position as usize;
        let offset = jumps[i];

        position += offset;
        jumps[i] += 1;
        count += 1;
    }

    count
}

fn part2(start: &Vec<i32>) -> u32 {
    let mut jumps = start.to_vec();
    let mut position: i32 = 0;
    let mut count = 0;

    while position >= 0 && position < jumps.len() as i32 {
        let i = position as usize;
        let offset = jumps[i];

        position += offset;

        if offset < 3 {
            jumps[i] += 1;
        } else {
            jumps[i] -= 1;
        }
        count += 1;
    }

    count
}

fn main() {
    let input = "day5.in";

    let mut file = File::open(input).expect("Unable to open file day5.in");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let jumps: Vec<i32> = data.split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("{}", part1(&jumps));
    println!("{}", part2(&jumps));
}

