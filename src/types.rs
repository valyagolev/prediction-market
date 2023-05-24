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
/// A market with two outcomes, YES and NO.
///
/// This is all the information (two numbers: the ppol), that is needed to calculate bet payouts.
/// It is responsibility of the crate's user to store bet values, and to calculate/distribute payouts.
///
/// Store those two numbers, and all the bets. When a new bet is being made,
/// calculate the shares (using [BinaryMarket::buy_shares]), update the market, and store the bet.
pub struct BinaryMarket {
    pub pool: YesNoValues<u64>,
}
