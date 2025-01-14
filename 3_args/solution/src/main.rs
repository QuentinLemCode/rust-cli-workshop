use std::{convert::TryFrom, env};

#[derive(Debug)]
pub enum Game {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&String> for Game {
    type Error = String;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "rock" => Ok(Game::Rock),
            "paper" => Ok(Game::Paper),
            "scissors" => Ok(Game::Scissors),
            _ => Err("Invalid game".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Draw,
    Lost,
}

pub fn play(a: Game, b: Game) -> GameResult {
    match (a, b) {
        // win
        (Game::Paper, Game::Rock) => GameResult::Win,
        (Game::Rock, Game::Scissors) => GameResult::Win,
        (Game::Scissors, Game::Paper) => GameResult::Win,
        // draw
        (Game::Rock, Game::Rock) => GameResult::Draw,
        (Game::Scissors, Game::Scissors) => GameResult::Draw,
        (Game::Paper, Game::Paper) => GameResult::Draw,
        // lose
        (Game::Rock, Game::Paper) => GameResult::Lost,
        (Game::Paper, Game::Scissors) => GameResult::Lost,
        (Game::Scissors, Game::Rock) => GameResult::Lost,
    }
}

pub fn greets(name: &str) -> String {
    format!("Hello, {} 🦀 !", name)
}

#[cfg(test)]
mod tests {
    use super::greets;

    use super::play;
    use super::Game;
    use super::GameResult;

    #[test]
    fn test_player_one_wins() {
        assert_eq!(play(Game::Rock, Game::Scissors), GameResult::Win);
        assert_eq!(play(Game::Scissors, Game::Paper), GameResult::Win);
        assert_eq!(play(Game::Paper, Game::Rock), GameResult::Win);
    }

    #[test]
    fn test_player_one_draw() {
        assert_eq!(play(Game::Rock, Game::Rock), GameResult::Draw);
        assert_eq!(play(Game::Paper, Game::Paper), GameResult::Draw);
        assert_eq!(play(Game::Scissors, Game::Scissors), GameResult::Draw);
    }

    #[test]
    fn test_player_one_lose() {
        assert_eq!(play(Game::Rock, Game::Paper), GameResult::Lost);
        assert_eq!(play(Game::Paper, Game::Scissors), GameResult::Lost);
        assert_eq!(play(Game::Scissors, Game::Rock), GameResult::Lost);
    }

    #[test]
    fn test_greets() {
        assert_eq!(greets("You"), String::from("Hello, You 🦀 !"));
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).expect("Command is required");

    match cmd.as_str() {
        "greets" => {
            let name = args.get(2).expect("Name is required for greetings");
            println!("{}", greets(&name));
        }
        "chifoumi" => {
            let p_one = args.get(2).expect("Player one is required for chifoumi");
            let p_two = args.get(3).expect("Player two is required for chifoumi");
            // cast to Game enum
            let p_one: Game = Game::try_from(p_one).unwrap();
            let p_two: Game = Game::try_from(p_two).unwrap();

            let result = play(p_one, p_two);
            println!("p1 vs p2 : {:?}", result);
        }
        _ => eprintln!("Not supported command"),
    }
}
