fn state(buffer: &Vec<usize>, position: usize) -> String {
    let mut out: Vec<String> = vec![];

    for i in 0..buffer.len() {
        if i == position {
            out.push(format!("({})", buffer[i]));
        } else {
            out.push(buffer[i].to_string());
        }
    }

    out.join(" ")
}

fn part2(step: u64) -> u64 {
    let mut position: u64 = 0;
    let mut ans = 0;

    for length in 1..50_000_001 {
        position += step;
        position %= length;

        if position == 0 {
            ans = length;
        }

        position += 1;
    }

    ans 
}

fn main() {
    let mut buffer: Vec<usize> = Vec::with_capacity(2017);
    buffer.push(0);

    let step = 371;
    let mut position = 0;
    
    for i in 1..2018 {
        position += step;
        position %= buffer.len();

        buffer.insert(position + 1, i);

        position += 1;
    }

    println!("{}", buffer[position + 1]);
    println!("{}", part2(step as u64));
}
