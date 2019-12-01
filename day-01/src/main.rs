use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("Result for part 1 is: {}", part1()?);
    println!("Result for part 2 is: {}", part2()?);
    return Ok(());
}

fn part1() -> io::Result<i32> {
    let path = Path::new("./input/input1.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let input_str = match line {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let parsed = input_str.parse::<i32>();

        let input = match parsed {
            Ok(i) => i,
            Err(_e) => continue,
        };
        total += (input / 3) - 2;
    }

    Ok(total)
}

fn part2() -> io::Result<i32> {
    let path = Path::new("./input/input1.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let input_str = match line {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let parsed = input_str.parse::<i32>();

        let input = match parsed {
            Ok(i) => i,
            Err(_e) => continue,
        };
        total += get_fuel_consumption(input, 0)
    }

    Ok(total)
}

fn get_fuel_consumption(remaining_mass: i32, total_fuel: i32) -> i32 {
    if remaining_mass / 3 - 2 <= 0 {
        return total_fuel;
    }
    return get_fuel_consumption(remaining_mass / 3 - 2, total_fuel + remaining_mass / 3 - 2);
}
