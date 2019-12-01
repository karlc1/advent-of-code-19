use std::env;
use std::fs;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    let day_arg = &args[1];
    let part_arg = &args[2];

    let input = fs::read_to_string("./input/input.txt")
        .expect("Something went wrong reading the input file");

    if part_arg == "1" {
        println!("Running day {} part 1...", day_arg);
        part1(&input)?;
    }

    if part_arg == "2" {
        println!("Running day {} part 2...", day_arg);
        part2(&input)?;
    }

    Ok(())
}

fn part1(input: &str) -> Result<(), ()> {
    let mut total = 0;

    for line in input.lines() {
        let parsed = line.parse::<i32>();
        let input = match parsed {
            Ok(i) => i,
            Err(_e) => continue,
        };

        total += input / 3 - 2;
    }
    println!("Result: {}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    let mut total = 0;

    for line in input.lines() {
        let parsed = line.parse::<i32>();
        let input = match parsed {
            Ok(i) => i,
            Err(_e) => continue,
        };

        total += get_fuel_consumption(input, 0);
    }
    println!("Result: {}", total);
    Ok(())
}

fn get_fuel_consumption(remaining_mass: i32, total_fuel: i32) -> i32 {
    if remaining_mass / 3 - 2 <= 0 {
        return total_fuel;
    }
    return get_fuel_consumption(remaining_mass / 3 - 2, total_fuel + remaining_mass / 3 - 2);
}
