mod math_problems;

fn main() {
    //sum_of_naturals_div_by_3_and_5();
    match math_problems::greatest_common_divisor() {
        Ok(result) => {
            println!("GCD is {}", result);
        },
        Err(err) => {
            println!("Something went wrong with input: {}", err);
        },
    }
}
