use std::fs::File;
use std::io::prelude::*;

fn score_groups(value: &Vec<u8>, start: usize, score_step: u32) -> (u32, u32, usize) {
    let mut garbage = false;
    let mut escape = false;

    let mut score = 0;
    let mut garbage_count = 0;
    let mut i = start;

    while i < value.len() {
        let v = value[i] as char;

        if v == '{' && !garbage {
            let (inner_score, inner_garbage, inner_position) = score_groups(value, i + 1, score_step + 1);

            score += inner_score;
            garbage_count += inner_garbage;
            i = inner_position;
        } else if v == '<' && !garbage {
            // start of garbage
            garbage = true;
        } else if v == '>' && !escape {
            // end of garbage (or not garbage)
            garbage = false;
        } else if v == '}' && !garbage {
            score += score_step;
            break;
        } else if garbage && !escape && v != '!' {
            garbage_count += 1;
        }

        escape = v == '!' && garbage && !escape;
        i += 1;
    }

    (score, garbage_count, i)
}

fn main() {
    let input = "day9.in";
    let mut file = File::open(input).expect("Unable to open file day9.in");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);

    let result = score_groups(&data.as_bytes().to_vec(), 0, 0);
    println!("{:?}", result.0);
    println!("{:?}", result.1);
}
