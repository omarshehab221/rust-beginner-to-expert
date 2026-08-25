use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct InProgress;
#[derive(Debug, PartialEq, Eq)]
pub struct Won;
#[derive(Debug, PartialEq, Eq)]
pub struct Lost;

#[derive(Debug, PartialEq, Eq)]
pub struct Game<State> {
    secret_number: u32,
    max_attempts: u32,
    attempts: u32,
    state: State,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GuessOutcome {
    TooLow(Game<InProgress>),
    TooHigh(Game<InProgress>),
    Won(Game<Won>),
    Lost(Game<Lost>),
}

impl<State> Game<State> {
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }
}

impl Game<InProgress> {
    pub fn new(secret_number: u32, max_attempts: u32) -> Self {
        assert!(max_attempts > 0);

        Self {
            secret_number,
            max_attempts,
            attempts: 0,
            state: InProgress,
        }
    }

    pub fn guess(mut self, guess: u32) -> GuessOutcome {
        self.attempts += 1;

        match guess.cmp(&self.secret_number) {
            Ordering::Less => {
                if self.is_over() {
                    GuessOutcome::Lost(Game {
                        secret_number: self.secret_number,
                        max_attempts: self.max_attempts,
                        attempts: self.attempts,
                        state: Lost,
                    })
                } else {
                    GuessOutcome::TooLow(self)
                }
            }
            Ordering::Greater => {
                if self.is_over() {
                    GuessOutcome::Lost(Game {
                        secret_number: self.secret_number,
                        max_attempts: self.max_attempts,
                        attempts: self.attempts,
                        state: Lost,
                    })
                } else {
                    GuessOutcome::TooHigh(self)
                }
            }
            Ordering::Equal => GuessOutcome::Won(Game {
                secret_number: self.secret_number,
                max_attempts: self.max_attempts,
                attempts: self.attempts,
                state: Won,
            }),
        }
    }

    fn is_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }
}
