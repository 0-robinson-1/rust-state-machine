use std::collections::BTreeMap;

pub struct Pallet {
        balances: BTreeMap<String, u128>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
            Self {
                balances: BTreeMap::new()
            }
    }

    /// Set the balances of an account 'who' to some 'amount'.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        /* Insert 'amount' into the BTreeMap under 'who'. */
        unimplemented!()
    }

    /// Get the balance of an account 'who'.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        /* Return the balance of 'who', returning zero if 'None'. */
        unimplemented!()
    }
 }