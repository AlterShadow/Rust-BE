use eyre::*;
use web3::types::Address;
use web3::types::TransactionReceipt;

const WALLET_HERALD_ABI_JSON: &str = include_str!("strategy_wallet_herald.json");

// TODO: deploy function for local testing

#[derive(Debug, Clone)]
pub struct StrategyWalletHeraldRevokeAdminshipEvent {
    pub strategy_wallet: Address,
    pub old_admin: Address,
}

pub fn parse_strategy_wallet_herald_revoke_adminship_event(
    herald_address: Address,
    receipt: TransactionReceipt,
) -> Result<StrategyWalletHeraldRevokeAdminshipEvent> {
    let herald = web3::ethabi::Contract::load(WALLET_HERALD_ABI_JSON.as_bytes())?;
    let redeem_event = herald
        .event("RevokeAdminship")
        .context("Failed to get RevokeAdminship event from strategy wallet")?;

    for log in receipt.logs {
        /* there can only be 4 indexed (topic) values in a event log */
        /* 1st topic is always the hash of the event signature */
        if log.topics[0] == redeem_event.signature()
						/* address of the contract that fired the event */
						&& log.address == herald_address
        {
            /* 2nd topic is sender of the call, should be strategy wallet address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let strategy_wallet_bytes = log.topics[1].as_bytes();
            if strategy_wallet_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let strategy_wallet = Address::from_slice(&strategy_wallet_bytes[12..]);

            /* 3rd topic is the revoked admin address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let old_admin_bytes = log.topics[2].as_bytes();
            if old_admin_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let old_admin = Address::from_slice(&old_admin_bytes[12..]);

            return Ok(StrategyWalletHeraldRevokeAdminshipEvent {
                strategy_wallet,
                old_admin,
            });
        }
    }
    Err(eyre!("could not find revoke adminship event in receipt"))
}
