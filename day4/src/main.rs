use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn is_valid(value: &str) -> bool {
    if value.len() < 1 {
        return false;
    }

    let mut words = HashSet::new();
    let word_list: Vec<&str> = value.split(' ').collect();

    for word in word_list {
        if words.contains(word) {
            return false;
        } else {
            words.insert(word);
        }
    }

    return true;
}

fn is_valid2(value: &str) -> bool {
    if value.len() < 1 {
        return false;
    }

    let mut words = HashSet::new();
    let word_list: Vec<&str> = value.split(' ').collect();

    for word in word_list {
        let mut word_v: Vec<char> = word.chars().collect();
        word_v.sort_by(|a, b| b.cmp(a));
        let word_s = String::from_iter(word_v.iter());

        if words.contains(&word_s) {
            return false;
        } else {
            words.insert(word_s);
        }
    }

    return true;
}

fn main() {
    let input = "day4.in";

    let mut file = File::open(input).expect("Unable to open file day4.in");
    let mut data = String::new();
    
    let _ = file.read_to_string(&mut data);
    let lines: Vec<&str> = data.split('\n').collect();

    let answer: usize = lines
        .iter()
        .filter(|x| is_valid(x))
        .count();
    println!("{}", answer);

    let answer2: usize = lines
        .iter()
        .filter(|x| is_valid2(x))
        .count();
    println!("{}", answer2);
}

