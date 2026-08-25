use rand::RngExt;
use std::io;
use std::ops::Range;

mod game;
use game::{Game, GuessOutcome as GameOutcome};

#[derive(Debug, thiserror::Error)]
enum RangeParseError {
    #[error("please enter a valid range")]
    InvalidFormat,
    #[error("please enter a valid range start")]
    InvalidStart,
    #[error("please enter a valid range end")]
    InvalidEnd,
    #[error("the start of the range must be less than the end")]
    InvalidBounds,
}

#[derive(Debug, thiserror::Error)]
enum GuessParseError {
    #[error("please enter a valid number")]
    InvalidNumber,
    #[error("please enter a valid number between {0} and {1}")]
    OutOfRange(u32, u32),
}

#[derive(Debug, thiserror::Error)]
enum CliError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("range error: {0}")]
    Range(#[from] RangeParseError),

    #[error("guess error: {0}")]
    Guess(#[from] GuessParseError),

    // #[error("game error: {0}")]
    // Game(#[from] game::GuessError),
}

fn parse_range(input: &str) -> Result<Range<u32>, CliError> {
    let (start, end) = input
        .trim()
        .split_once("..")
        .ok_or(RangeParseError::InvalidFormat)?;

    let start = start
        .parse::<u32>()
        .map_err(|_| RangeParseError::InvalidStart)?;

    let end = end
        .parse::<u32>()
        .map_err(|_| RangeParseError::InvalidEnd)?;

    if start >= end {
        return Err(RangeParseError::InvalidBounds.into());
    }

    Ok(start..end)
}

fn read_input() -> Result<String, CliError> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn read_range() -> Result<Range<u32>, CliError> {
    loop {
        println!("Enter the range of numbers you'd like to guess in the format \"start..end\":");

        let range = read_input()?;

        match parse_range(&range) {
            Ok(range) => {
                // println!("The range is: {:?}", range);
                return Ok(range);
            }
            Err(err) => println!("{err}"),
        }
    }
}

fn get_range(range: &mut Option<Range<u32>>) -> Result<Range<u32>, CliError> {
    if range.is_none() {
        println!("\nFirst, let's agree on a range");
        *range = Some(read_range()?);
    }

    Ok(range.as_ref().unwrap().clone())
}

fn read_guess(range: &Range<u32>) -> Result<u32, CliError> {
    loop {
        println!("\nEnter your guess:");

        let input = read_input()?;

        match input.trim().parse::<u32>() {
            Ok(guess) if range.contains(&guess) => {
                // println!("You guessed: {}", guess);
                return Ok(guess);
            }
            Ok(_) => println!(
                "{}",
                CliError::Guess(GuessParseError::OutOfRange(range.start, range.end))
            ),
            Err(_) => println!("{}", CliError::Guess(GuessParseError::InvalidNumber)),
        }
    }
}

fn play_game(range: &Range<u32>) -> Result<(), CliError> {
    let secret_number = rand::rng().random_range(range.clone());
    // println!("The secret number is: {}", secret_number);

    let mut game = Game::new(secret_number, 6);

    println!("You have {} attempts", game.max_attempts());

    loop {
        println!("\nAttempt: {}/{}", game.attempts(), game.max_attempts());

        let guess = read_guess(range)?;

        match game.guess(guess) {
            GameOutcome::TooLow(next_game) => {
                println!("\nToo small!!!");
                game = next_game;
            }

            GameOutcome::TooHigh(next_game) => {
                println!("\nToo big!!!");
                game = next_game;
            }

            GameOutcome::Won(_) => {
                println!("\nYou guessed right!!!");
                return Ok(());
            }

            GameOutcome::Lost(_) => {
                println!("\nYou lose!");
                println!("The number was: {secret_number}");
                return Ok(());
            }
        }
    }
}

fn init_game(range: &mut Option<Range<u32>>) -> Result<(), CliError> {
    println!("Guess the number!");

    let range = get_range(range)?;

    println!("\nOK! Let's play!");

    play_game(&range)?;

    Ok(())
}

fn main() -> Result<(), CliError> {
    let mut range: Option<Range<u32>> = None;

    loop {

        init_game(&mut range)?;

        println!("Wanna play again? [Y/n]");
        match read_input()?.trim().to_lowercase().as_str() {
            "y" | "yes" | "" => {
                println!("Let's play again!");
                continue;
            }
            _ => break,
        }
    }

    Ok(())
}
