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

fn main() {
    // Part 1
    {
        let lengths: Vec<usize> = vec![14, 58, 0, 116, 179, 16, 1, 104, 2, 254, 167, 86, 255, 55,
                                       122, 244];
        let mut position = 0;
        let mut skip_size = 0;
        let numbers: Vec<u32> = (0..256).collect();
        let hash = round(numbers, &lengths, &mut position, &mut skip_size);
        let result = hash[0] * hash[1];
        println!("{}", result);
    }

    // Part 2
    {
        let extension: Vec<usize> = vec![17, 31, 73, 47, 23];
        let input = {
            let input_s = String::from("14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244");
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

        let out_s: Vec<String> = out.iter().map(|x| format!("{:02x}", x)).collect();
        println!("{}", out_s.join(""));
    }
}
