pub mod types;

/// We use Maniswap, see https://manifoldmarkets.notion.site/Maniswap-ce406e1e897d417cbd491071ea8a0c39
use serde::{Deserialize, Serialize};
use types::{Outcome, YesNoValues};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bet {
    pub outcome: Outcome,
    pub shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryMarket {
    pub pool: YesNoValues<u64>,
    // Must be an ordered Vector (allows us to avoid storing date, which the crate user's responsible for)
    // pub bets: Vec<Bet>,
    // todo: "add extra liquidity"
    // todo: "rule"
}

impl BinaryMarket {
    /// Returns new pool values, and the bet
    pub fn evaluate_shares(&self, outcome: Outcome, money: u64) -> (YesNoValues<u64>, Bet) {
        // The AMM receives the order, and converts the $10 into 10 YES shares and 10 NO shares. (Since 1 YES share + 1 NO share always equals $1, the AMM can always issue shares in equal amounts for cash they receive.)
        let current_product = self.pool.yes * self.pool.no;
        // println!("current_product: {}", current_product);
        let new_pool = self.pool.map(|pool| pool + money);
        // println!("new_pool: {:?}", new_pool);

        // The AMM uses a formula based on the number of shares in the liquidity pool to figure out how many YES shares to give back to the trader in return for his wager:
        // Uniswap-style [constant-product](https://medium.com/bollinger-investment-group/constant-function-market-makers-defis-zero-to-one-innovation-968f77022159#5bc7) formula.

        let div_by = new_pool[-outcome];
        // println!("div_by: {}", div_by);

        // println!(
        //     "solving: ({}-x)*{} = {}",
        //     new_pool[outcome], new_pool[-outcome], current_product
        // );

        let expected_shares = (current_product as f64 / div_by as f64).ceil() as u64;
        // println!("expected_shares: {}", expected_shares);

        assert!(expected_shares <= new_pool[outcome]);

        let share_diff = new_pool[outcome] - expected_shares;
        // println!("share_diff: {}", share_diff);

        (
            new_pool,
            Bet {
                outcome,
                shares: share_diff,
            },
        )
    }

    pub fn buy_shares(&mut self, outcome: Outcome, money: u64) -> Bet {
        let (new_pool, bet) = self.evaluate_shares(outcome, money);
        self.pool = new_pool;
        bet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_docs() {
        // For example, if the AMM initializes the pool with 3 YES shares, and 2 NO shares, the initial constant will be 6. If someone wants to buy $1 of YES, the AMM will update the pool to 4 YES, 3 NO. Since the product of 4*3 is not 6, the AMM will figure out how many YES shares to remove to restore the condition, (4-x)(3) = 6. In this case, x=2, which means the trader will get 2 YES shares back for their $1, and the AMM’s resulting liquidity pool will be 2 YES, 3 NO.

        let market = BinaryMarket {
            pool: YesNoValues::new(3, 2),
        };

        let bet = market.evaluate_shares(Outcome::Yes, 1).1;

        assert_eq!(bet.outcome, Outcome::Yes);
        assert_eq!(bet.shares, 2);
    }

    #[test]
    fn bigger() {
        let market = BinaryMarket {
            pool: YesNoValues::new(300, 200),
        };

        let bet = market.evaluate_shares(Outcome::Yes, 200).1;

        assert_eq!(bet.outcome, Outcome::Yes);
        assert_eq!(bet.shares, 350);
    }

    #[test]
    fn rounding_down() {
        // Round down as it's conservative, to avoid conjuring free shares
        let market = BinaryMarket {
            pool: YesNoValues::new(200, 200),
        };

        let bet = market.evaluate_shares(Outcome::No, 100).1;

        assert_eq!(bet.outcome, Outcome::No);
        assert_eq!(bet.shares, 166);
    }
}
