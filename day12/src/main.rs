use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn children_of(connections: &Vec<HashSet<usize>>, start: usize, results: &mut HashSet<usize>) {
    for child in &connections[start] {
        if !results.contains(&child) {
            results.insert(*child);
            children_of(connections, *child, results);
        }
    }
}

fn main() {
    // Compute the symmetric, transitive closure of the input file
    let input = "day12.in";
    let mut file = File::open(input).expect("Unable to open file day12.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let mut connections = vec![HashSet::new(); 2000];

    // Preprocess the data, then iterate over it and add all relevant connections.
    data.trim()
        .split('\n')
        .filter(|x| x.len() > 0)
        .for_each(|x| {
            let line_parts: Vec<&str> = x.split(" <-> ").collect();
            let root = line_parts[0]
                .parse::<usize>()
                .expect("Invalid root number");

            line_parts[1]
                .split(", ")
                .map(|x| x.parse::<usize>().expect("Invalid Number"))
                .for_each(|x| {
                              connections[root].insert(x);
                              connections[x].insert(root);
                          });
        });

    let mut programs: HashSet<usize> = (0..2000).collect();
    let mut count = 0;

    // Repeatedly find a group using the smallest program number left, and remove all elements
    // of that group from consideration
    while programs.len() > 0 {
        let smallest: usize = *programs.iter().min().unwrap_or(&0);

        let mut result = HashSet::new();
        children_of(&connections, smallest, &mut result);

        if smallest == 0 {
            println!("{:?}", result.len());
        }

        programs = &programs - &result;
        count += 1;
    }

    println!("{:?}", count);
}
