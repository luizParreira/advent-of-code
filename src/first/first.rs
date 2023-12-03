#[derive(Debug)]
pub enum FirstError {
    CantFindFile,
    InvalidNumber,
}

const INPUT_PATH: &str = "src/first/input.txt";
const TEST_INPUT_PATH: &str = "src/first/test_input.txt";

pub fn solve_part_one() -> Result<i64, FirstError> {
    let file = std::fs::read_to_string(INPUT_PATH).map_err(|_| FirstError::CantFindFile)?;
    let lines = file.lines();
    let mut result = 0;
    for line in lines {
        let chars = line
            .chars()
            .filter_map(|c| if c.is_digit(10) { Some(c) } else { None });
        let first = chars.clone().next().unwrap_or_else(|| '0');
        let last = chars.rev().next().unwrap_or_else(|| '0');
        let number = format!("{first}{last}");
        println!("{line} -> {number}");
        let mapped_number: i64 = number.parse().map_err(|_| FirstError::InvalidNumber)?;
        result += mapped_number;
    }
    // 5. return result
    return Ok(result);
}

struct Digit {
    digit: String,
    digit_number: i64,
}

impl Digit {
    fn new(digit: &str, number: i64) -> Self {
        Self {
            digit: digit.to_string(),
            digit_number: number,
        }
    }
}

pub fn solve_part_two() -> Result<i64, FirstError> {
    let digits: Vec<Digit> = vec![
        Digit::new("one", 1),
        Digit::new("two", 2),
        Digit::new("three", 3),
        Digit::new("four", 4),
        Digit::new("five", 5),
        Digit::new("six", 6),
        Digit::new("seven", 7),
        Digit::new("eight", 8),
        Digit::new("nine", 9),
    ];
    let file = std::fs::read_to_string(INPUT_PATH).map_err(|_| FirstError::CantFindFile)?;
    let lines = file.lines();
    let mut result = 0;
    for line in lines {
        let mut non_numeric_chars: String = String::new();
        let mut numeric_chars: Vec<String> = vec![];
        // Find first
        for c in line.chars() {
            if c.is_digit(10) {
                numeric_chars.push(c.to_string());
                non_numeric_chars = String::new();
                break;
            } else {
                non_numeric_chars = format!("{non_numeric_chars}{c}");
                if let Some(d) = digits.iter().find(|d| non_numeric_chars.contains(&d.digit)) {
                    numeric_chars.push(d.digit_number.to_string());
                    non_numeric_chars = String::new();
                    break;
                }
            }
        }
        // Find last
        for c in line.chars().rev() {
            if c.is_digit(10) {
                numeric_chars.push(c.to_string());
                break;
            } else {
                non_numeric_chars = format!("{c}{non_numeric_chars}");
                if let Some(d) = digits.iter().find(|d| non_numeric_chars.contains(&d.digit)) {
                    numeric_chars.push(d.digit_number.to_string());
                    break;
                }
            }
        }

        let first = numeric_chars
            .clone()
            .into_iter()
            .next()
            .unwrap_or_else(|| "0".to_string());
        let last = numeric_chars
            .into_iter()
            .rev()
            .next()
            .unwrap_or_else(|| "0".to_string());
        let number = format!("{first}{last}");
        println!("{line} -> {number}");
        let mapped_number: i64 = number.parse().map_err(|_| FirstError::InvalidNumber)?;
        result += mapped_number;
    }
    // 5. return result
    return Ok(result);
}
