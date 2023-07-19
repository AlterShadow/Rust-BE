use crate::evm::DexTrade;
use crate::pancake_swap::parse::PancakeSwapParser;
use crate::{DexAddresses, TransactionReady};
use eyre::*;
use gen::model::EnumBlockChain;

pub async fn parse_dex_trade(
    chain: EnumBlockChain,
    tx: &TransactionReady,
    dex_addresses: &DexAddresses,
    pancake_swap_parser: &PancakeSwapParser,
) -> Result<DexTrade> {
    let called_contract = tx.get_to().context("no called contract")?;
    let dex = dex_addresses
        .get_by_address(chain, called_contract)
        .unwrap();
    let trade = match dex {
        "PancakeSwap" => pancake_swap_parser.parse_trade(tx, chain.clone())?,
        _ => {
            bail!("does not support dex: {}", dex);
        }
    };

    Ok(trade)
}
