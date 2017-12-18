use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

// For Part II
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::{SyncSender, Receiver};
use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
enum Argument {
    Reg(char),
    Lit(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop,
    Snd(Argument),
    Rcv(Argument),
    Set(Argument, Argument),
    Add(Argument, Argument),
    Mul(Argument, Argument),
    Mod(Argument, Argument),
    Jgz(Argument, Argument),
}

use self::Instruction::*;
use self::Argument::*;

fn parse_arg(arg: &str) -> Argument {
    let first = arg.chars().next().unwrap();

    if first.is_digit(10) || first == '-' {
        Argument::Lit(arg.parse::<i64>().unwrap())
    } else {
        Argument::Reg(first)
    }
}

fn parse_inst(instruction: &str) -> Instruction {
    let parts: Vec<&str> = instruction.split(' ').collect();

    let op = parts[0];

    let r1 = parts[1];
    let arg1 = parse_arg(r1);

    if parts.len() == 2 {
        match op {
            "snd" => Instruction::Snd(arg1),
            "rcv" => Instruction::Rcv(arg1),
            _ => Instruction::Nop,
        }
    } else if parts.len() == 3 {
        let r2 = parts[2];
        let arg2 = parse_arg(r2);

        match op {
            "set" => Instruction::Set(arg1, arg2),
            "add" => Instruction::Add(arg1, arg2),
            "mul" => Instruction::Mul(arg1, arg2),
            "mod" => Instruction::Mod(arg1, arg2),
            "jgz" => Instruction::Jgz(arg1, arg2),
            _ => Instruction::Nop,
        }
    } else {
        Instruction::Nop
    }
}

fn eval_arg(arg: &Argument, registers: &mut HashMap<char, i64>) -> i64 {
    match arg {
        &Reg(ref x) => *registers.get(x).unwrap_or(&0),
        &Lit(x) => x,
    }
}

fn eval_instruction(inst: &Instruction,
                    registers: &mut HashMap<char, i64>,
                    pc: i64,
                    last_sound: i64)
                    -> (i64, Option<i64>) {
    let mut sound = None;

    match inst {
        &Snd(ref a1) => {
            sound = Some(eval_arg(a1, registers));
        }
        &Rcv(ref a1) => {
            if eval_arg(a1, registers) != 0 {
                sound = Some(last_sound);
            }
        }
        &Set(ref r1, ref val) => {
            if let &Reg(rc) = r1 {
                let value = eval_arg(val, registers);
                registers.insert(rc, value);
            }
        }
        &Add(ref r1, ref val) => {
            if let &Reg(rc) = r1 {
                let v1 = eval_arg(val, registers);
                let v2 = *registers.get(&rc).unwrap_or(&0);
                registers.insert(rc, v1 + v2);
            }
        }
        &Mul(ref r1, ref val) => {
            if let &Reg(rc) = r1 {
                let v1 = eval_arg(val, registers);
                let v2 = *registers.get(&rc).unwrap_or(&0);
                registers.insert(rc, v1 * v2);
            }
        }
        &Mod(ref r1, ref val) => {
            if let &Reg(rc) = r1 {
                let v1 = eval_arg(val, registers);
                let v2 = *registers.get(&rc).unwrap_or(&0);
                registers.insert(rc, v2 % v1);
            }
        }
        _ => (),
    }

    if let &Jgz(ref a1, ref a2) = inst {
        let x = eval_arg(a1, registers);
        let y = eval_arg(a2, registers);

        if x > 0 {
            (pc + y, sound)
        } else {
            (pc + 1, sound)
        }
    } else {
        (pc + 1, sound)
    }
}

fn spawn_thread(instructions: Vec<Instruction>,
                id: i64,
                tx: SyncSender<i64>,
                rx: Receiver<i64>)
                -> JoinHandle<i64> {
    thread::spawn(move || {
        let mut registers = HashMap::new();
        registers.insert('p', id);
        let mut pc = 0;

        let mut inst_count = 0;
        let inst_limit = 100_000_000;

        let mut sent = 0;

        while pc < instructions.len() && inst_count < inst_limit {
            let instruction = &instructions[pc];

            if let &Rcv(ref x) = instruction {
                if let &Reg(rc) = x {
                    let val = rx.recv_timeout(Duration::from_secs(1));

                    if let Ok(val_raw) = val {
                        registers.insert(rc, val_raw);
                    } else {
                        // return number of sent values
                        break;
                    }
                }
                pc += 1;
            } else if let &Snd(ref x) = instruction {
                let val = eval_arg(x, &mut registers);

                sent += 1;
                tx.send(val).unwrap();
                pc += 1;
            } else {
                let res = eval_instruction(&instruction, &mut registers, pc as i64, 0);
                pc = res.0 as usize;
            }

            inst_count += 1;
        }

        sent
    })
}

fn main() {
    let input = "day18.in";
    let mut file = File::open(input).expect("Unable to open file day18.in");

    let mut data = String::new();
    let _ = file.read_to_string(&mut data);

    let instructions: Vec<Instruction> = data.trim()
        .split('\n')
        .map(|x| parse_inst(x))
        .collect();

    let mut registers = HashMap::new();
    let mut last_sound = 0;
    let mut pc = 0;
    let mut inst_count = 0;
    let instruction_limit = 100_000;

    while pc < instructions.len() && inst_count < instruction_limit {
        let instruction = &instructions[pc];
        let res = eval_instruction(&instruction, &mut registers, pc as i64, last_sound);

        pc = res.0 as usize;

        if let Some(x) = res.1 {
            last_sound = x;

            if let &Rcv(_) = instruction {
                println!("Part 1: {}", x);
                break;
            }
        }

        inst_count += 1;
    }

    // Part 2:
    // I'm going to use this as an excuse to practice programming with channels
    // in rust. Set up two threads with a FIFO channel from each one to the other

    let (tx1, rx1): (SyncSender<i64>, Receiver<i64>) = sync_channel(10000);
    let (tx2, rx2): (SyncSender<i64>, Receiver<i64>) = sync_channel(10000);

    // Spawn two threads
    let t1 = spawn_thread(instructions.to_vec(), 0, tx1, rx2);
    let t2 = spawn_thread(instructions.to_vec(), 1, tx2, rx1);

    let _ = t1.join();
    let t2_sent = t2.join();

    println!("Part 2: {:?}", t2_sent.unwrap());
}
