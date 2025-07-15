use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    // Initialize a new Pallet with an empty balance map.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    // Set the balance for a specific account.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }
    // Get the balance for a specific account, returning 0 if the account does not exist.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
    // Transfer a specified amount from a sender to a receiver.
    pub fn transfer(
        &mut self,
        sender: T::AccountId,
        receiver: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
        let sender_balance = self.balance(&sender);
        let receiver_balance = self.balance(&receiver);

        let new_sender_balance = sender_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds")?;
        let new_receiver_balance = receiver_balance
            .checked_add(&amount)
            .ok_or("Overflow in receiver balance")?;

        self.balances.insert(sender, new_sender_balance);
        self.balances.insert(receiver, new_receiver_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::u128;

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn init_balance() {
        let mut balance = super::Pallet::<TestConfig>::new();
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        assert_eq!(balance.balance(&alice), 0);
        balance.set_balance(&alice, 100);
        assert_eq!(balance.balance(&alice), 100);
        assert_eq!(balance.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balance = super::Pallet::<TestConfig>::new();

        assert_eq!(
            balance.transfer("Alice".to_string(), "Bob".to_string(), 50),
            Err("Not enough funds")
        );
        balance.set_balance(&"Alice".to_string(), 100);
        assert_eq!(
            balance.transfer("Alice".to_string(), "Bob".to_string(), 50),
            Ok(())
        );
        assert_ne!(balance.balance(&"Alice".to_string()), 100);
        assert_eq!(balance.balance(&"Alice".to_string()), 50);
        assert_eq!(balance.balance(&"Bob".to_string()), 50);
    }
}
