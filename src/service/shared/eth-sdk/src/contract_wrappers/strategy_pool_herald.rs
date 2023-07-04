use eyre::*;
use web3::types::TransactionReceipt;
use web3::types::{Address, U256};

const POOL_HERALD_ABI_JSON: &str = include_str!("strategy_pool_herald.json");

// TODO: deploy function for local testing

#[derive(Debug, Clone)]
pub struct HeraldRedeemEvent {
    pub strategy_pool: Address,
    pub strategy_wallet: Address,
    pub backer: Address,
    pub amount: U256,
}

pub fn parse_herald_redeem_event(
    herald_address: Address,
    receipt: TransactionReceipt,
) -> Result<HeraldRedeemEvent> {
    let herald = web3::ethabi::Contract::load(POOL_HERALD_ABI_JSON.as_bytes())?;
    let redeem_event = herald
        .event("Redeem")
        .context("Failed to get Redeem event from strategy pool")?;

    for log in receipt.logs {
        /* there can only be 4 indexed (topic) values in a event log */
        /* 1st topic is always the hash of the event signature */
        if log.topics[0] == redeem_event.signature()
						/* address of the contract that fired the event */
						&& log.address == herald_address
        {
            /* 2nd topic is sender of the call, should be strategy pool address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let strategy_pool_bytes = log.topics[1].as_bytes();
            if strategy_pool_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let strategy_pool = Address::from_slice(&strategy_pool_bytes[12..]);

            /* 3rd topic is the owner of the strategy pool tokens, should be strategy wallet address */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let strategy_wallet_bytes = log.topics[2].as_bytes();
            if strategy_wallet_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let strategy_wallet = Address::from_slice(&strategy_wallet_bytes[12..]);

            /* 4th topic is the receiver of the assets, should be backer address in strategy wallet */
            /* topics have 32 bytes, so we must fetch the last 20 bytes for an address */
            let backer_bytes = log.topics[3].as_bytes();
            if backer_bytes.len() < 32 {
                return Err(eyre!("invalid topic length"));
            }
            let backer = Address::from_slice(&backer_bytes[12..]);

            /* instantiate an ethabi::Log from raw log to enable access to non indexed data */
            let parsed_log = redeem_event.parse_log(web3::ethabi::RawLog {
                topics: log.topics.clone(),
                data: log.data.0.clone(),
            })?;

            /* parse non indexed event data from event log */
            /* ethabi::Log params ignore the first topic, so params[0] is not the event signature */
            let amount = parsed_log.params[3]
                .value
                .clone()
                .into_uint()
                .context("could not parse redeemed sp tokens from event log")?;

            return Ok(HeraldRedeemEvent {
                strategy_pool,
                strategy_wallet,
                backer,
                amount,
            });
        }
    }
    Err(eyre!("could not find redeem event in receipt"))
}
