use super::pancake_swap::PancakeSwap;
use crate::evm::DexTrade;
use crate::{DexAddresses, TransactionReady};
use eyre::*;
use gen::model::{EnumBlockChain, EnumDex};


pub async fn parse_dex_trade(
    chain: EnumBlockChain,
    tx: &TransactionReady,
    dex_addresses: &DexAddresses,
    pancake_swap: &PancakeSwap,
) -> Result<DexTrade> {
    let called_contract = tx.get_to().context("no called contract")?;
    let dex = dex_addresses
        .get_by_address(chain, called_contract)
        .unwrap();
    let trade = match dex {
        EnumDex::PancakeSwap => pancake_swap.parse_trade(tx, chain.clone())?,
        EnumDex::UniSwap => {
            bail!("does not support dex: UniSwap");
        }
        EnumDex::SushiSwap => {
            bail!("does not support dex: SushiSwap");
        }
    };

    Ok(trade)
}
