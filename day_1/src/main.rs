use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    
    // Split input into left and right parts
    let (left, right) = process_input(&input);
    
    // Sort and calculate distances
    let result = calculate_distances(left, right);
    
    println!("Sum of distances: {}", result);
}

fn process_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        
        if numbers.len() >= 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }
    
    (left_numbers, right_numbers)
}

fn calculate_distances(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Sort both vectors
    left.sort();
    right.sort();
    
    // Calculate element-wise absolute differences and sum
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distances() {
        let left = vec![1, 3, 2];
        let right = vec![4, 5, 6];
        assert_eq!(calculate_distances(left, right), 9); // (1-4) + (2-5) + (3-6) = 9
    }
} 