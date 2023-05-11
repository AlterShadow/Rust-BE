use web3::types::{H160, U256};

#[derive(Clone, Debug)]
pub enum Chain {
    Ethereum,
    Bsc,
}

#[derive(Clone, Debug)]
pub enum Dex {
    UniSwap,
    PancakeSwap,
    SushiSwap,
}

#[derive(Clone, Debug)]
pub enum DexVersion {
    V1,
    V2,
    V3,
}

#[derive(Clone, Debug)]
pub struct TradingPair {
    chain: Chain,
    contract: H160,
    dex: Dex,
    dex_version: DexVersion,
    token_in: H160,
    token_out: H160,
    fee: Option<u32>,
}

impl TradingPair {
    pub fn new(
        chain: Chain,
        contract: H160,
        dex: Dex,
        dex_version: DexVersion,
        token_in: H160,
        token_out: H160,
        fee: Option<u32>,
    ) -> Self {
        Self {
            chain,
            contract,
            dex,
            dex_version,
            token_in,
            token_out,
            fee,
        }
    }

    pub fn get_chain(&self) -> Chain {
        self.chain.clone()
    }

    pub fn get_contract(&self) -> H160 {
        self.contract.clone()
    }

    pub fn get_dex(&self) -> Dex {
        self.dex.clone()
    }

    pub fn get_dex_version(&self) -> DexVersion {
        self.dex_version.clone()
    }

    pub fn get_token_in(&self) -> H160 {
        self.token_in.clone()
    }

    pub fn get_token_out(&self) -> H160 {
        self.token_out.clone()
    }

    pub fn get_fee(&self) -> Option<u32> {
        self.fee.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Swap {
    recipient: H160,
    chain: Chain,
    contract: H160,
    dex: Dex,
    dex_version: DexVersion,
    token_in: H160,
    token_out: H160,
    token_in_amount: U256,
    token_out_amount: U256,
    fee: Option<u32>,
}

impl Swap {
    pub fn new(
        recipient: H160,
        chain: Chain,
        contract: H160,
        dex: Dex,
        dex_version: DexVersion,
        token_in: H160,
        token_out: H160,
        token_in_amount: U256,
        token_out_amount: U256,
        fee: Option<u32>,
    ) -> Self {
        Self {
            recipient,
            chain,
            contract,
            dex,
            dex_version,
            token_in,
            token_out,
            token_in_amount,
            token_out_amount,
            fee,
        }
    }

    pub fn get_recipient(&self) -> H160 {
        self.recipient.clone()
    }

    pub fn get_chain(&self) -> Chain {
        self.chain.clone()
    }

    pub fn get_contract(&self) -> H160 {
        self.contract.clone()
    }

    pub fn get_dex(&self) -> Dex {
        self.dex.clone()
    }

    pub fn get_dex_version(&self) -> DexVersion {
        self.dex_version.clone()
    }

    pub fn get_token_in(&self) -> H160 {
        self.token_in.clone()
    }

    pub fn get_token_out(&self) -> H160 {
        self.token_out.clone()
    }

    pub fn get_token_in_amount(&self) -> U256 {
        self.token_in_amount.clone()
    }

    pub fn get_token_out_amount(&self) -> U256 {
        self.token_out_amount.clone()
    }

    pub fn get_fee(&self) -> Option<u32> {
        self.fee.clone()
    }
}
