use std::fs;

struct Level {
    values: Vec<i32>,
}

impl Level {
    fn new(values: Vec<i32>) -> Self {
        Level { values }
    }

    fn calculate_differences(&self) -> Vec<i32> {
        self.values.windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect()
    }

    fn has_valid_difference_sizes(&self) -> bool {
        let differences = self.calculate_differences();
        differences.iter()
            .all(|&diff| diff.abs() >= 1 && diff.abs() <= 3)
    }

    fn is_monotonic(&self) -> bool {
        let differences = self.calculate_differences();
        if differences.is_empty() {
            return false;
        }
        let first_diff = differences[0];
        differences.iter()
            .all(|&diff| (diff > 0) == (first_diff > 0))
    }

    fn is_safe(&self) -> bool {
        self.has_valid_difference_sizes() && self.is_monotonic()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let levels: Vec<Level> = parse_file(&input).into_iter()
        .map(Level::new)
        .collect();

    for level in &levels {
        println!("Level (safe: {}) {:?} -> {:?}", 
                level.is_safe(), level.values, level.calculate_differences());
    }

    let safe_count = levels.iter().filter(|level| level.is_safe()).count();
    println!("Total safe levels: {}", safe_count);
}

fn parse_file(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_creation() {
        let values = vec![1, 3, 6, 10, 15];
        let level = Level::new(values.clone());
        assert_eq!(level.values, values);
    }

    #[test]
    fn test_safety() {
        let level = Level::new(vec![1, 3, 5, 7]); // valid diffs & monotonic
        assert!(level.is_safe());

        let level2 = Level::new(vec![1, 5, 6, 9]); // invalid diffs
        assert!(!level2.is_safe());

        let level3 = Level::new(vec![1, 3, 2, 4]); // not monotonic
        assert!(!level3.is_safe());
    }

    #[test]
    fn test_calculate_differences() {
        let level = Level::new(vec![1, 3, 6, 10, 15]);
        assert_eq!(level.calculate_differences(), vec![2, 3, 4, 5]);
        
        let level2 = Level::new(vec![1, 3, 6]);
        assert_eq!(level2.calculate_differences(), vec![2, 3]);
    }

    #[test]
    fn test_parse_file() {
        let input = "1 3 5 7 9\n2 4 6 8 10\n3 6 9 12 15";
        let expected = vec![
            vec![1, 3, 5, 7, 9],
            vec![2, 4, 6, 8, 10],
            vec![3, 6, 9, 12, 15]
        ];
        assert_eq!(parse_file(input), expected);

        // Test with varying line lengths and extra whitespace
        let input_irregular = "1 2 3\n  4  5  \n6 7 8 9";
        let expected_irregular = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6, 7, 8, 9]
        ];
        assert_eq!(parse_file(input_irregular), expected_irregular);
    }

    #[test]
    fn test_parse_line() {
        let input = "1 3 5 7 9";
        assert_eq!(parse_line(input), vec![1, 3, 5, 7, 9]);
        
        let input_with_extra_spaces = "  1   3   5  ";
        assert_eq!(parse_line(input_with_extra_spaces), vec![1, 3, 5]);
    }

    #[test]
    fn test_valid_difference_sizes() {
        let level = Level::new(vec![1, 3, 5, 7, 9]); // diffs: [2, 2, 2, 2]
        assert!(level.has_valid_difference_sizes());

        let level2 = Level::new(vec![1, 5, 8, 12]); // diffs: [4, 3, 4]
        assert!(!level2.has_valid_difference_sizes());
    }

    #[test]
    fn test_monotonic() {
        let increasing = Level::new(vec![1, 3, 4, 6]); // all positive diffs
        assert!(increasing.is_monotonic());

        let decreasing = Level::new(vec![6, 4, 3, 1]); // all negative diffs
        assert!(decreasing.is_monotonic());

        let mixed = Level::new(vec![1, 3, 2, 4]); // mixed positive/negative
        assert!(!mixed.is_monotonic());
    }

    #[test]
    fn test_full_program() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let levels: Vec<Level> = parse_file(input).into_iter()
            .map(Level::new)
            .collect();
        
        let safe_count = levels.iter().filter(|level| level.is_safe()).count();
        assert_eq!(safe_count, 2);

        // Let's also verify which sequences are safe
        assert!(levels[0].is_safe());
        assert!(!levels[1].is_safe());
        assert!(!levels[2].is_safe());
        assert!(!levels[3].is_safe());
        assert!(!levels[4].is_safe());
        assert!(levels[5].is_safe());
    }
} 