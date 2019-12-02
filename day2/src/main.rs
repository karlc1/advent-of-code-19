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

    let mut numbers: Vec<i32> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();

    let mut i = 0;

    loop {
        if numbers[i] == 99 || i >= numbers.len()  {
            break;
        }
        else if numbers[i] == 1 {
           let j = numbers[i+3];
           numbers[j as usize] = numbers[i+1] + numbers[i +2]; 
           // numbers[numbers[i+3]] = numbers[i+1] + numbers[i +2]; 
        }
        else if numbers[i] == 2 {
            let j = numbers[i+3];
            numbers[j as usize] = numbers[i+1] * numbers[i +2]; 
        }
        else {
            println!("Something went wrong, expected number 1, 2 or 99 but got {}", numbers[i]);
        }
        i += 4;
    }

    println!("Result: {}", numbers[0]);

    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    for line in input.lines() {
	// Implement solution for part 2 here <------------------
        let _remove_me = line; // remove this
    }
    Ok(())
}
