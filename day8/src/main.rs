use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Operator {
    GreaterThan,
    GreaterThanEq,
    Equal,
    LessThan,
    LessThanEq,
    NotEqual,
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: Operator,
    literal: i32,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    increase: i32,

    condition: Condition,
}

fn check_condition(registers: &HashMap<String, i32>, condition: &Condition) -> bool {
    let value = *registers.get(&condition.register).unwrap_or(&0);

    match condition.operator {
        Operator::GreaterThan => value > condition.literal,
        Operator::GreaterThanEq => value >= condition.literal,
        Operator::Equal => value == condition.literal,
        Operator::LessThan => value < condition.literal,
        Operator::LessThanEq => value <= condition.literal,
        Operator::NotEqual => value != condition.literal,
    }
}

fn step(registers: &mut HashMap<String, i32>, instruction: &Instruction) {
    let value = *registers.get(&instruction.register).unwrap_or(&0);

    // check condition
    if check_condition(registers, &instruction.condition) {
        registers.insert(instruction.register.to_string(), value + instruction.increase);
    }
}

fn parse_condition(line: &str) -> Condition {
    let parts : Vec<&str> = line.split(' ').collect();

    let register = String::from(parts[0]);
    let literal = parts[2].parse::<i32>().expect("Invalid condition literal"); 
    let operator = match parts[1] {
        ">" => Operator::GreaterThan,
        ">=" => Operator::GreaterThanEq,
        "==" => Operator::Equal,
        "<" => Operator::LessThan,
        "<=" => Operator::LessThanEq,
        "!=" => Operator::NotEqual,
        _ => panic!("Invalid operator")
    };

    Condition { register: register, literal: literal, operator: operator }
}

fn parse_instruction(line: &str) -> (String, i32) {
    let parts : Vec<&str> = line.split(' ').collect();

    let register = parts[0];
    let sign = match parts[1] {
        "inc" => 1,
        "dec" => -1,
        _ => panic!("Invalid operator")
    };
    let value = sign * parts[2].parse::<i32>().expect("Invalid instruction");

    (String::from(register), value)
}

fn parse_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" if ").collect();

    let left = parts[0];
    let right = parts[1];

    let condition = parse_condition(right);
    let (register, increase) = parse_instruction(left);

    Instruction {
        register: register, 
        increase: increase,
        condition: condition 
    }
}

fn main() {
    let mut registers = HashMap::new();
    let mut max = 0;

    let input = "day8.in";
    let mut file = File::open(input).expect("Unable to open file day8.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let _ = data.split('\n')
        .filter(|x| x.len() > 0)
        .map(|x| parse_line(x))
        .for_each(|x| {
            step(&mut registers, &x);

            let value = registers.get(&x.register).unwrap_or(&0);

            if *value > max {
                max = *value;
            }
        });

    let answer = registers.values().max();
    println!("{}", answer.unwrap_or(&0));
    println!("{}", max);
}
