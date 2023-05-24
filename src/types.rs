use std::ops::{Index, IndexMut, Neg};

#[cfg(test)]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(Arbitrary))]
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
#[cfg_attr(test, derive(Arbitrary))]
pub struct Bet {
    pub outcome: Outcome,
    pub shares: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(Arbitrary))]
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

impl<T> IndexMut<Outcome> for YesNoValues<T> {
    fn index_mut(&mut self, outcome: Outcome) -> &mut Self::Output {
        match outcome {
            Outcome::Yes => &mut self.yes,
            Outcome::No => &mut self.no,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BinaryMarket {
    pub pool: YesNoValues<u64>,
    // Must be an ordered Vector (allows us to avoid storing date, which the crate user's responsible for)
    // pub bets: Vec<Bet>,
    // todo: "add extra liquidity"
    // todo: "rule"
}
