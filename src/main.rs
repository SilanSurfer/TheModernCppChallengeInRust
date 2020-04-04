use std::io;

fn sum_of_naturals_div_by_3_and_5() {
    println!("This function will calculate the sum of numbers up
    to limit that are divisible by 3 and 5");
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read input line");
    let upper_limit = input_buffer.trim().parse::<u64>().unwrap();

    let mut sum = 0;
    for no in 1..upper_limit {
        if no % 3 == 0 && no % 5 == 0 {
            sum += no;
        }
    }
    println!("Sum is {}", sum);
}

fn main() {
    sum_of_naturals_div_by_3_and_5();
}
