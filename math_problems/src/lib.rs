type FunResult = Result<u64, &'static str>;

pub fn sum_of_naturals_div_by_3_and_5(input_vec: &[u64]) -> FunResult {
    if input_vec.is_empty() {
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

pub fn greatest_common_divisor(input_vec: &[u64]) -> FunResult {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd_too_short_input() {
        let input_vec = vec![2];
        assert_eq!(
            Err("Not enough inputs provided"),
            greatest_common_divisor(&input_vec)
        );
    }

    #[test]
    fn test_gcd_too_long_input() {
        let input_vec = vec![2, 4, 8];
        assert_eq!(Ok(2), greatest_common_divisor(&input_vec));
    }

    #[test]
    fn test_gcd_proper_input() {
        let input_vec = vec![3, 9];
        assert_eq!(Ok(3), greatest_common_divisor(&input_vec));
    }

    #[test]
    fn test_sum_of_naturals_too_short_input() {
        let input_vec = vec![];
        assert_eq!(
            Err("Not enough inputs provided"),
            sum_of_naturals_div_by_3_and_5(&input_vec)
        );
    }

    #[test]
    fn test_sum_of_naturals_too_long_input() {
        let input_vec = vec![16, 100];
        assert_eq!(Ok(15), sum_of_naturals_div_by_3_and_5(&input_vec));
    }

    #[test]
    fn test_sum_of_naturals_ok_input() {
        let input_vec = vec![31];
        assert_eq!(Ok(45), sum_of_naturals_div_by_3_and_5(&input_vec));
    }
}
