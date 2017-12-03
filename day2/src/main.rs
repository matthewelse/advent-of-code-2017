use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = "day2.in";

    let mut file = File::open(input).expect("Unable to open file day2.in");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);
    let lines: Vec<&str> = data.split('\n').collect();

    let answer: i32 = lines
        .iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.split(' '))
        .map(|l| l.map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|l: Vec<i32>| {
                 let m = l.iter().max().unwrap();
                 let n = l.iter().min().unwrap();
                 m - n
             })
        .sum();

    println!("{:?}", answer);

    let answer2: i32 = lines
        .iter()
        .filter(|s| s.len() > 0)
        .map(|s| s.split(' '))
        .map(|l| l.map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|l: Vec<i32>| {
            for i in l.iter() {
                for j in l.iter() {
                    if i != j && i % j == 0 {
                        return i / j;
                    }
                }

            }
            return 0;
        })
        .sum();

    println!("{:?}", answer2);
}
