use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

#[test]
fn keygen() {
// Create a new keypair
let kp = Keypair::new();
println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
println!("To save your wallet, copy and paste the following into a JSON file:"); println!("{:?}", kp.to_bytes());
}

#[cfg(test)] mod tests {
    use solana_sdk;
    // use super::*;
    #[test]
    fn keygen() {} #[test]
    fn airdop() {} #[test]
    fn transfer_sol() {}
     
    }
