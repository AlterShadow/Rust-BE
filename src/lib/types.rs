use bytes::BytesMut;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use web3::types::{Address, H256, U256};
pub fn amount_to_display(amount: U256) -> String {
    let amount = amount.as_u128();
    let amount = amount as f64 / 1e18;
    format!("{:.4}", amount)
}
pub fn amount_from_display(s: &str) -> U256 {
    let amount = s.parse::<f64>().unwrap();
    U256::from((amount * 1e18) as u128)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockchainDecimal(pub U256);

impl Serialize for BlockchainDecimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = amount_to_display(self.0);
        value.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BlockchainDecimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // we accept either f64 or string
        let value = String::deserialize(deserializer)?;
        let value = amount_from_display(&value);
        Ok(BlockchainDecimal(value))
    }
}
impl ToSql for BlockchainDecimal {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.0.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_string().to_sql_checked(ty, out)
    }
}
impl<'a> FromSql<'a> for BlockchainDecimal {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_sql(ty, raw)?;
        let u256 = U256::from_dec_str(&s)?;
        Ok(BlockchainDecimal(u256))
    }

    fn accepts(ty: &Type) -> bool {
        <String as FromSql>::accepts(ty)
    }
}
impl From<U256> for BlockchainDecimal {
    fn from(u256: U256) -> Self {
        BlockchainDecimal(u256)
    }
}
impl Into<U256> for BlockchainDecimal {
    fn into(self) -> U256 {
        self.0
    }
}
impl Deref for BlockchainDecimal {
    type Target = U256;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockchainAddress(pub Address);
impl Serialize for BlockchainAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", self.0).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BlockchainAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // we accept either f64 or string
        let value = String::deserialize(deserializer)?;
        let value = Address::from_str(&value).unwrap();
        Ok(BlockchainAddress(value))
    }
}
impl ToSql for BlockchainAddress {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.0.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_string().to_sql_checked(ty, out)
    }
}
impl<'a> FromSql<'a> for BlockchainAddress {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_sql(ty, raw)?;
        let address = Address::from_str(&s)?;
        Ok(BlockchainAddress(address))
    }

    fn accepts(ty: &Type) -> bool {
        <String as FromSql>::accepts(ty)
    }
}
impl From<Address> for BlockchainAddress {
    fn from(address: Address) -> Self {
        BlockchainAddress(address)
    }
}
impl Into<Address> for BlockchainAddress {
    fn into(self) -> Address {
        self.0
    }
}
impl Deref for BlockchainAddress {
    type Target = Address;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BlockchainTransactionHash(pub H256);
impl Serialize for BlockchainTransactionHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:?}", self.0).serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for BlockchainTransactionHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // we accept either f64 or string
        let value = String::deserialize(deserializer)?;
        let value = H256::from_str(&value).unwrap();
        Ok(BlockchainTransactionHash(value))
    }
}
impl ToSql for BlockchainTransactionHash {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        self.0.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        <String as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_string().to_sql_checked(ty, out)
    }
}
impl<'a> FromSql<'a> for BlockchainTransactionHash {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_sql(ty, raw)?;
        let hash = H256::from_str(&s)?;
        Ok(BlockchainTransactionHash(hash))
    }

    fn accepts(ty: &Type) -> bool {
        <String as FromSql>::accepts(ty)
    }
}
impl From<H256> for BlockchainTransactionHash {
    fn from(hash: H256) -> Self {
        BlockchainTransactionHash(hash)
    }
}
impl Into<H256> for BlockchainTransactionHash {
    fn into(self) -> H256 {
        self.0
    }
}
impl Deref for BlockchainTransactionHash {
    type Target = H256;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for BlockchainTransactionHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
