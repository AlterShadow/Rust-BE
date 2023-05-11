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
pub struct Trade {
    chain: Chain,
    contract: H160,
    dex: Dex,
    dex_version: DexVersion,
    token_in: H160,
    token_out: H160,
    fee: Option<U256>,
    /* caller should be the same as recipient if swap was done by user */
    caller: H160,
    recipient: H160,
    amount_in: U256,
    amount_out: U256,
}

impl Trade {
    pub fn new(
        chain: Chain,
        contract: H160,
        dex: Dex,
        dex_version: DexVersion,
        token_in: H160,
        token_out: H160,
        fee: Option<U256>,
        caller: H160,
        recipient: H160,
        amount_in: U256,
        amount_out: U256,
    ) -> Self {
        Self {
            chain,
            contract,
            dex,
            dex_version,
            token_in,
            token_out,
            fee,
            caller,
            recipient,
            amount_in,
            amount_out,
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

    pub fn get_fee(&self) -> Option<U256> {
        self.fee.clone()
    }

    pub fn get_caller(&self) -> H160 {
        self.caller.clone()
    }

    pub fn get_recipient(&self) -> H160 {
        self.recipient.clone()
    }

    pub fn get_amount_in(&self) -> U256 {
        self.amount_in.clone()
    }

    pub fn get_amount_out(&self) -> U256 {
        self.amount_out.clone()
    }
}
