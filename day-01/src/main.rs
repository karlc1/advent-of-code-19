use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
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

    println!("Result day 1: {}", total);

    Ok(())
}
