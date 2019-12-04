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

    numbers[1] = 12;
    numbers[2] = 2;

    let mut i = 0usize;

    loop {
        if i >= numbers.len() - 1{
            break;
        }
        let operator = numbers[i];

        if operator == 99 {
            break;
        }

        let operand_1_pos = numbers[i + 1] as usize;
        let operand_2_pos = numbers[i + 2] as usize;
        let target_pos = numbers[i + 3] as usize;

        if operator == 1 {
           numbers[target_pos] = numbers[operand_1_pos] + numbers[operand_2_pos]; 
        }
        if operator == 2 {
           numbers[target_pos] = numbers[operand_1_pos] * numbers[operand_2_pos]; 
        }
        i += 4;
    }
    println!("Result: {}", numbers[0]);
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    
    let input_s: Vec<i32> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();

    for noun in 0..input_s.len() {
        for verb in 0..input_s.len() {

            let mut numbers: Vec<i32> = input.split(",").map(|x| x.trim().parse().unwrap()).collect();
            numbers[1] = noun as i32;
            numbers[2] = verb as i32;
            let mut i = 0usize;

            loop {
                if i >= numbers.len() - 1{
                    break;
                }
                let operator = numbers[i];

                if operator == 99 {
                    break;
                }

                let operand_1_pos = numbers[i + 1] as usize;
                let operand_2_pos = numbers[i + 2] as usize;
                let target_pos = numbers[i + 3] as usize;

                if operator == 1 {
                   numbers[target_pos] = numbers[operand_1_pos] + numbers[operand_2_pos]; 
                }
                if operator == 2 {
                   numbers[target_pos] = numbers[operand_1_pos] * numbers[operand_2_pos]; 
                }

                if numbers[0] == 19690720 {
                    println!("Result: {}", 100 * noun + verb);
                    return Ok(());
                }

                i += 4;
            }
        }
    }
    Ok(())
}

