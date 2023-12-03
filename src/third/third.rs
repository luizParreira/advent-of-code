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

fn is_gear(digit: char) -> bool {
    digit == '*'
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

    fn sum_gear_ratios(&self) -> Result<i64, ThirdError> {
        let mut result = 0;
        for (line_index, line) in self.digits.iter().enumerate() {
            let max_bound: usize = line.chars().count() - 1;
            let chars: Vec<char> = line.chars().collect();
            for (index, digit_char) in line.chars().enumerate() {
                if is_gear(digit_char) {
                    let mut ratios: Vec<i64> = vec![];
                    if index != 0 {
                        let joined = Self::extract_number_before(&chars, index).join("");

                        if !joined.is_empty() {
                            let number_before = joined
                                .parse::<i64>()
                                .map_err(|_| ThirdError::InvalidNumber)?;
                            if number_before > 0 {
                                ratios.push(number_before);
                            }
                        }
                    }

                    if index != max_bound {
                        let joined = Self::extract_number_after(&chars, index).join("");

                        if !joined.is_empty() {
                            let number_after = joined
                                .parse::<i64>()
                                .map_err(|_| ThirdError::InvalidNumber)?;

                            if number_after > 0 {
                                ratios.push(number_after);
                            }
                        }
                    }

                    if line_index != 0 {
                        let mut numbers_from_above_line = Self::extract_number_from_line(
                            &self.digits[line_index - 1].chars().collect(),
                            index,
                        )?;

                        ratios.append(&mut numbers_from_above_line);
                    }

                    if line_index != max_bound {
                        let mut numbers_from_below_line = Self::extract_number_from_line(
                            &self.digits[line_index + 1].chars().collect(),
                            index,
                        )?;

                        ratios.append(&mut numbers_from_below_line);
                    }

                    let mut r = 1;
                    if ratios.len() == 2 {
                        for ratio in ratios {
                            r *= ratio;
                        }

                        result += r;
                    }
                }
            }
        }

        Ok(result)
    }

    fn extract_number_before(chars: &Vec<char>, index: usize) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for d in chars[0..index].iter().rev() {
            if !is_digit(*d) {
                break;
            }

            result.push(d.to_string());
        }

        result.reverse();

        result
    }

    fn extract_number_after(chars: &Vec<char>, index: usize) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for d in &chars[(index + 1)..chars.len()] {
            if !is_digit(*d) {
                break;
            }

            result.push(d.to_string());
        }

        result
    }

    fn extract_number_from_line(line: &Vec<char>, index: usize) -> Result<Vec<i64>, ThirdError> {
        let mut result: Vec<String> = vec![];
        let mut line_index = index;
        while line_index < line.len() && is_digit(line[line_index]) {
            result.push(line[line_index].to_string());
            if line_index == 0 {
                break;
            }
            line_index -= 1;
        }

        line_index = index + 1;
        result.reverse();
        while line_index < line.len() && is_digit(line[line_index]) {
            result.push(line[line_index].to_string());
            if line_index == line.len() - 1 {
                break;
            }
            line_index += 1;
        }

        let mut second_result: Vec<String> = vec![];

        line_index = index - 1;
        while line_index < line.len() && is_digit(line[line_index]) {
            if !is_digit(line[index]) {
                second_result.push(line[line_index].to_string());
            }
            if line_index == 0 {
                break;
            }
            line_index -= 1;
        }

        second_result.reverse();

        let mut response_0 = result.into_iter().collect::<String>();

        if response_0.is_empty() {
            response_0 = "0".to_string();
        }

        let mut response_1: String = second_result.into_iter().collect::<String>();

        if response_1.is_empty() {
            response_1 = "0".to_string();
        }

        Ok(vec![
            response_0
                .parse::<i64>()
                .map_err(|_| ThirdError::InvalidNumber)?,
            response_1
                .parse::<i64>()
                .map_err(|_| ThirdError::InvalidNumber)?,
        ]
        .into_iter()
        .filter_map(|s| if s == 0 { None } else { Some(s) })
        .collect())
    }
}

pub fn solve_part_one(path: &str) -> Result<i64, ThirdError> {
    let file = std::fs::read_to_string(path).map_err(|_| ThirdError::CantFindFile)?;
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let plane = Plane { digits: lines };
    plane.count_machine_parts()
}

pub fn solve_part_two(path: &str) -> Result<i64, ThirdError> {
    let file = std::fs::read_to_string(path).map_err(|_| ThirdError::CantFindFile)?;
    let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
    let plane = Plane { digits: lines };
    plane.sum_gear_ratios()
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

    #[test]
    pub fn test_part_two() {
        let solve = super::solve_part_two("src/third/test_input.txt").unwrap();

        assert_eq!(solve, 467835);
        let solve = super::solve_part_two("src/third/test_input-2.txt").unwrap();
        assert_eq!(solve, 6756);
    }
}
