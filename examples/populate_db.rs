#[path = "../src/service/shared/audit/mod.rs"]
pub mod audit;
pub mod tools;
use crate::audit::get_audit_rules;
use eth_sdk::signer::Secp256k1SecretKey;
use eth_sdk::utils::get_signed_text;
use eth_sdk::{BlockchainCoinAddresses, EscrowAddresses};
use eyre::*;
use futures::future::join_all;
use gen::model::*;
use lib::database::drop_and_recreate_database;
use lib::log::{setup_logs, LogLevel};
use rand::random;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tools::*;
use tracing::*;

const ADMIN_KEY: (&str, &str) = (
    "d34ffbe32eea1de01e30f5cccb3f9863b07c1ac600c09bae80370fc14c899913",
    "0xDE17763e1F8785331720C419a159267780018a8A",
);

const KEYS: &[(&str, &str)] = &[
    (
        "cdfacb4a0e4798afde43ec61101ea15d23eda595dde504cec6b411f51b864168",
        "0x3feb82EbA5c8eFdF435A1f5066972056568b447e",
    ),
    (
        "b2f3fbcde603236761de7f16501bd3706e568a1e96665ffc5220e4cffe0476b5",
        "0xc90F1F40e6e1BAc3B04DF19907D81d6786ED1aaE",
    ),
    (
        "3c655aaca07fe3219a230b1883d7b3eafa7ba29638c9f180e6adaf4bab2bc863",
        "0x9141305f766374A8f5ed3Ae3Bee78d782BAF3156",
    ),
    (
        "3ad843e9efdabe0734b18cc2e0ce18b70dca955b79983b5cef2e9f1c4a5e931e",
        "0x4A78e69Ac4141f2fFecDab0E4e28b5940d2427e2",
    ),
    (
        "1452f45970220d1b3397f5e638adfd9491ba8e3530ca5ec0d021f47763730db1",
        "0xB0420bA9C9ff02e58728DFb26aE062eaf36b9c53",
    ),
    (
        "223e029c4f522e62b07f7026f5633d080230fc71512942b379f4ab28fcbfb187",
        "0x739dAeb4Ef4594Ed5e0dA1375e122833B64F6724",
    ),
    (
        "478c76a5bb6c8d53543ef45e0fa72ec914715c6cbd09442b8ac34d98df6e1604",
        "0x56e5a465aDE9450F892de8207B7726D0A7947824",
    ),
    (
        "745133dfaeed8c9024d723021cc918eefcd08c4ffa7fa939011e7874d8867fbe",
        "0x73ab5b0370Ed93A97aF7BFF8e49DF1aff08BA650",
    ),
    (
        "49615f57f93d06cff08a1248a02a869f7d92622fa228afe3b88a7d98162cf6c1",
        "0xe142023AE18355bB501f3BC0b53a8139b8C335Af",
    ),
    (
        "8cf37a134c9c72fec550f7a79a40f917a0a2d8c6911ab201d817c586b6adcfb1",
        "0xa1EB98790595C476De7d466587E5780915015A3F",
    ),
    (
        "eeb06160297de42eccfede60f60ac7b10fe10f98edb961b94a7d33b39ad4f2ac",
        "0xfC4f23a2371B4BaD6135028008689Ed7a8Ec946b",
    ),
    (
        "588862be5a689382a01d109e6d635cfcac3365b0801f51a4f45215a2b2ebde1f",
        "0x939876300143ad7F073A7C63D7b4D8E460823D45",
    ),
    (
        "9ef0fef353588e6be202b1aee6981b230c6d7a8533e17a68ded7c36a54fc4b0c",
        "0xD94ef02da4694F08F40359A93a2b004AD33fD901",
    ),
    (
        "9da86a838bde019acf942e2c537dd9b6efe7a1f4651b51b4bbbe06be97fb752d",
        "0x51Bd9080f9D4C0b2d207dDab298dC2463525Cd7c",
    ),
    (
        "ff8a53cc3bfec216c9323135f8a186c9bc39061b608e643c428744f3377dae10",
        "0x049730d4614b6376cdffB7795c22ba4f83aB3E39",
    ),
    (
        "d83540e109cace371f13031a11685be24e9bcb885a0cc7e0a895b0a8dbfd54f7",
        "0x67aFf502d520c80A772EA9e324e74C3c94b25db6",
    ),
    (
        "4d67bb08ce00591770ec12ccd42cbee94bd410ed79493d46859b67a823c63621",
        "0x9bD349A3719013B97127e2BCF69498043EdD4B59",
    ),
    (
        "927c861b4b52efac79943a357662fad925366fd0e3423ac93c147fe7315b2818",
        "0x9AB298c9bB2f446b9D6e5b091D0315ab79D74df6",
    ),
    (
        "c4ae8d946873a552350e93fac21f820c44b81628f6c9e902b4e3ac09c21a19a9",
        "0x982e6A6C5F822bf68bF98c60B6c02Ad8B4464412",
    ),
    (
        "2cfd420d48f61e1a46c8c96e7249c781e4b2c250c89226e8867adfac6ae5a220",
        "0x3C93b85d9f9191E365E23d0D6EcEEa11A7505189",
    ),
    (
        "6559e02f95130026b87c5413a3abfad88a5b160f3071e7292b7cd83b3de2ed6f",
        "0x410C2aD287d2b250B93C24b1b96F2dE636aEfb06",
    ),
    (
        "6af89fed2e7a656b5df5e5c9ad93cfa711b60cb169954a3e1fed786e73249ea6",
        "0xCB2A8d64909135b57c2774286a564c94684583Bf",
    ),
    (
        "40b2b583d6dbad5db520c1d516ef2a3deb780b2e1ae59989dc94468a65428529",
        "0x4Af31B89Be2536be9E8019F0ef5b1e36C2dF813f",
    ),
    (
        "6effb559f55eb99e489e5c20ec85b43740789504009ebecf7d7a9ec91b437ea2",
        "0xa01b9ceC765B3D837a8817F5aef60baA2788A3F8",
    ),
    (
        "9df931236e8688a428d39e1dfd9bd8905759d1dc9f2dc08e56a67e270580e007",
        "0x88f4C939A90797a337234E45B77de680dCF82D06",
    ),
    (
        "3431b0e4e254b60496147f1cd326122f8a7da8b1ff5b6000ac4ba86439221214",
        "0xE96B50B0d57d4C9C2B9b44eDD52a8ee0e0d1871e",
    ),
    (
        "4077ec8ed69dda0a3e3dedc98f65601ee88ca1431f9437cd434af1ae2b103b38",
        "0x7FE444c284A382c9c3CA34fE5ed9bB6064c642A4",
    ),
    (
        "36764cecfacd2d7790e87733cd715931d62bc1622d3dff30f6f1548c624816f0",
        "0x4650A533CBaaCA142351966524fc53E6d5390B94",
    ),
    (
        "ebaef13ea72da27acedd27735f21f5cef03c0cf9e42cb16ddae0816306a5913e",
        "0x91e09C7a91e45658Ae9AF441f7F48791bb145889",
    ),
    (
        "02452f0be52f669a3ddba357f7aa003edb2d79c9fb1cfee08fe7d024b33fa4ca",
        "0x55c3d38b7cF62c92c777F2e74C8F3530A8A206DE",
    ),
    (
        "d829aac00e1dff0a2e63314e5138170a5d7692b8c1b09ae6b9f71d2961126717",
        "0xf5d878EC53AF05a3c21179f142A9112261Ea8Ae0",
    ),
    (
        "db0f2d55074aeef9cebb9b7c0529b0f04f7229884bf487c0b0e2b94d78aad3d5",
        "0xE3A3F302b4e534E93CBdf829214344523F90A76e",
    ),
    (
        "6a23e749dcca132cd58e14ce3396e0af1562d00a035129489ad3ea876f94148d",
        "0xBC40761a95DC6330C8F609157A9803726A164FD2",
    ),
    (
        "2e45ac3197a810120d040d37ed9babab4b796cb7a809720509abae6e5f44df82",
        "0xDef78a17B59eB8850003c8a0422890d9767764b2",
    ),
    (
        "ac130d43b992f39d392b5b7d2bafb5ea2243c0bb6140bf10e52af2c4997d438f",
        "0x0ba22b988E2b7873ccf1793b33963b926c5F5fd1",
    ),
    (
        "6f30fa35a534edecf50149b10191276a63738d0d6553ade38626479d5486dec4",
        "0x533a02B0A8a9c1387f53b9832f35D2EAF1a36B20",
    ),
    (
        "1317af776695e8841adab75004bc2022180271feb7ddedaf8161996088861c46",
        "0x30565bBf5f3E91bc2f6BAa91438Cc5DfD3d8a7F6",
    ),
    (
        "9683ebc59cd9e5d7cecd3938676ab3fb5269edabb4b36d35131cc38d1b4f83f9",
        "0x8f9560E1e124b4Bc587f43dd2058c177c8Cb9d32",
    ),
    (
        "d32f5f0c3647642583ea3637f35ea9b5d986496b8632cad76b5f8d65e376472e",
        "0x9c3Cd6e0a897a6321aB890244087484002Bb0bbD",
    ),
    (
        "692be19435f753ef899c611c64a794b6388f58d89d1e8f0381df990ec4790a27",
        "0xD5E410C0660ADAF44a924D2C3E4538C28b2AECdb",
    ),
    (
        "14b31f96cf02e165d8d5e6b61d65d3b04bf751fb4217e61ec7b04477c49daa1e",
        "0x3D24D458617a6E306b1Dd6C420f2209EBa0A4dba",
    ),
    (
        "b94451e0f4369372beb447dcb0eb550e501ffb8516a0300921d5df6058256e7b",
        "0x866B7C952d25703ff649cAa661F5A9E2Ee6AE07E",
    ),
    (
        "4509a3c063aff2c9de08414062a89b9335ff73035a430e2b6c0f230aef173f9f",
        "0x8B0a0bb5E1e1c8881E3a0075df5714Ee56C9F542",
    ),
];

