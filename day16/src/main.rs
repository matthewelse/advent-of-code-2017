use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn reverse(line: &mut Vec<u8>, from: usize, length: usize) {
    let midpoint = length / 2;
    for i in 0..midpoint {
        let left = from + i;
        let right = length - i + from - 1;
        let tmp = line[left];

        line[left] = line[right];
        line[right] = tmp;
    }
}

fn spin(line: &mut Vec<u8>, count: usize) {
    let len = line.len();
    reverse(line, 0, len);
    reverse(line, 0, count);
    reverse(line, count, len - count);
}

fn exchange(line: &mut Vec<u8>, a: usize, b: usize) {
    let tmp = line[a];
    line[a] = line[b];
    line[b] = tmp;
}

fn partner(line: &mut Vec<u8>, a: u8, b: u8) {
    let mut tmp = None;
    let mut tmp_ix = None;

    for i in 0..line.len() {
        if line[i] == a || line[i] == b {
            match tmp {
                None => {
                    tmp = Some(line[i]);
                    tmp_ix = Some(i);
                }
                Some(x) => {
                    line[tmp_ix.unwrap()] = line[i];
                    line[i] = x;
                    return;
                }
            }
        }
    }
}

fn as_string(line: &Vec<u8>) -> String {
    String::from_utf8(line.iter().map(|x| x + 'a' as u8).collect()).unwrap()
}

const LINE_SIZE: u8 = 16;
const LINE_SIZE_S: usize = LINE_SIZE as usize;

fn main() {
    let mut line: Vec<u8> = (0..LINE_SIZE).collect();
    let input = "day16.in";
    let mut file = File::open(input).expect("Unable to open file day16.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let mut cycle_time = 0;
    let mut perms = vec![];
    perms.push(line.to_vec());

    for i in 0.. {
        data.trim()
            .split(',')
            .for_each(|x| {
                if x.starts_with("p") {
                    let parts: Vec<u8> = x[1..]
                        .split('/')
                        .map(|x| x.bytes().next().unwrap() - 'a' as u8)
                        .collect();

                    partner(&mut line, parts[0], parts[1]);
                } else if x.starts_with("s") {
                    let number: usize = x[1..].parse::<usize>().unwrap();

                    spin(&mut line, number);
                } else if x.starts_with("x") {
                    let numbers: Vec<usize> = x[1..]
                        .split('/')
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect();

                    exchange(&mut line, numbers[0], numbers[1]);
                };
            });


        if i == 0 {
            println!("{}", as_string(&line));
        }

        let mut cycle = true;

        for j in 0..LINE_SIZE_S {
            if line[j] != j as u8 {
                cycle = false;
                break;
            }
        }

        if cycle {
            println!("{}", i + 1);
            cycle_time = i + 1;
            break;
        } else {
            perms.push(line.to_vec());
        }
    }

    let which = 1_000_000 % cycle_time;
    println!("{}", as_string(&perms[which]));
}
