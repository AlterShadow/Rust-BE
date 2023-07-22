use eyre::*;
use lazy_static::lazy_static;
use tiny_keccak::Keccak;
use web3::contract::{Contract, Options};
use web3::types::{Address, H256};

const ENS_MAINNET_ADDR: &str = "314159265dD8dbb310642f98f50C066173C1259b";
const ENS_REVERSE_REGISTRAR_DOMAIN: &str = "addr.reverse";

struct EnsSetting {
    mainnet_addr: Address,
}

lazy_static! {
    static ref ENS_SETTING: EnsSetting = EnsSetting {
        mainnet_addr: ENS_MAINNET_ADDR
            .parse()
            .expect("don't parse ens.mainnet.addr")
    };
}

#[derive(Debug)]
struct Resolver<T: web3::Transport> {
    contract: Contract<T>,
}

impl<T: web3::Transport> Resolver<T> {
    async fn new(ens: &ENS<T>, resolver_addr: &str) -> Result<Self> {
        let addr_namehash = H256::from_slice(namehash(resolver_addr).as_slice());
        let result =
            ens.contract
                .query("resolver", (addr_namehash,), None, Options::default(), None);
        let resolver_addr: Address = result.await.with_context(|| {
            format!("get resolver of addr: {} failed", addr_namehash.to_string())
        })?;

        // resolve
        let resolver_contract = Contract::from_json(
            ens.web3.eth(),
            resolver_addr,
            include_bytes!("./PublicResolver.abi"),
        )
        .context("fail load resolver contract")?;
        Ok(Self {
            contract: resolver_contract,
        })
    }

    async fn address(self, name: &str) -> Result<Address> {
        let name_namehash = H256::from_slice(namehash(name).as_slice());
        let result = self
            .contract
            .query("addr", (name_namehash,), None, Options::default(), None);
        result
            .await
            .with_context(|| format!("get address of name: {} failed", name))
    }

    async fn name(self, name: &str) -> Result<String> {
        let addr_namehash = H256::from_slice(namehash(name).as_slice());
        let result = self
            .contract
            .query("name", (addr_namehash,), None, Options::default(), None);
        result.await.with_context(|| {
            format!(
                "get name of resolver_addr: {} failed",
                addr_namehash.to_string()
            )
        })
    }

    async fn text(self, addr: &str, key: &str) -> Result<String> {
        let key1 = key.to_owned();
        let addr_namehash = H256::from_slice(namehash(addr).as_slice());
        let result = self.contract.query(
            "text",
            (addr_namehash, key1),
            None,
            Options::default(),
            None,
        );
        result
            .await
            .with_context(|| format!("get text of addr: {}, key: {} failed", addr, key))
    }
}

#[derive(Debug)]
pub struct ENS<T: web3::Transport> {
    pub web3: web3::Web3<T>,
    pub contract: Contract<T>,
}

impl<T: web3::Transport> ENS<T> {
    pub fn new(web3: web3::Web3<T>) -> Self {
        let contract = Contract::from_json(
            web3.eth(),
            ENS_SETTING.mainnet_addr,
            include_bytes!("./ENS.abi"),
        )
        .expect("fail contract::from_json(ENS.abi)");
        ENS { web3, contract }
    }

    pub async fn name(&self, address: Address) -> Result<String> {
        let resolver_addr = format!("{:x}.{}", address, ENS_REVERSE_REGISTRAR_DOMAIN);
        let resolver = Resolver::new(self, resolver_addr.as_str()).await?;
        resolver.name(resolver_addr.as_str()).await
    }

    pub async fn owner(&self, name: &str) -> Result<Address> {
        let ens_namehash = H256::from_slice(namehash(name).as_slice());
        let result = self
            .contract
            .query("owner", (ens_namehash,), None, Options::default(), None);
        result
            .await
            .with_context(|| format!("get owner of name: {} failed", name))
    }

    pub async fn address(&self, name: &str) -> Result<Address> {
        let resolver = Resolver::new(self, name).await?;
        resolver.address(name).await
    }
    pub async fn text(&self, addr: &str, key: &str) -> Result<String> {
        let resolver = Resolver::new(self, addr).await?;
        resolver.text(addr, key).await
    }
}

fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];
    if name.is_empty() {
        return node;
    }
    let mut labels: Vec<&str> = name.split(".").collect();
    labels.reverse();
    for label in labels.iter() {
        let mut labelhash = [0u8; 32];
        Keccak::keccak256(label.as_bytes(), &mut labelhash);
        node.append(&mut labelhash.to_vec());
        labelhash = [0u8; 32];
        Keccak::keccak256(node.as_slice(), &mut labelhash);
        node = labelhash.to_vec();
    }
    node
}

#[cfg(test)]
mod test {
    use super::namehash;
    use web3::types::Address;

    #[test]
    fn test_namehash() {
        let addresses = vec![
            ("", "0x0000000000000000000000000000000000000000"),
            ("eth", "0x93cdeb708b7545dc668eb9280176169d1c33cfd8"),
            ("foo.eth", "0xde9b09fd7c5f901e23a3f19fecc54828e9c84853"),
        ];
        for (name, address) in addresses {
            let hash_address = Address::from_slice(namehash(name).as_slice());
            let h = format!("{:?}", hash_address);
            assert_eq!(address.to_string(), h);
        }
    }
}