pub fn get_user_key(i: usize) -> Option<Secp256k1SecretKey> {
    KEYS.get(i)
        .map(|(privkey, _)| Secp256k1SecretKey::from_str(privkey).unwrap())
}
pub fn get_admin_key() -> Secp256k1SecretKey {
    Secp256k1SecretKey::from_str(ADMIN_KEY.0).unwrap()
}
fn spawn_task(f: impl Future<Output = Result<()>> + Send + 'static) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(e) = f.await {
            println!("Error: {:?}", e);
        }
    })
}
async fn populate_users() -> Result<()> {
    let admin_signer = get_admin_key();
    signup(format!("dev-{}", 0), &admin_signer.key).await?;

    for i in 0..KEYS.len() {
        let signer = get_user_key(i).unwrap();

        signup(format!("user-{}", i), &signer.key).await?;
    }
    Ok(())
}

async fn populate_audit_rules() -> Result<()> {
    let admin_signer = get_admin_key();
    let mut admin_client = connect_user("dev-0", &admin_signer.key).await?;
    for rule in get_audit_rules() {
        admin_client
            .request(AdminAddAuditRuleRequest {
                rule_id: rule.id,
                name: rule.name.to_string(),
                description: rule.description.to_string(),
            })
            .await?;
    }
    Ok(())
}
async fn populate_escrow_token_contract_address() -> Result<()> {
    let admin_signer = get_admin_key();
    let mut admin_client = connect_user("dev-0", &admin_signer.key).await?;
    let addresses = BlockchainCoinAddresses::new();
    for (i, (blockchain, coin, address)) in addresses.iter().enumerate() {
        if let Err(err) = admin_client
            .request(AdminAddEscrowTokenContractAddressRequest {
                pkey_id: i as _,
                address: format!("{:?}", address),
                symbol: coin.to_string(),
                short_name: coin.to_string(),
                blockchain,
                description: format!("This is {:?}", coin),
                is_stablecoin: match coin {
                    EnumBlockchainCoin::USDC
                    | EnumBlockchainCoin::USDT
                    | EnumBlockchainCoin::BUSD => true,
                    _ => false,
                },
            })
            .await
        {
            warn!("Error when inserting token: {:?}", err);
        }
    }
    Ok(())
}
async fn populate_escrow_contract_address() -> Result<()> {
    let admin_signer = get_admin_key();
    let mut admin_client = connect_user("dev-0", &admin_signer.key).await?;
    let addresses = EscrowAddresses::new();
    for (i, (blockchain, _, address)) in addresses.iter().enumerate() {
        if let Err(err) = admin_client
            .request(AdminAddEscrowContractAddressRequest {
                pkey_id: i as _,
                address: format!("{:?}", address),
                blockchain,
            })
            .await
        {
            warn!("Error when inserting token: {:?}", err);
        }
    }
    Ok(())
}
#[allow(unused)]
async fn populate_user_register_wallets() -> Result<()> {
    let mut tasks = vec![];
    for i in 0..KEYS.len() {
        tasks.push(spawn_task(async move {
            let signer = get_user_key(i).unwrap();

            let mut client = connect_user(format!("user-{}", i), &signer.key).await?;
            let (txt, sig) =
                get_signed_text(format!("User register wallet request {}", i), &signer.key)?;
            let _resp = client
                .request(UserRegisterWalletRequest {
                    blockchain: EnumBlockChain::LocalNet,
                    wallet_address: format!("{:?}", signer.address),
                    message_to_sign: txt,
                    message_signature: sig,
                })
                .await?;
            Ok(())
        }))
    }
    join_all(tasks).await;

    Ok(())
}

