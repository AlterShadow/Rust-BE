use gen::model::EnumBlockChain;
use std::collections::HashMap;
use std::hash::Hash;
use web3::types::Address;

pub struct AddressTable<ENUM> {
    inner: HashMap<ENUM, Address>,
    reverse_inner: HashMap<Address, ENUM>,
}
impl<ENUM: Copy + Eq + Hash> AddressTable<ENUM> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            reverse_inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, enum_: ENUM, address: Address) {
        if self.inner.get(&enum_).is_some() {
            unreachable!()
        }
        if self.reverse_inner.get(&address).is_some() {
            unreachable!()
        }
        self.inner.insert(enum_, address);
        self.reverse_inner.insert(address, enum_);
    }
    pub fn get(&self, enum_: ENUM) -> Option<Address> {
        self.inner.get(&enum_).copied()
    }
    pub fn get_by_address(&self, address: Address) -> Option<ENUM> {
        self.reverse_inner.get(&address).copied()
    }
}
pub struct MultiChainAddressTable<ENUM> {
    inner: HashMap<EnumBlockChain, AddressTable<ENUM>>,
}
impl<ENUM: Copy + Eq + Hash> MultiChainAddressTable<ENUM> {
    pub fn empty() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, chain: EnumBlockChain, enum_: ENUM, address: Address) {
        let table = self.inner.entry(chain).or_insert_with(AddressTable::new);
        table.insert(enum_, address);
    }
    pub fn get(&self, chain: EnumBlockChain, enum_: ENUM) -> Option<Address> {
        self.inner.get(&chain)?.get(enum_)
    }
    pub fn get_by_address(&self, chain: EnumBlockChain, address: Address) -> Option<ENUM> {
        self.inner.get(&chain)?.get_by_address(address)
    }
}
