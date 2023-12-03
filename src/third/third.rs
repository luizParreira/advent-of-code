#[derive(Debug)]
pub enum ThirdError {
    CantFindFile,
    InvalidNumber,
}

#[derive(Debug)]
struct Digit {
    digit: char,
    index: i32,
}

fn is_symbol(digit: char) -> bool {
    digit != '.' && !is_digit(digit)
}

fn is_digit(digit: char) -> bool {
    digit.is_digit(10)
}

struct Plane {
    digits: Vec<String>,
}

impl Plane {
    fn count_machine_parts(&self) -> Result<i64, ThirdError> {
        let mut result = 0;
        for (line_index, line) in self.digits.iter().enumerate() {
            let max_bound: usize = line.chars().count() - 1;
            let mut numbers: Vec<Digit> = vec![];
            for (index, digit_char) in line.chars().enumerate() {
                let digit = Digit {
                    digit: digit_char,
                    index: index as i32,
                };

                if is_digit(digit_char) {
                    numbers.push(digit);
                    if index != max_bound {
                        continue;
                    }
                }

                if !numbers.is_empty() {
                    let nums = numbers
                        .iter()
                        .map(|d| d.digit.to_string())
                        .collect::<Vec<String>>();

                    let number = nums
                        .join("")
                        .parse::<i64>()
                        .map_err(|_| ThirdError::InvalidNumber)?;
                    let indexes = numbers.iter().map(|n| n.index);
                    let min_index = (indexes.clone().min().unwrap_or(0) - 1).max(0) as usize;
                    let max_index = (indexes.max().unwrap_or(0) + 1).min(max_bound as i32) as usize;
                    let line_list: Vec<char> = line.chars().collect();
                    let min_char = line_list[min_index];

                    if index != 0 && is_symbol(min_char) {
                        result += number;
                        numbers = vec![];
                        continue;
                    }

                    let max_char = line_list[max_index];
                    if index != max_bound && is_symbol(max_char) {
                        result += number;
                        numbers = vec![];
                        continue;
                    }
                    if line_index != 0 {
                        let above_line = &self.digits[line_index - 1][min_index..=max_index];
                        if above_line.contains(|c| is_symbol(c)) {
                            result += number;
                            numbers = vec![];
                            continue;
                        }
                    }

                    if line_index != max_bound {
                        let below_line = &self.digits[line_index + 1][min_index..=max_index];
                        if below_line.contains(|c| is_symbol(c)) {
                            result += number;
                            numbers = vec![];
                            continue;
                        }
                    }

                    numbers = vec![];
                }
            }
        }

        Ok(result)
    }
}

pub fn solve_part_one(path: &str) -> Result<i64, ThirdError> {
    let file = std::fs::read_to_string(path).map_err(|_| ThirdError::CantFindFile)?;
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let plane = Plane { digits: lines };
    plane.count_machine_parts()
}

#[cfg(test)]
mod test {

    const TEST_INPUT_PATH: &str = "src/third/test_input.txt";

    #[test]
    pub fn test_part_one() {
        let solve = super::solve_part_one("src/third/test_input.txt").unwrap();

        assert_eq!(solve, 4361);
        let solve = super::solve_part_one("src/third/test_input-2.txt").unwrap();
        assert_eq!(solve, 925);
    }
}
