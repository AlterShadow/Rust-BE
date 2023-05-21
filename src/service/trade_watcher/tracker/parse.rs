use eyre::*;
use tracing::info;
use web3::types::H160;

use super::pancake_swap::PancakeSwap;
use crate::evm::Transaction;
use crate::DexAddresses;
use gen::model::{EnumBlockChain, EnumDex};
pub async fn parse_dex_trade(
    chain: EnumBlockChain,
    tx: &Transaction,
    called_contract: &H160,
    dex_addresses: &DexAddresses,
    pancake_swap: &PancakeSwap,
) -> Result<()> {
    let eth_mainnet_dexes = dex_addresses.get(&chain).unwrap();
    for (dex, address) in eth_mainnet_dexes {
        if *address == *called_contract {
            let trade = match dex {
                EnumDex::PancakeSwap => pancake_swap.parse_trade(tx, chain.clone()),
                EnumDex::UniSwap => {
                    bail!("does not support dex: UniSwap");
                }
                EnumDex::SushiSwap => {
                    bail!("does not support dex: SushiSwap");
                }
            };
            info!("tx: {:?}", tx.get_id().unwrap());
            info!("trade: {:?}", trade);
        }
    }
    Ok(())
}
