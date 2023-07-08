use gen::model::EnumBlockChain;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
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
    inner: Vec<(i64, EnumBlockChain, String, Address)>,
    _phantom: PhantomData<ENUM>,
}
impl<ENUM: Copy + Eq + Hash + Debug> MultiChainAddressTable<ENUM> {
    pub fn empty() -> Self {
        Self {
            inner: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn insert(&mut self, chain: EnumBlockChain, enum_: ENUM, address: Address) {
        let id = self.inner.len() as i64;
        self.inner.push((id, chain, format!("{enum_:?}"), address));
    }
    pub fn insert_record(
        &mut self,
        index: i64,
        chain: EnumBlockChain,
        enum_: String,
        address: Address,
    ) {
        self.inner
            .push((index, chain, format!("{enum_:?}"), address));
    }
    pub fn get(&self, chain: EnumBlockChain, enum_: ENUM) -> Option<Address> {
        let enum_ = format!("{enum_:?}");
        self.inner
            .iter()
            .find(|(_id, c, e, _)| *c == chain && *e == enum_)
            .map(|(_, _, _, address)| *address)
    }
    pub fn get_by_address(&self, chain: EnumBlockChain, address: Address) -> Option<&str> {
        self.inner
            .iter()
            .find(|(_id, c, _e, a)| *c == chain && *a == address)
            .map(|(_, _, e, _)| e.as_str())
    }
    pub fn iter(&self) -> impl Iterator<Item = &(i64, EnumBlockChain, String, Address)> + '_ {
        self.inner.iter()
    }
}
