use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = "day1.in";

    let mut data = vec![];
    let mut file = File::open(input).expect("Unable to open file day1.in");

    let _ = file.read_to_end(&mut data);

    {
        let mut total = 0;

        for i in 0..data.len() {
            if data[i] < 48 {
                continue
            }

            let j: usize = data[i] as usize - 48;
            let k: usize = data[(i + 1) % (data.len() - 1)] as usize - 48;

            if j == k {
                total += j;
            }
        }

        println!("{}", total);
    }

    {
        let mut total = 0;
        let offset = data.len() / 2;

        for i in 0..data.len() {
            if data[i] < 48 {
                continue
            }

            let j: usize = data[i] as usize - 48;
            let k: usize = data[(i + offset) % (data.len() - 1)] as usize - 48;

            if j == k {
                total += j;
            }
        }

        println!("{}", total);
    }
}

