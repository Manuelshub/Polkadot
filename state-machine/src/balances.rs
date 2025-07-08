use std::collections::BTreeMap;

pub struct Pallet {
    pub balances: BTreeMap<String, u128>,
}

impl Pallet {
    // Initialize a new Pallet with an empty balance map.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    // Set the balance for a specific account.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }
    // Get the balance for a specific account, returning 0 if the account does not exist.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }
    // Transfer a specified amount from a sender to a receiver.
    pub fn transfer(
        &mut self,
        sender: &String,
        receiver: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let sender_balance = self.balance(sender);
        let receiver_balance = self.balance(receiver);

        let new_sender_balance = sender_balance
            .checked_sub(amount)
            .ok_or("Not enough funds")?;
        let new_receiver_balance = receiver_balance
            .checked_add(amount)
            .ok_or("Overflow in receiver balance")?;

        self.set_balance(&sender, new_sender_balance);
        self.set_balance(&receiver, new_receiver_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balance() {
        let mut balance = super::Pallet::new();
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        assert_eq!(balance.balance(&alice), 0);
        balance.set_balance(&alice, 100);
        assert_eq!(balance.balance(&alice), 100);
        assert_eq!(balance.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balance = super::Pallet::new();
        let alice = "Alice".to_string();
        let bob = "Bob".to_string();

        assert_eq!(balance.transfer(&alice, &bob, 50), Err("Not enough funds"));
        balance.set_balance(&alice, 100);
        assert_eq!(balance.transfer(&alice, &bob, 50), Ok(()));
        assert_ne!(balance.balance(&alice), 100);
        assert_eq!(balance.balance(&alice), 50);
        assert_eq!(balance.balance(&bob), 50);
    }
}
