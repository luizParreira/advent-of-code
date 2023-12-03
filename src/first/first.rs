#[derive(Debug)]
pub enum FirstError {
    CantFindFile,
    InvalidNumber,
}

const INPUT_PATH: &str = "src/first/input.txt";

pub fn solve() -> Result<i64, FirstError> {
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
