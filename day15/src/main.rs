const A_START: u64 = 679;
const B_START: u64 = 771;

const A_MODULUS: u64 = 4;
const B_MODULUS: u64 = 8;

const ITERS: u64 = 40_000_000;
const ITERS_2: u64 = 5_000_000;

#[inline]
fn step(input: u64, factor: u64) -> u64 {
    (input * factor) % 0x7FFF_FFFF
}

fn step_mult(input: u64, factor: u64, modulus: u64) -> u64 {
    let mut next = step(input, factor);

    while next % modulus != 0 {
        next = step(next, factor);
    }

    next
}

fn main() {
    let mut a = A_START;
    let mut b = B_START;

    let mut count = 0;

    for _ in 0..ITERS {
        a = step(a, A_START);
        b = step(b, B_START);

        if a & 0xFFFF == b & 0xFFFF {
            count += 1;
        }
    }

    println!("{}", count);

    a = A_START;
    b = B_START;

    count = 0;

    for _ in 0..ITERS_2 {
        a = step_mult(a, A_START, A_MODULUS);
        b = step_mult(b, B_START, B_MODULUS);

        if a & 0xFFFF == b & 0xFFFF {
            count += 1;
        }
    }

    println!("{}", count);
}
