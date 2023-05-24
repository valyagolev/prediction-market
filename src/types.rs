use std::ops::{Index, Neg};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Outcome {
    Yes,
    No,
}

impl Neg for Outcome {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Outcome::Yes => Outcome::No,
            Outcome::No => Outcome::Yes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct YesNoValues<T> {
    pub yes: T,
    pub no: T,
}

impl<T> YesNoValues<T> {
    pub fn new(yes: T, no: T) -> Self {
        Self { yes, no }
    }

    pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> YesNoValues<U> {
        YesNoValues {
            yes: f(&self.yes),
            no: f(&self.no),
        }
    }
}

impl<T> Index<Outcome> for YesNoValues<T> {
    type Output = T;

    fn index(&self, outcome: Outcome) -> &Self::Output {
        match outcome {
            Outcome::Yes => &self.yes,
            Outcome::No => &self.no,
        }
    }
}