async fn populate_user_apply_become_experts() -> Result<()> {
    let mut tasks = vec![];
    let admin_signer = get_admin_key();
    let admin_client = Arc::new(Mutex::new(connect_user("dev-0", &admin_signer.key).await?));
    for i in 0..KEYS.len() / 2 {
        let admin_client = admin_client.clone();
        tasks.push(spawn_task(async move {
            let signer = get_user_key(i).unwrap();

            let (mut client, login_info) =
                connect_user_ext(format!("user-{}", i), &signer.key).await?;

            let become_expert_resp = client.request(UserApplyBecomeExpertRequest {}).await?;
            if i % 2 == 0 {
                admin_client
                    .lock()
                    .await
                    .request(AdminApproveUserBecomeExpertRequest {
                        user_id: login_info.user_id,
                    })
                    .await?;
                let mut client = connect_user(format!("user-{}", i), &signer.key).await?;

                let create_strategy_resp = client
                    .request(ExpertCreateStrategyRequest {
                        name: format!("test strategy {}", i),
                        description: "this is a test strategy".to_string(),
                        strategy_thesis_url: "".to_string(),
                        minimum_backing_amount_usd: 0.0,
                        strategy_fee: random(),
                        expert_fee: random(),
                        agreed_tos: true,
                        wallet_address: format!("{:?}", signer.address),
                        wallet_blockchain: EnumBlockChain::EthereumMainnet,
                        audit_rules: None,
                    })
                    .await?;
                info!("User Create Strategy {:?}", create_strategy_resp);

                let _resp = client
                    .request(UserFollowStrategyRequest {
                        strategy_id: create_strategy_resp.strategy_id,
                    })
                    .await?;
                client
                    .request(UserFollowExpertRequest {
                        expert_id: become_expert_resp.expert_id,
                    })
                    .await?;
            }
            Ok(())
        }))
    }
    join_all(tasks).await;
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    setup_logs(LogLevel::Debug)?;
    drop_and_recreate_database()?;
    populate_escrow_token_contract_address().await?;
    populate_escrow_contract_address().await?;
    populate_users().await?;
    populate_audit_rules().await?;
    populate_user_apply_become_experts().await?;

    Ok(())
}
