use num::{One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

pub trait Config {
    type AccuontId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccuontId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccuontId) {
        let current_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = current_nonce + T::Nonce::one();
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet;
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccuontId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn test_system_pallet() {
        let mut sys_pallet = Pallet::<TestConfig>::new();

        // Increase the block number
        sys_pallet.inc_block_number();
        // Increase the nonce
        sys_pallet.inc_nonce(&"Nolan".to_string());
        assert_eq!(sys_pallet.block_number(), 1);
        assert_eq!(sys_pallet.nonce.get("Nolan"), Some(&1));
        assert_eq!(sys_pallet.nonce.get("Grey"), None);
    }
}
