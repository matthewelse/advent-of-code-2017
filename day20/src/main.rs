use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

type V3 = (i64, i64, i64);

#[derive(Clone, Debug)]
struct ParticleState {
    x: V3,
    v: V3,
    a: V3,
}

fn parse_part(x: &str) -> V3 {
    // Parse `a=<x, y, z>`
    let last = &x[3..(x.len() - 1)];
    let parts: Vec<i64> = last.split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    (parts[0], parts[1], parts[2])
}

fn parse_line(x: &str) -> ParticleState {
    let parts: Vec<&str> = x.split(", ").collect();

    if parts.len() != 3 {
        panic!("Invalid number of parts for this particle.");
    }

    let vecs: Vec<V3> = parts.iter().map(|x| parse_part(x)).collect();
    ParticleState {
        x: vecs[0],
        v: vecs[1],
        a: vecs[2],
    }
}

fn vec_add(x: &V3, y: &V3) -> V3 {
    (x.0 + y.0, x.1 + y.1, x.2 + y.2)
}

fn step(x: &ParticleState) -> ParticleState {
    let mut new = x.clone();

    new.v = vec_add(&x.a, &x.v);
    new.x = vec_add(&new.v, &x.x);

    new
}

fn manhattan_distance(x: V3) -> i64 {
    x.0.abs() + x.1.abs() + x.2.abs()
}

fn remove_step(particles: Vec<ParticleState>) -> Vec<ParticleState> {
    let mut used_particles = HashMap::new();
    let mut delete_particles = HashSet::new();

    for p in particles.iter() {
        if used_particles.contains_key(&p.x) {
            used_particles.remove(&p.x);
            delete_particles.insert(p.x);
        } else if !delete_particles.contains(&p.x) {
            used_particles.insert(p.x, p);
        }
    }

    used_particles.values().map(|x| step(x)).collect()
}

fn main() {
    let input = "day20.in";
    let mut file = File::open(input).expect("Unable to open file day20.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let particles: Vec<ParticleState> = data.trim()
        .split('\n')
        .map(|x| parse_line(x))
        .collect();

    let part1: usize = particles
        .iter()
        .map(|x| manhattan_distance(x.a))
        .zip(0..)
        .min()
        .unwrap()
        .1;

    println!("{:?}", part1);

    let mut particles_new = remove_step(particles);
    for _ in 0..1000 {
        particles_new = remove_step(particles_new);
    }
    println!("{}", particles_new.len());
}
