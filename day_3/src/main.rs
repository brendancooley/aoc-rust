use std::fs;
use regex::Regex;

fn extract_multiplications(input: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| {
            let first = cap[1].parse::<i32>().ok()?;
            let second = cap[2].parse::<i32>().ok()?;
            Some((first, second))
        })
        .collect()
}

fn multiply_and_sum(values: Vec<(i32, i32)>) -> i32 {
    values.iter().map(|(a, b)| a * b).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let multiplications = extract_multiplications(&input);
    let sum = multiply_and_sum(multiplications);
    println!("Result: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const AOC_TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_extract_multiplications() {
        let input = "some text mul(123,456) more text mul(789,012) end";
        let result = extract_multiplications(input);
        assert_eq!(result, vec![(123, 456), (789, 12)]);

        // Test with no multiplications
        let input = "no multiplications here";
        let result = extract_multiplications(input);
        assert_eq!(result, Vec::<(i32, i32)>::new());

        // Test with invalid format
        let input = "mul(abc,123) mul(456,789)";
        let result = extract_multiplications(input);
        assert_eq!(result, vec![(456, 789)]);

        // Test with spaces
        let input = "mul(456 , 789)";
        let result = extract_multiplications(input);
        assert_eq!(result, Vec::<(i32, i32)>::new());

        // Test AOC example
        let result = extract_multiplications(AOC_TEST_INPUT);
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn test_multiply_and_sum() {
        assert_eq!(multiply_and_sum(vec![(2, 3), (4, 5)]), 26); // (2*3) + (4*5) = 6 + 20 = 26
        assert_eq!(multiply_and_sum(vec![]), 0); // empty vector should sum to 0

        let result = extract_multiplications(AOC_TEST_INPUT);
        let sum = multiply_and_sum(result);
        assert_eq!(sum, 161)
    }

}
