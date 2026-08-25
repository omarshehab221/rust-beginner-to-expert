use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    InProgress,
    Won,
    Lost,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GuessOutcome {
    TooLow,
    TooHigh,
    Won,
    Lost { secret_number: u32 },
}

#[derive(Debug, thiserror::Error)]
pub enum GuessError {
    #[error("the game is already over")]
    GameOver,
}

#[derive(Debug)]
pub struct Game {
    secret_number: u32,
    max_attempts: u32,
    attempts: u32,
    state: GameState,
}

impl Game {
    pub fn new(secret_number: u32, max_attempts: u32) -> Self {
        Self {
            secret_number,
            max_attempts,
            attempts: 0,
            state: GameState::InProgress,
        }
    }

    pub fn guess(&mut self, guess: u32) -> Result<GuessOutcome, GuessError> {
        if self.state != GameState::InProgress {
            return Err(GuessError::GameOver);
        }

        self.attempts += 1;

        let outcome = match guess.cmp(&self.secret_number) {
            Ordering::Less => GuessOutcome::TooLow,
            Ordering::Greater => GuessOutcome::TooHigh,
            Ordering::Equal => {
                self.state = GameState::Won;
                GuessOutcome::Won
            }
        };

        if self.state == GameState::InProgress && self.is_over() {
            self.state = GameState::Lost;

            return Ok(GuessOutcome::Lost {
                secret_number: self.secret_number,
            });
        }

        Ok(outcome)
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    pub fn is_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }
}
