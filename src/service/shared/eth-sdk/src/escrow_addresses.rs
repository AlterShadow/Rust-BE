use crate::MultiChainAddressTable;
use std::ops::{Deref, DerefMut};

pub struct EscrowAddresses(pub MultiChainAddressTable<()>);
impl EscrowAddresses {
    pub fn empty() -> Self {
        Self(MultiChainAddressTable::empty())
    }
}

impl Deref for EscrowAddresses {
    type Target = MultiChainAddressTable<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EscrowAddresses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
