#[derive(Debug)]
pub enum SecondError {
    CantFindFile,
    InvalidNumber,
    InvalidString(String),
    MissingGammeId,
    InvalidInput,
}

#[derive(Debug)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

impl Bag {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    fn valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    bag_sets: Vec<Bag>,
}

impl Game {
    fn new(str: &str) -> Result<Game, SecondError> {
        let mut splitted = str.split(":");
        let game = splitted
            .next()
            .ok_or(SecondError::InvalidString(str.to_string()))?;
        let data = splitted
            .next()
            .ok_or(SecondError::InvalidString(str.to_string()))?;

        let id = Self::parse_game_id(game)?;
        let bag_sets = Self::parse_bag_sets(data)?;

        Ok(Game { id, bag_sets })
    }

    fn all_valid(&self) -> bool {
        self.bag_sets.iter().all(|b| b.valid())
    }

    fn parse_game_id(string: &str) -> Result<i32, SecondError> {
        string
            .trim()
            .split(" ")
            .find(|d| d.parse::<i32>().is_ok())
            .ok_or(SecondError::MissingGammeId)?
            .parse::<i32>()
            .map_err(|_| SecondError::InvalidNumber)
    }

    fn parse_bag_sets(string: &str) -> Result<Vec<Bag>, SecondError> {
        string.split(";").map(|s| Game::parse_bag(s)).collect()
    }

    fn parse_bag(string: &str) -> Result<Bag, SecondError> {
        let mut red = 0i32;
        let mut green = 0i32;
        let mut blue = 0i32;
        for split in string.trim().split(",") {
            let mut splitted = split.trim().split(" ");

            let n = splitted
                .next()
                .ok_or(SecondError::InvalidInput)?
                .parse::<i32>()
                .map_err(|_| SecondError::InvalidNumber)?;

            let color = splitted.next().ok_or(SecondError::InvalidInput)?;

            match color.to_lowercase().as_str() {
                "green" => green += n,
                "red" => red += n,
                "blue" => blue += n,
                _ => return Err(SecondError::InvalidInput),
            }
        }

        return Ok(Bag { red, green, blue });
    }
}

fn sum_valid_ids(games: &Vec<Game>) -> i32 {
    games
        .into_iter()
        .map(|g| if g.all_valid() { g.id } else { 0 })
        .sum()
}

const INPUT_PATH: &str = "src/second/input.txt";
const TEST_INPUT_PATH: &str = "src/second/test_input.txt";

pub fn solve_part_one() -> Result<i32, SecondError> {
    let file = std::fs::read_to_string(INPUT_PATH).map_err(|_| SecondError::CantFindFile)?;
    let games: Vec<Game> = file
        .lines()
        .filter_map(|line| Game::new(line).ok())
        .collect();

    Ok(sum_valid_ids(&games))
}
