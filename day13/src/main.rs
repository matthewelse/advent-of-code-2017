use std::fs::File;
use std::io::prelude::*;

fn scan_pos(height: usize, time: usize) -> usize {
    let cycle_pos = time % (2 * height - 2);
    let inner_cycle_pos = cycle_pos % height;

    let scan = if cycle_pos >= height {
        height - inner_cycle_pos - 2
    } else {
        inner_cycle_pos
    };

    scan
}

fn severity(layers: &Vec<usize>, start_time: usize) -> usize {
    let mut score = 0;

    for delta in 0..layers.len() {
        let time = delta + start_time;

        if layers[delta] > 0 {
            // scan oscillates, rather than cycles
            // compute the scan position
            let scan = scan_pos(layers[delta], time);

            if scan == 0 {
                // println!("Caught at depth {} with range {}", layers[time], time);
                score += layers[delta] * time;
            }
        }
    }

    score
}

fn main() {
    let input = "day13.in";
    let mut file = File::open(input).expect("Unable to open file day13.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let mut layers = vec![0; 95];

    data.trim()
        .split('\n')
        .map(|x| {
                 let lr: Vec<usize> = x.split(": ")
                     .map(|x| x.parse::<usize>().expect("Invalid number"))
                     .collect();

                 (lr[0], lr[1])
             })
        .for_each(|x| layers[x.0] = x.1);

    println!("{}", severity(&layers, 0));

    // Finds the answer soon enough 
    let mut i = 0;
    loop {
        if severity(&layers, i) == 0 {
            println!("{}", i);
            break;
        }

        i += 1;
    }
}

