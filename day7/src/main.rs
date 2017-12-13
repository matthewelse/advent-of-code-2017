use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn parse_line(line: &str) -> (&str, (i32, Vec<&str>)) {
    let arrow_split: Vec<&str> = line.split(" -> ").collect();

    let children: Vec<&str> = {
        if arrow_split.len() == 2 {
            arrow_split[1].split(", ").collect()
        } else {
            vec![]
        }
    };

    let l = arrow_split[0].trim_right_matches(')');
    let parts: Vec<&str> = l.split(" (").collect();

    let root = parts[0];
    let weight = parts[1].parse::<i32>().expect("No weight found");

    (root, (weight, children))
}

fn total_weight(root: &str, connections: &HashMap<&str, (i32, Vec<&str>)>) -> i32 {
    let (w, ref children) = *&connections[root];

    let s: i32 = children
        .iter()
        .map(|x| total_weight(x, connections))
        .sum();

    s + w
}

fn compute_weights(root: &str,
                   connections: &HashMap<&str, (i32, Vec<&str>)>,
                   memos: &mut HashMap<&str, i32>) {
    if !memos.contains_key(&root) {
        let (w, ref children) = connections[root];
        let mut total = 0;

        for child in children.iter() {
            if !memos.contains_key(child) {
                compute_weights(child, connections, memos);
            }

            total += memos.get(child).expect("No value found for child");
        }

        memos.insert(root, total);
    }
}

fn find_wrong_weight<'a>(root: &str,
                     connections: &HashMap<&str, (i32, Vec<& 'a str>)>,
                     cumulative_weights: &HashMap<&str, i32>)
                     -> Option<& 'a str> {
    let (w, ref children) = *connections
        .get(root)
        .expect("root not found in connections");
    let wc = cumulative_weights
        .get(root)
        .expect("root not found in cumulative_weights");

    if children.len() < 2 {
        None
    } else if children.len() == 2 {
        let l = find_wrong_weight(children[0], connections, cumulative_weights);
        let r = find_wrong_weight(children[1], connections, cumulative_weights);

        match l {
            Some(x) => l,
            None => {
                match r {
                    Some(x) => r,
                    None => None,
                }
            }
        }
    } else {
        // find the one different cumulative weight below

        let mut value1 = None;
        let mut key1 : Option<&str> = None;
        let mut count1 = 0;
        let mut value2 = None;
        let mut key2 : Option<&str> = None;
        let mut count2 = 0;

        for child in children {
            let w = cumulative_weights.get(child).expect("Child not found");

            match value1 {
                Some(x) => {
                    if x == w {
                        count1 += 1;
                    } else {
                        match value2 {
                            Some(y) => {
                                if y == w {
                                    count2 += 1;
                                } else {
                                    panic!("invalid");
                                }
                            }
                            None => {
                                value2 = Some(w);
                                key2 = Some(child);
                                count2 = 1;
                            }
                        }
                    }
                },
                None => {
                    count1 = 1;
                    value1 = Some(w);
                    key1 = Some(child);
                }
            }
        }

        if count1 == 1 {
            key1
        } else {
            key2
        }
    }
}

fn main() {
    let input = "day7.in";
    let mut file = File::open(input).expect("Unable to open file day7.in");
    let mut data = String::new();

    let _ = file.read_to_string(&mut data);

    let connections: HashMap<&str, (i32, Vec<&str>)> = data.split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| parse_line(x))
        .collect();

    // Find the root. There's probably a less memory-intensive way of dealing with this, but oh
    // well :)
    let root = {
        let mut nodes: HashSet<&str> = HashSet::new();
        let mut inner: HashSet<&str> = HashSet::new();

        for (k, v) in &connections {
            let (_w, ref children) = *v;
            nodes.insert(k);

            for child in children.iter() {
                inner.insert(child);
            }
        }

        let roots = &nodes - &inner;
        roots.iter().next().expect("no root found").to_owned()
    };

    println!("{}", root);

    // Find the incorrect weight.
    let mut cumulative_weights: HashMap<&str, i32> = HashMap::new();
    compute_weights(&root, &connections, &mut cumulative_weights);

    println!("{:?}", cumulative_weights);
}
