use std::fs::File;
use std::io::prelude::*;

const W: f64 = 1.0;
const WH: f64 = 1.73205080757 / 2.0;
const H: f64 = W * WH;

fn direction(dir: &str) -> (f64, f64) {
    match dir {
        "n" => (0.0, H),
        "ne" => (W, H / 2.0),
        "se" => (W, -H / 2.0),
        "s" => (0.0, -H),
        "sw" => (-W, -H / 2.0),
        "nw" => (-W, H / 2.0),
        _ => {
            println!("dir: {}", dir);
            panic!("invalid direction")
        },
    }
}

fn steps(position: &(f64, f64)) -> i64 {
    // based on the quadrant, we can figure out how many of each
    let horizontal = position.0 / W;

    // if horizontal is negative, it's [NS]W, otherwise it's [NS]E
    // if position.1 < 0 then S[EW] otherwise, N[EW]

    // i.e. take horizontal.abs() steps towards y=0
    let vertical = if position.1 < 0.0 {
        (position.1 + horizontal.abs() * H / 2.0) / H
    } else {
        (position.1 - horizontal.abs() * H / 2.0) / H
    };

    let answer = (vertical.abs() + horizontal.abs()).round() as i64;

    answer
}

fn main() {
    let input = "day11.in";
    let mut file = File::open(input).expect("Unable to open file day11.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let parts: Vec<&str> = data.trim().split(',').collect();

    let mut position = (0.0, 0.0);
    let mut max_distance = 0;

    for step in parts {
        let dir = direction(step);

        position = (position.0 + dir.0, position.1 + dir.1);

        let new_distance = steps(&position);

        if new_distance > max_distance {
            max_distance = new_distance; 
        }
    }

    println!("{}", steps(&position));
    println!("{}", max_distance);
}
