use api::AssetInfoClient;
use eth_sdk::{BlockchainCoinAddresses, EscrowAddresses};
use eyre::*;
use eyre::{anyhow, ensure, ContextCompat};
use gen::database::*;
use gen::model::{EnumBlockChain, EnumErrorCode, EnumRole, ListExpertsRow, ListStrategiesRow};
use lib::database::DbClient;
use lib::toolbox::{CustomError, RequestContext};
use lib::ws::WsServerConfig;
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use rust_decimal::Decimal;
use std::sync::Arc;
use tracing::info;
use web3::types::{Address, H256};

pub fn ensure_user_role(ctx: RequestContext, role: EnumRole) -> Result<()> {
    let ctx_role = EnumRole::from_u32(ctx.role).context("Invalid role")?;
    ensure!(
        ctx_role >= role,
        CustomError::new(
            EnumErrorCode::InvalidRole,
            format!("Requires {} Actual {}", role, ctx_role)
        )
    );
    Ok(())
}

fn convert_strategy_db_to_api(x: FunUserStrategyRowType) -> ListStrategiesRow {
    ListStrategiesRow {
        strategy_id: x.strategy_id,
        strategy_name: x.strategy_name,
        strategy_description: x.strategy_description,
        followers: x.followers as _,
        backers: x.backers as _,
        risk_score: x.risk_score.unwrap_or_default(),
        aum: x.aum.unwrap_or_default(),
        followed: x.followed,
        strategy_pool_address: x.strategy_pool_address.map(|x| x.into()),
        approved: x.approved,
        approved_at: x.approved_at,
        requested_at: x.requested_at,
        created_at: x.created_at,
        expert_id: x.creator_public_id,
        expert_username: x.creator_username,
        expert_family_name: x.creator_family_name.unwrap_or_default(),
        expert_given_name: x.creator_given_name.unwrap_or_default(),
        blockchain: x.blockchain,
        swap_price: 0.0,
        price_change: 0.0,
        reputation: 5,
        strategy_pool_token: x.strategy_pool_token.unwrap_or_default(),
        strategy_fee: x.platform_fee.unwrap_or_default() + x.expert_fee.unwrap_or_default(),
        platform_fee: x.platform_fee.unwrap_or_default(),
        expert_fee: x.expert_fee.unwrap_or_default(),
        swap_fee: x.swap_fee.unwrap_or_default(),
        total_fee: x.platform_fee.unwrap_or_default()
            + x.expert_fee.unwrap_or_default()
            + x.swap_fee.unwrap_or_default(),
        number_of_tokens: x.number_of_tokens.unwrap_or_default(),
        backed: x.backed,
    }
}
pub async fn convert_strategy_db_to_api_net_value(
    x: FunUserStrategyRowType,
    asset_client: &Arc<dyn AssetInfoClient>,
    db: &DbClient,
) -> Result<ListStrategiesRow> {
    let mut value = convert_strategy_db_to_api(x);
    let mut usd = Decimal::zero();
    info!(
        "Querying strategy pool tokens {} {:?}",
        value.strategy_id, value.blockchain
    );

    let tokens = db
        .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
            strategy_pool_contract_id: None,
            strategy_id: Some(value.strategy_id),
            blockchain: Some(value.blockchain),
            token_address: None,
        })
        .await?;
    info!("Tokens {:?}", tokens);
    let symbols = tokens.clone().map(|x| x.token_symbol);
    let prices = asset_client.get_usd_price_latest(&symbols).await?;
    for token in tokens.into_iter() {
        let price = prices
            .get(&token.token_symbol)
            .ok_or_else(|| anyhow!("No price for {}", token.token_symbol))?;
        usd += Decimal::from_f64(*price).unwrap() * token.balance;
    }

    value.aum = usd.to_f64().unwrap();
    Ok(value)
}
pub fn convert_expert_db_to_api(x: FunUserExpertRowType) -> ListExpertsRow {
    ListExpertsRow {
        expert_id: x.user_public_id,
        name: x.username,
        family_name: x.family_name,
        given_name: x.given_name,
        follower_count: x.follower_count,
        backer_count: x.backer_count,
        strategy_count: x.strategy_count,
        description: x.description.unwrap_or_default(),
        social_media: x.social_media.unwrap_or_default(),
        risk_score: x.risk_score.unwrap_or_default(),
        reputation_score: x.reputation_score.unwrap_or_default(),
        consistent_score: 0.5,
        aum: x.aum.unwrap_or_default(),
        joined_at: x.joined_at,
        requested_at: x.requested_at.unwrap_or_default(),
        approved_at: x.approved_at,
        pending_expert: x.pending_expert,
        linked_wallet: x.linked_wallet.into(),
        approved_expert: x.approved_expert,
        followed: x.followed,
    }
}

pub async fn load_coin_addresses(db: &DbClient) -> Result<Arc<BlockchainCoinAddresses>> {
    let mut coin_addresses = BlockchainCoinAddresses::empty();
    let coins_from_db = db
        .execute(FunUserListEscrowTokenContractAddressReq {
            limit: 10000,
            offset: 0,
            token_id: None,
            blockchain: None,
            address: None,
            symbol: None,
            is_stablecoin: None,
        })
        .await?;
    for coin in coins_from_db.into_iter() {
        coin_addresses.insert_record(
            coin.token_id,
            coin.blockchain,
            coin.symbol,
            coin.address.into(),
        );
    }
    let coin_addresses = Arc::new(coin_addresses);
    Ok(coin_addresses)
}

