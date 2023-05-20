use eyre::*;
use tracing::info;
use web3::types::H160;

use lib::evm_parse::tx::Tx;
use lib::evm_parse::Chain;

use super::pancake_swap::PancakeSwap;
use crate::tracker::trade::Dex;
use crate::DexAddresses;

pub async fn parse_dex_trade(
    chain: Chain,
    tx: &Tx,
    called_contract: &H160,
    dex_addresses: &DexAddresses,
    pancake_swap: &PancakeSwap,
) -> Result<()> {
    let eth_mainnet_dexes = dex_addresses.get(&chain).unwrap();
    for (dex, address) in eth_mainnet_dexes {
        if *address == *called_contract {
            let trade = match dex {
                Dex::PancakeSwap => pancake_swap.parse_trade(tx, chain.clone()),
                Dex::UniSwap => {
                    bail!("does not support dex: UniSwap");
                }
                Dex::SushiSwap => {
                    bail!("does not support dex: SushiSwap");
                }
            };
            info!("tx: {:?}", tx.get_id().unwrap());
            info!("trade: {:?}", trade);
        }
    }
    Ok(())
}
