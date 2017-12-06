use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn argmax(vector: &Vec<usize>) -> (usize, usize) {
    vector
        .iter()
        .zip(0..)
        .fold((0, vector[0]), |last, this| {
            let (i, val) = last;
            let (new, j) = this;

            if *new > val { (j, *new) } else { (i, val) }
        })
}

/**
 * It'd be nice to speed this up using a priority queue, but for the time-being,
 * this should be enough to solve the problem pretty quickly.
 *
 * Using a binary queue would probably involve learning how to use Box<T>, so
 * that could be fun.
 */
fn step(banks: &Vec<usize>) -> Vec<usize> {
    let mut out = banks.to_vec();
    let (ix, max) = argmax(banks);
    out[ix] = 0;

    for i in 0..banks.len() {
        let j = (i + ix + 1) % banks.len();
        let gain = max / banks.len();

        if i < max % banks.len() {
            out[j] += gain + 1;
        } else {
            out[j] += gain;
        }
    }

    // println!("{:?} -> {:?}", banks, out);
    out
}

fn solve(start: Vec<usize>) -> (i32, i32) {
    let mut visited = HashMap::new();
    let mut next = start.to_vec();
    let mut steps = 0;

    while !visited.contains_key(&next) {
        visited.insert(next.to_vec(), steps);
        next = step(&next);
        steps += 1;
    }

    let previous = visited
        .get(&next)
        .expect("Unable to find pre-existing state");

    (steps, steps - *previous)
}

fn main() {
    println!("Example: {:?}", part1(vec![0, 2, 7, 0]));

    // Load sample data for day6
    let input = "day6.in";
    let mut file = File::open(input).expect("Unable to open file day6.in");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);

    let start: Vec<usize> = data.split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!("Both Parts: {:?}", solve(start));
}
