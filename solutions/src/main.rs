use std::io;

use math_problems;

fn parse_u64_input() -> Result<Vec<u64>, std::io::Error> {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;
    let results = input_buffer
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    Ok(results)
}

fn main() {
    println!("Input 2 natural numbers to calculate GCD:");
    let input_vec = parse_u64_input().expect("Error parsing input");
    match math_problems::greatest_common_divisor(&input_vec) {
        Ok(result) => {
            println!("GCD is {}", result);
        }
        Err(err) => {
            println!("Something went wrong with input: {}", err);
        }
    }
}
