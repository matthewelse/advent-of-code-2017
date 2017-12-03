use std::collections::HashMap;

fn location(input: i32) -> i32 {
    // find the ring to search through
    let input_f = input as f64;
    let result = ((input_f.sqrt() - 1.0) / 2.0).ceil();

    let px = result as i32;
    let py = 1 - px;

    let mut x = px;
    let mut y = py;

    let mut n = (px * 2 - 1) * (px * 2 - 1) + 1;

    while n < input {
        if y < px && x == px {
            y = y + 1;
        } else if y == px && x > -px {
            x = x - 1;
        } else if x == -px && y > -px {
            y = y - 1;
        } else if y == -px && x < px {
            x = x + 1;
        } else {
            y = y + 1;
        }

        n = n + 1;
    }

    x.abs() + y.abs()
}

fn value_at(map: &HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
    let mut val = 0;

    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 {
                continue
            }

            let pos: (i32, i32) = (x + dx, y + dy);
            val = val + match map.get(&pos) {
                Some(v) => *v,
                _ => 0
            };
        }
    }

    val
}

fn part2(limit: i32) -> i32 {
    let mut max = 1;
    let mut distance = 1;
    let mut data = HashMap::new();

    let mut x = 1;
    let mut y = 0;

    data.insert((0, 0), 1);

    while max < limit {
        let val = value_at(&data, x, y);
        data.insert((x, y), val);

        if y < 0 && x > 0 && x == -y {
            distance += 1;
            x += 1;
        } else if x == distance && y < distance {
            y += 1;
        } else if y == distance && x > -distance {
            x -= 1;
        } else if x == -distance && y > -distance {
            y -= 1;
        } else {
            x += 1;
        }

        if val > max {
            max = val;
        }
    }

    max
}

fn main() {
    // part 1
    println!("{}", location(265149));

    // part 2 
    println!("{}", part2(265149))
}
