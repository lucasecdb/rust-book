use std::collections::HashMap;
use std::io::prelude::*;
use std::io;

fn main() {
    println!("Type the number of numbers");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Unknown error occurred");

    let n = n.trim().parse().expect("Invalid number");

    let mut numbers = Vec::new();

    let mut total = 0;
    let mut ocurrence_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..n {
        loop {
            let mut num = String::new();

            print!("Type number nº {}: ", i + 1);

            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut num)
                .expect("Unknown error occurred");

            let num: i32 = match num.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number");
                    continue;
                }
            };

            total += num;
            numbers.push(num);
            let count = ocurrence_map.entry(num).or_insert(0);

            *count += 1;

            break;
        }
    }

    let mean = f64::from(total) / f64::from(n as i32);

    numbers.sort();

    let median = if n % 2 == 0 {
        let middle_left = n / 2 - 1;
        let middle_right = n / 2;

        (numbers[middle_left] + numbers[middle_right]) / 2
    } else {
        let middle = n / 2;

        numbers[middle]
    };

    let mut mode: (i32, i32) = (0, 0);

    for (num, frequency) in ocurrence_map {
        if frequency > mode.1 {
            mode.0 = num;
            mode.1 = frequency;
        }
    }

    let mode = mode.0;

    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Mode: {}", mode);
}
