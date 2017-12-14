use std::collections::HashSet;

fn reverse(start: usize, length: usize, numbers: &mut Vec<u32>) {
    for i in 0..(length / 2) {
        let j = (i + start) % numbers.len();
        let k = ((length - i - 1) + start) % numbers.len();

        let tmp = numbers[j];
        numbers[j] = numbers[k];
        numbers[k] = tmp;
    }
}

fn round(data: Vec<u32>,
         lengths: &Vec<usize>,
         position: &mut usize,
         skip_size: &mut usize)
         -> Vec<u32> {
    let mut numbers: Vec<u32> = data.to_vec();

    for length in lengths.iter() {
        if *length > 0 {
            reverse(*position, *length, &mut numbers);
        }

        *position += length;
        *position += *skip_size;

        *position %= numbers.len();

        *skip_size += 1;
    }

    numbers
}

fn knot_hash(input_text: &str) -> Vec<u32> {
    let extension: Vec<usize> = vec![17, 31, 73, 47, 23];
    let input = {
        let input_s = String::from(input_text);
        let mut input_i: Vec<usize> = input_s
            .into_bytes()
            .iter()
            .map(|x| *x as usize)
            .collect();
        input_i.extend(extension.iter());
        input_i
    };

    let mut position = 0;
    let mut skip_size = 0;
    let mut numbers: Vec<u32> = (0..256).collect();

    for _ in 0..64 {
        numbers = round(numbers, &input, &mut position, &mut skip_size);
    }

    let mut out: Vec<u32> = vec![0; 16];

    for i in 0..16 {
        for j in 0..16 {
            out[i] ^= numbers[i * 16 + j];
        }
    }

    out
}

fn line_bits(code: &str, line: usize) -> u32 {
    let code = format!("{}-{}", code, line);
    let hash = knot_hash(&code);

    let mut out = 0; 
    for byte in hash {
        out += byte.count_ones();
    }

    out
}

fn make_grid(code: &str) -> HashSet<(i32, i32)> { 
    let mut out = HashSet::new(); 

    for line in 0..128 {
        let code = format!("{}-{}", code, line);
        let hash = knot_hash(&code);

        for j in 0..16 {
            let byte = hash[j] as u8;
            for i in 0..8 {
                let mask = 1 << i;

                if byte & mask > 0 {
                    out.insert((line, (j*8 + 7 - i) as i32));
                }
            }
        }
    }

    out
}

fn children_of(grid: &HashSet<(i32, i32)>, start: (i32, i32), results: &mut HashSet<(i32, i32)>) {
    let (x, y) = start;

    for dx in -1..2 {
        if dx == 0 {
            continue
        }

        let pos = (x + dx, y);

        if grid.contains(&pos) {
            if !results.contains(&pos) {
                results.insert(pos);
                children_of(grid, pos, results);
            }
        }
    }

    for dy in -1..2 {
        if dy == 0 {
            continue
        }

        let pos = (x, y + dy);

        if grid.contains(&pos) {
            if !results.contains(&pos) {
                results.insert(pos);
                children_of(grid, pos, results);
            }
        }
    }
}


fn main() {
    let code = "wenycdww";
    let result: u32 = (0..128).map(|x| line_bits(code, x)).sum();

    println!("{}", result);

    // make a grid 
    let mut grid = make_grid(code);
    let mut count = 0;

    while grid.len() > 0 {
        let smallest: (i32, i32) = *grid.iter().min().expect("not found");
        let mut result = HashSet::new();

        result.insert(smallest);
        children_of(&grid, smallest, &mut result);

        grid = &grid - &result;
        count += 1;
    }

    println!("{}", count);
}
