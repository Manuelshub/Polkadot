use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Ord + Zero + One + AddAssign + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    current_author: Option<T::AccountId>,
    authors_history: BTreeMap<T::BlockNumber, T::AccountId>, // maps Block number to author
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            current_author: None,
            authors_history: BTreeMap::new(),
        }
    }

    // Set the author for the current block
    pub fn set_author(&mut self, block_number: T::BlockNumber, who: T::AccountId) {
        self.current_author = Some(who.clone());
        self.authors_history.insert(block_number, who);
    }

    // Get the current block author
    pub fn author(&self) -> Option<&T::AccountId> {
        self.current_author.as_ref()
    }

    // Get the author of a past block.
    pub fn author_of(&self, block_number: T::BlockNumber) -> Option<&T::AccountId> {
        self.authors_history.get(&block_number)
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet;

    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
    }

    #[test]
    fn test_authorship() {
        let mut authorship = Pallet::<TestConfig>::new();
        assert!(authorship.author().is_none());

        authorship.set_author(1, "Alice".to_string());
        assert_eq!(authorship.author(), Some(&"Alice".to_string()));
        assert_eq!(authorship.author_of(1), Some(&"Alice".to_string()));

        authorship.set_author(2, "Bob".to_string());
        assert_eq!(authorship.author(), Some(&"Bob".to_string()));
        assert_eq!(authorship.author_of(1), Some(&"Alice".to_string()));
        assert_eq!(authorship.author_of(2), Some(&"Bob".to_string()));
    }
}
