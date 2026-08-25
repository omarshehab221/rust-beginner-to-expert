use std::{cmp::Ordering, marker::PhantomData};

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
    state: PhantomData<State>,
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
            state: PhantomData,
        }
    }

    pub fn guess(mut self, guess: u32) -> GuessOutcome {
        self.attempts += 1;

        match guess.cmp(&self.secret_number) {
            Ordering::Equal => {
                return GuessOutcome::Won(Game {
                    secret_number: self.secret_number,
                    max_attempts: self.max_attempts,
                    attempts: self.attempts,
                    state: PhantomData,
                });
            }
            Ordering::Greater | Ordering::Less if self.is_over() => {
                return GuessOutcome::Lost(Game {
                    secret_number: self.secret_number,
                    max_attempts: self.max_attempts,
                    attempts: self.attempts,
                    state: PhantomData,
                });
            }
            Ordering::Greater => GuessOutcome::TooHigh(self),
            Ordering::Less => GuessOutcome::TooLow(self),
        }
    }

    fn is_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }
}
