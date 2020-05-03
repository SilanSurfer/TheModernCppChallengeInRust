type FunResult = Result<u64, &'static str>;

pub fn sum_of_naturals_div_by_3_and_5(input_vec: &Vec<u64>) -> FunResult {
    if input_vec.len() < 1 {
        eprintln!("Expected 1 input, received {}", input_vec.len());
        return Err("Not enough inputs provided");
    }

    let upper_limit = input_vec[0];
    let mut sum = 0;
    for no in 1..upper_limit {
        if no % 3 == 0 && no % 5 == 0 {
            sum += no;
        }
    }
    Ok(sum)
}

pub fn greatest_common_divisor(input_vec: &Vec<u64>) -> FunResult {
    if input_vec.len() < 2 {
        eprintln!("Expected 2 inputs, received {}", input_vec.len());
        return Err("Not enough inputs provided");
    }
    let mut a = input_vec[0];
    let mut b = input_vec[1];

    let mut r: u64;
    while a % b > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    Ok(b)
}
