use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let current_nonce = self.nonce.get(who).unwrap_or(&0);
        let new_nonce = current_nonce + 1;
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet;

    #[test]
    fn test_system_pallet() {
        let mut sys_pallet = Pallet::new();

        // Increase the block number
        sys_pallet.inc_block_number();
        // Increase the nonce
        sys_pallet.inc_nonce(&"Nolan".to_string());
        assert_eq!(sys_pallet.block_number(), 1);
    }
}
