mod authorship;
mod balances;
mod system;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

impl system::Config for Runtime {
    type AccuontId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let nolan = "Nolan".to_string();
    let bradford = "Bradford".to_string();
    let grey = "Grey".to_string();

    runtime.balances.set_balance(&nolan, 100);

    // Genesis block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // First transaction in genesis block
    runtime.system.inc_nonce(&nolan);
    let _res = runtime
        .balances
        .transfer(nolan.clone(), bradford, 50)
        .map_err(|e| eprintln!("{e}"));

    // Second transaction
    runtime.system.inc_nonce(&nolan);
    let _res = runtime
        .balances
        .transfer(nolan, grey, 20)
        .map_err(|e| eprintln!("{e}"));

    println!("{runtime:#?}");
}