pub async fn load_escrow_address(db: &DbClient) -> Result<Arc<EscrowAddresses>> {
    let mut this = EscrowAddresses::empty();
    let rows = db
        .execute(FunUserListEscrowContractAddressReqReq { blockchain: None })
        .await?;
    for row in rows.into_iter() {
        this.insert(row.blockchain, (), row.address.into());
    }

    Ok(Arc::new(this))
}

pub async fn load_allow_domain_urls(db: &DbClient, config: &mut WsServerConfig) -> Result<()> {
    let system_config = db
        .execute(FunAdminGetSystemConfigReq { config_id: 0 })
        .await?
        .into_result()
        .unwrap_or_else(|| FunAdminGetSystemConfigRespRow {
            platform_fee: None,
            allow_domain_urls: None,
        });
    config.allow_cors_urls = Arc::new(
        system_config
            .allow_domain_urls
            .map(|x| x.split(";").map(|x| x.to_string()).collect()),
    );
    Ok(())
}

pub async fn update_strategy_token_balances_and_ledger_exit_strategy(
    db: &DbClient,
    blockchain: EnumBlockChain,
    strategy_id: i64,
    user_id: i64,
    redeem_transaction_hash: H256,
    redeemed_amount: Decimal,
) -> Result<()> {
    /* update user strategy token ledger */
    db.execute(FunUserExitStrategyReq {
        user_id,
        strategy_id,
        // TODO: calculate value of sp tokens exit in usdc
        quantity: Decimal::zero(),
        blockchain,
        transaction_hash: redeem_transaction_hash.into(),
        redeem_sp_tokens: redeemed_amount,
    })
    .await?;

    /* update user strategy token balance */
    let user_strategy_balance = db
        .execute(FunWatcherListUserStrategyBalanceReq {
            limit: 1,
            offset: 0,
            strategy_id: Some(strategy_id),
            user_id: Some(user_id),
            blockchain: Some(blockchain),
        })
        .await?
        .first(|x| x.balance)
        .context("could not get user strategy token balance from database on exit strategy")?;
    db.execute(FunWatcherUpsertUserStrategyBalanceReq {
        user_id,
        strategy_id,
        blockchain,
        old_balance: user_strategy_balance,
        new_balance: user_strategy_balance - redeemed_amount,
    })
    .await?;

    Ok(())
}

pub async fn update_asset_balances_and_ledger_exit_strategy(
    db: &DbClient,
    blockchain: EnumBlockChain,
    strategy_id: i64,
    strategy_pool_contract_id: i64,
    user_id: i64,
    strategy_wallet_id: i64,
    withdraw_transaction_hash: H256,
    assets_withdrawn: Vec<Address>,
    amounts_withdrawn: Vec<Decimal>,
) -> Result<()> {
    for idx in 0..assets_withdrawn.len() {
        /* update per-user strategy pool asset balance & ledger */
        let asset = assets_withdrawn[idx];
        let amount = amounts_withdrawn[idx];
        let asset_old_balance = db
            .execute(FunUserListUserStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_id),
                user_id: Some(user_id),
                strategy_wallet_id: Some(strategy_wallet_id),
                token_address: Some(asset.into()),
                blockchain: Some(blockchain),
            })
            .await?
            .into_result()
            .context("user strategy pool asset balance not found")?
            .balance;

        db.execute(FunUserUpsertUserStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id,
            strategy_wallet_id,
            token_address: asset.into(),
            blockchain,
            old_balance: asset_old_balance,
            new_balance: asset_old_balance - amount,
        })
        .await?;

        db.execute(FunUserAddUserStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id,
            strategy_wallet_id,
            token_address: asset.into(),
            blockchain,
            amount: amount.into(),
            is_add: false,
        })
        .await?;

        /* update strategy pool asset balances & ledger */
        let old_asset_balance_row = db
            .execute(FunWatcherListStrategyPoolContractAssetBalancesReq {
                strategy_pool_contract_id: Some(strategy_pool_contract_id),
                strategy_id: Some(strategy_id),
                blockchain: Some(blockchain),
                token_address: Some(asset.into()),
            })
            .await?
            .into_result()
            .context("strategy pool balance of redeemed asset not found")?;

        db.execute(FunWatcherUpsertStrategyPoolContractAssetBalanceReq {
            strategy_pool_contract_id,
            token_address: asset.into(),
            blockchain,
            new_balance: old_asset_balance_row.balance - amount,
        })
        .await?;

        db.execute(FunUserAddStrategyPoolContractAssetLedgerEntryReq {
            strategy_pool_contract_id,
            token_address: asset.into(),
            blockchain,
            amount: amount.into(),
            is_add: false,
            transaction_hash: withdraw_transaction_hash.into(),
        })
        .await?;
    }

    Ok(())
}
