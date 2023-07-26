use crate::shared_method::{
    convert_expert_db_to_api, convert_strategy_db_to_api_net_value, ensure_user_role,
};
use api::cmc::CoinMarketCap;
use eth_sdk::erc20::Erc20Token;
use eth_sdk::logger::get_blockchain_logger;
use eth_sdk::utils::u256_to_decimal;
use eth_sdk::EthereumRpcConnectionPool;
use eyre::ContextCompat;
use eyre::*;
use futures::FutureExt;
use gen::database::*;
use gen::model::*;
use lib::database::DbClient;
use lib::handler::{FutureResponse, RequestHandler};
use lib::toolbox::{RequestContext, Toolbox};
use lib::ws::SubscribeManager;
use lib::{DEFAULT_LIMIT, DEFAULT_OFFSET};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use web3::types::Address;

pub struct MethodAdminListUsers;
impl RequestHandler for MethodAdminListUsers {
    type Request = AdminListUsersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListUsersReq {
                    user_id: req.user_id,
                    address: None,
                    username: None,
                    email: None,
                    role: req.role,
                    limit: req.limit,
                    offset: req.offset,
                })
                .await?;

            Ok(AdminListUsersResponse {
                users_total: ret.first(|x| x.total).unwrap_or_default(),
                users: ret
                    .into_iter()
                    .map(|x| ListUserRow {
                        user_id: x.user_id,
                        address: x.address.into(),
                        last_ip: x.last_ip,
                        last_login_at: x.last_login_at,
                        username: x.username,
                        email: x.email,
                        role: x.role,
                        created_at: x.created_at,
                        updated_at: x.updated_at,
                        public_user_id: x.public_user_id,
                        login_count: x.login_count,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminSetUserRole;
impl RequestHandler for MethodAdminSetUserRole {
    type Request = AdminSetUserRoleRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let _ret = db
                .execute(FunAdminSetUserRoleReq {
                    user_id: req.user_id,
                    role: req.role,
                })
                .await?;

            Ok(AdminSetUserRoleResponse {})
        }
        .boxed()
    }
}
pub struct MethodAdminSetBlockUser;
impl RequestHandler for MethodAdminSetBlockUser {
    type Request = AdminSetBlockUserRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let _ret = db
                .execute(FunAdminSetBlockUserReq {
                    user_id: req.user_id,
                    blocked: req.blocked,
                })
                .await?;

            Ok(AdminSetBlockUserResponse {})
        }
        .boxed()
    }
}
pub struct MethodAdminApproveUserBecomeExpert;
impl RequestHandler for MethodAdminApproveUserBecomeExpert {
    type Request = AdminApproveUserBecomeExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminApproveUserBecomeExpertReq {
                    user_public_id: req.user_id,
                })
                .await?;

            Ok(AdminApproveUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to approve user become expert")?
                    .success,
            })
        }
        .boxed()
    }
}
pub struct MethodAdminRejectUserBecomeExpert;
impl RequestHandler for MethodAdminRejectUserBecomeExpert {
    type Request = AdminRejectUserBecomeExpertRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminRejectUserBecomeExpertReq {
                    user_public_id: req.user_id,
                })
                .await?;

            Ok(AdminRejectUserBecomeExpertResponse {
                success: ret
                    .into_result()
                    .context("failed to reject user become expert")?
                    .success,
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListPendingExpertApplications;
impl RequestHandler for MethodAdminListPendingExpertApplications {
    type Request = AdminListPendingExpertApplicationsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListPendingUserExpertApplicationsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                })
                .await?;

            Ok(AdminListPendingExpertApplicationsResponse {
                users_total: ret.first(|x| x.total).unwrap_or_default(),
                users: ret.map(|x| ListPendingExpertApplicationsRow {
                    user_id: x.user_public_id,
                    name: x.name,
                    linked_wallet: x.linked_wallet.into(),
                    joined_at: x.joined_at.unwrap_or_default(),
                    requested_at: x.requested_at.unwrap_or_default(),
                    follower_count: x.follower_count as _,
                    description: x.description.unwrap_or_default(),
                    social_media: x.social_media.unwrap_or_default(),
                    risk_score: x.risk_score.unwrap_or_default(),
                    reputation_score: x.reputation_score.unwrap_or_default(),
                    aum: x.aum.unwrap_or_default(),
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminGetSystemConfig;
impl RequestHandler for MethodAdminGetSystemConfig {
    type Request = AdminGetSystemConfigRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let ret = db
                .execute(FunAdminGetSystemConfigReq { config_id: 0 })
                .await?
                .into_result();
            let escrow_contract_address = db
                .execute(FunUserListEscrowContractAddressReqReq { blockchain: None })
                .await?;
            let get_contract_address = |blockchain| {
                escrow_contract_address
                    .iter()
                    .find(|x| x.blockchain == blockchain)
                    .map(|x| x.address)
                    .unwrap_or_default()
            };
            let x = AdminGetSystemConfigResponse {
                platform_fee: ret
                    .as_ref()
                    .map(|x| x.platform_fee)
                    .flatten()
                    .unwrap_or_default(),
                allow_domain_urls: ret
                    .as_ref()
                    .map(|x| x.allow_domain_urls.clone())
                    .flatten()
                    .unwrap_or_default(),
                escrow_contract_address_ethereum: get_contract_address(
                    EnumBlockChain::EthereumMainnet,
                )
                .into(),
                escrow_contract_address_goerli: get_contract_address(
                    EnumBlockChain::EthereumGoerli,
                )
                .into(),
                escrow_contract_address_bsc: get_contract_address(EnumBlockChain::BscMainnet)
                    .into(),
                escrow_contract_address_bsc_testnet: get_contract_address(
                    EnumBlockChain::BscTestnet,
                )
                .into(),
            };
            Ok(x)
        }
        .boxed()
    }
}
pub struct MethodAdminUpdateSystemConfig;
impl RequestHandler for MethodAdminUpdateSystemConfig {
    type Request = AdminUpdateSystemConfigRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let _ret = db
                .execute(FunAdminUpdateSystemConfigReq {
                    config_id: 0,
                    platform_fee: req.platform_fee,
                    allow_domain_urls: req.allow_domain_urls,
                })
                .await?;
            let update_escrow_contract_address =
                |blockchain: EnumBlockChain, address: Option<Address>| {
                    let db = &db;
                    async move {
                        if let Some(address) = address {
                            db.execute(FunAdminUpdateEscrowContractAddressReq {
                                blockchain,
                                address: address.into(),
                            })
                            .await?;
                        }

                        Ok::<_, Error>(())
                    }
                    .boxed()
                };
            update_escrow_contract_address(
                EnumBlockChain::EthereumMainnet,
                req.escrow_contract_address_ethereum,
            )
            .await?;
            update_escrow_contract_address(
                EnumBlockChain::EthereumGoerli,
                req.escrow_contract_address_goerli,
            )
            .await?;
            update_escrow_contract_address(
                EnumBlockChain::BscMainnet,
                req.escrow_contract_address_bsc,
            )
            .await?;
            update_escrow_contract_address(
                EnumBlockChain::BscTestnet,
                req.escrow_contract_address_bsc_testnet,
            )
            .await?;

            Ok(AdminUpdateSystemConfigResponse { success: true })
        }
        .boxed()
    }
}
pub struct MethodAdminListExperts;
impl RequestHandler for MethodAdminListExperts {
    type Request = AdminListExpertsRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListExpertsReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    expert_id: req.expert_id,
                    user_id: req.user_id,
                    user_public_id: req.user_public_id,
                    username: req.username,
                    family_name: req.family_name,
                    given_name: req.given_name,
                    description: req.description,
                    social_media: req.social_media,
                })
                .await?;

            Ok(AdminListExpertsResponse {
                experts_total: ret.first(|x| x.total).unwrap_or_default(),
                experts: ret.map(convert_expert_db_to_api),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListBackers;
impl RequestHandler for MethodAdminListBackers {
    type Request = AdminListBackersRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListBackersReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    user_id: req.user_id,
                    user_public_id: req.user_public_id,
                    username: req.username,
                    family_name: req.family_name,
                    given_name: req.given_name,
                })
                .await?;

            Ok(AdminListBackersResponse {
                backers_total: ret.first(|x| x.total).unwrap_or_default(),
                backers: ret.map(|x| AdminListBackersRow {
                    username: x.username,
                    user_id: x.user_public_id,
                    joined_at: x.joined_at,
                    login_wallet_address: x.login_wallet_address.into(),
                    // TODO: calculate these fees and total backing amount
                    total_platform_fee_paid: 0.0,
                    total_strategy_fee_paid: 0.0,
                    total_backing_amount: 0.0,
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListStrategies {
    pub cmc: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodAdminListStrategies {
    type Request = AdminListStrategiesRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let cmc = self.cmc.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let ret = db
                .execute(FunAdminListStrategiesReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    strategy_name: req.strategy_name,
                    expert_public_id: req.expert_id,
                    expert_name: req.expert_name,
                    description: req.description,
                    approved: req.approved,
                    pending_approval: req.pending_approval,
                })
                .await?;

            Ok(AdminListStrategiesResponse {
                strategies_total: ret.first(|x| x.total).unwrap_or_default(),
                strategies: ret
                    .map_async(|x| convert_strategy_db_to_api_net_value(x, &cmc, &db))
                    .await?,
            })
        }
        .boxed()
    }
}

pub struct MethodAdminApproveStrategy {
    pub pool: EthereumRpcConnectionPool,
}
impl RequestHandler for MethodAdminApproveStrategy {
    type Request = AdminApproveStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            db.execute(FunAdminApproveStrategyReq {
                strategy_id: req.strategy_id,
            })
            .await?;

            fetch_and_update_strategy_watched_wallet_asset_balances(&db, &pool, req.strategy_id)
                .await?;

            Ok(AdminApproveStrategyResponse { success: true })
        }
        .boxed()
    }
}

pub struct MethodAdminRefreshExpertWalletBalance {
    pub pool: EthereumRpcConnectionPool,
}
impl RequestHandler for MethodAdminRefreshExpertWalletBalance {
    type Request = AdminRefreshExpertWalletBalanceRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            fetch_and_update_strategy_watched_wallet_asset_balances(&db, &pool, req.strategy_id)
                .await?;

            Ok(AdminRefreshExpertWalletBalanceResponse { success: true })
        }
        .boxed()
    }
}

pub async fn fetch_and_update_strategy_watched_wallet_asset_balances(
    db: &DbClient,
    pool: &EthereumRpcConnectionPool,
    strategy_id: i64,
) -> Result<()> {
    let strategy_watched_wallet_rows = db
        .execute(FunUserListStrategyWatchWalletsReq { strategy_id })
        .await?
        .into_rows();

    if strategy_watched_wallet_rows.len() == 0 {
        bail!(
            "no strategy watched wallet found for strategy id: {:?}",
            strategy_id
        );
    }

    for strategy_watched_wallet in strategy_watched_wallet_rows {
        let wallet_chain = strategy_watched_wallet.blockchain;
        let wallet_address = strategy_watched_wallet.wallet_address;

        let conn = pool.get(wallet_chain).await?;

        let known_token_contract_rows = db
            .execute(FunAdminListEscrowTokenContractAddressReq {
                limit: None,
                offset: None,
                blockchain: Some(wallet_chain),
                token_address: None,
                symbol: None,
                token_id: None,
            })
            .await?
            .into_rows();

        if known_token_contract_rows.len() == 0 {
            bail!(
                "no known token contracts found in watched wallet chain: {:?}",
                wallet_chain
            );
        }

        for known_token_contract in known_token_contract_rows {
            let token_address = known_token_contract.address;
            let token_id = known_token_contract.pkey_id;
            let token_contract = Erc20Token::new(conn.clone(), token_address.into())?;
            let wallet_balance = token_contract.balance_of(wallet_address.into()).await?;

            let wallet_old_balance = db
                .execute(FunWatcherListExpertListenedWalletAssetBalanceReq {
                    limit: Some(1),
                    offset: None,
                    strategy_id: Some(strategy_id),
                    address: Some(wallet_address),
                    blockchain: Some(wallet_chain),
                    token_id: Some(token_id),
                })
                .await?
                .first(|x| x.balance)
                .unwrap_or_default();

            db.execute(FunWatcherUpsertExpertListenedWalletAssetBalanceReq {
                address: wallet_address,
                blockchain: wallet_chain,
                token_id,
                old_balance: wallet_old_balance,
                new_balance: u256_to_decimal(wallet_balance, known_token_contract.decimals as _),
            })
            .await?;
        }
    }

    Ok(())
}

pub struct MethodAdminRejectStrategy;
impl RequestHandler for MethodAdminRejectStrategy {
    type Request = AdminRejectStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            db.execute(FunAdminRejectStrategyReq {
                strategy_id: req.strategy_id,
            })
            .await?;

            Ok(AdminRejectStrategyResponse { success: true })
        }
        .boxed()
    }
}

pub struct MethodAdminAddAuditRule;
impl RequestHandler for MethodAdminAddAuditRule {
    type Request = AdminAddAuditRuleRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let _ret = db
                .execute(FunAdminAddAuditRuleReq {
                    rule_id: req.rule_id,
                    name: req.name,
                    description: req.description,
                })
                .await?;

            Ok(AdminAddAuditRuleResponse {})
        }
        .boxed()
    }
}
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AdminSubscribeTopic {
    AdminNotifyEscrowLedgerChange = 1,
    AdminNotifyEscrowLedgerChangeAll = 2,
    UserBackProgress = 3,
}
impl Into<u32> for AdminSubscribeTopic {
    fn into(self) -> u32 {
        self as _
    }
}
pub struct MethodAdminNotifyEscrowLedgerChange {
    pub manager: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodAdminNotifyEscrowLedgerChange {
    type Request = AdminNotifyEscrowLedgerChangeRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        _ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manager.clone();
        let toolbox = toolbox.clone();
        async move {
            manager.publish_with_filter(
                &toolbox,
                AdminSubscribeTopic::AdminNotifyEscrowLedgerChange,
                &req.balance,
                |ctx| ctx.user_id == req.user_id,
                // TODO: filter by blockchain
            );
            manager.publish_to_all(
                &toolbox,
                AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll,
                &req.balance,
            );

            Ok(AdminNotifyEscrowLedgerChangeResponse {})
        }
        .boxed()
    }
}
pub struct MethodAdminSubscribeDepositLedger {
    pub manger: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodAdminSubscribeDepositLedger {
    type Request = AdminSubscribeDepositLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        let toolbox = toolbox.clone();
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            manager.subscribe(AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll, ctx);
            if let Some(limit) = req.initial_data {
                let resp = db
                    .execute(FunUserListDepositWithdrawLedgerReq {
                        user_id: None,
                        limit,
                        offset: 0,
                        blockchain: req.blockchain,
                        is_deposit: Some(true),
                        is_back: None,
                        is_withdraw: None,
                    })
                    .await?;
                let manager = manager.clone();
                let toolbox = toolbox.clone();
                tokio::spawn(async move {
                    sleep(Duration::from_secs_f32(0.05)).await;
                    for row in resp.into_iter() {
                        manager.publish_with_filter(
                            &toolbox,
                            AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll,
                            &UserListDepositLedgerRow {
                                transaction_id: row.transaction_id,
                                quantity: row.quantity.into(),
                                blockchain: row.blockchain,
                                user_address: row.user_address.into(),
                                contract_address: row.contract_address.into(),
                                transaction_hash: row.transaction_hash.into(),
                                receiver_address: row.receiver_address.into(),
                                happened_at: row.happened_at,
                                is_deposit: row.is_deposit,
                            },
                            |x| x.connection_id == ctx.connection_id,
                        )
                    }
                });
            }

            Ok(AdminSubscribeDepositLedgerResponse {})
        }
        .boxed()
    }
}

pub struct MethodAdminUnsubscribeDepositLedger {
    pub manger: Arc<SubscribeManager<AdminSubscribeTopic>>,
}
impl RequestHandler for MethodAdminUnsubscribeDepositLedger {
    type Request = AdminUnsubscribeDepositLedgerRequest;

    fn handle(
        &self,
        _toolbox: &Toolbox,
        ctx: RequestContext,
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        async move {
            manager.unsubscribe(
                AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll,
                ctx.connection_id,
            );

            Ok(AdminUnsubscribeDepositLedgerResponse {})
        }
        .boxed()
    }
}
pub struct MethodAdminAddEscrowTokenContractAddress {
    pub pool: EthereumRpcConnectionPool,
}
impl RequestHandler for MethodAdminAddEscrowTokenContractAddress {
    type Request = AdminAddEscrowTokenContractAddressRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        let pool = self.pool.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let rpc = pool.get(req.blockchain).await?;
            let contract = Erc20Token::new(rpc.clone(), req.address.into())?;
            let decimals = contract.decimals().await?;
            let _ret = db
                .execute(FunAdminAddEscrowTokenContractAddressReq {
                    pkey_id: req.pkey_id,
                    symbol: req.symbol,
                    short_name: req.short_name,
                    description: req.description,
                    address: req.address.into(),
                    blockchain: req.blockchain,
                    decimals: decimals.as_u32() as _,
                    is_stablecoin: req.is_stablecoin,
                    is_wrapped: false,
                })
                .await?;

            Ok(AdminAddEscrowTokenContractAddressResponse {})
        }
        .boxed()
    }
}

pub struct MethodAdminAddEscrowContractAddress;
impl RequestHandler for MethodAdminAddEscrowContractAddress {
    type Request = AdminAddEscrowContractAddressRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();

        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            let _ret = db
                .execute(FunAdminAddEscrowContractAddressReq {
                    pkey_id: req.pkey_id,
                    address: req.address.into(),
                    blockchain: req.blockchain,
                })
                .await?;

            Ok(AdminAddEscrowContractAddressResponse {})
        }
        .boxed()
    }
}

pub struct MethodAdminListBackStrategyLedger;
impl RequestHandler for MethodAdminListBackStrategyLedger {
    type Request = AdminListBackStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let ledger = db
                .execute(FunUserListBackStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: None,
                })
                .await?;
            Ok(AdminListBackStrategyLedgerResponse {
                back_ledger_total: ledger.first(|x| x.total).unwrap_or_default(),
                back_ledger: ledger.map(|x| AdminBackStrategyLedgerRow {
                    user_id: x.user_id,
                    back_ledger_id: x.back_ledger_id,
                    strategy_id: x.strategy_id,
                    quantity: x.quantity.into(),
                    blockchain: x.blockchain,
                    transaction_hash: x.transaction_hash.into(),
                    happened_at: x.happened_at,
                }),
            })
        }
        .boxed()
    }
}

pub struct MethodAdminListExitStrategyLedger;
impl RequestHandler for MethodAdminListExitStrategyLedger {
    type Request = AdminListExitStrategyLedgerRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let ledger = db
                .execute(FunUserListExitStrategyLedgerReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    user_id: None,
                })
                .await?;
            Ok(AdminListExitStrategyLedgerResponse {
                exit_ledger_total: ledger.first(|x| x.total).unwrap_or_default(),
                exit_ledger: ledger.map(|x| AdminExitStrategyLedgerRow {
                    user_id: x.user_id,
                    back_ledger_id: x.back_ledger_id,
                    strategy_id: x.strategy_id,
                    quantity: x.quantity.into(),
                    blockchain: x.blockchain,
                    transaction_hash: x.transaction_hash.into(),
                    happened_at: x.happened_at,
                }),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminSetBlockchainLogger;
impl RequestHandler for MethodAdminSetBlockchainLogger {
    type Request = AdminSetBlockchainLoggerRequest;
    fn handle(
        &self,
        _toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            get_blockchain_logger().set_enabled(req.enabled);
            Ok(AdminSetBlockchainLoggerResponse {})
        }
        .boxed()
    }
}

pub struct MethodAdminListEscrowTokenContractAddresses {
    pub cmc_client: Arc<CoinMarketCap>,
}
impl RequestHandler for MethodAdminListEscrowTokenContractAddresses {
    type Request = AdminListEscrowTokenContractAddressesRequest;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        let cmc = self.cmc_client.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let ret = db
                .execute(FunAdminListEscrowTokenContractAddressReq {
                    limit: req.limit,
                    offset: req.offset,
                    blockchain: req.blockchain,
                    token_address: req.address.map(|x| x.into()),
                    symbol: req.symbol,
                    token_id: None,
                })
                .await?;
            Ok(AdminListEscrowTokenContractAddressesResponse {
                addresses_total: ret.first(|x| x.total).unwrap_or_default(),
                addresses: ret
                    .map_async(|x| {
                        let cmc = cmc.clone();
                        async move {
                            let row = AdminEscrowTokenContractAddressRow {
                                pkey_id: x.pkey_id,
                                price: *cmc
                                    .get_usd_prices_by_symbol(&[x.symbol.clone()])
                                    .await?
                                    .get(&x.symbol)
                                    .unwrap(),
                                symbol: x.symbol,
                                short_name: x.short_name,
                                description: x.description,
                                address: x.address.into(),
                                blockchain: x.blockchain,
                                decimals: x.decimals as _,
                                is_stablecoin: x.is_stablecoin,
                                is_wrapped: x.is_wrapped,
                            };
                            Ok(row)
                        }
                    })
                    .await?,
            })
        }
        .boxed()
    }
}
pub struct MethodAdminUpdateEscrowTokenContractAddress;
impl RequestHandler for MethodAdminUpdateEscrowTokenContractAddress {
    type Request = AdminUpdateEscrowTokenContractAddressRequest;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            let _ret = db
                .execute(FunAdminUpdateEscrowTokenContractAddressReq {
                    pkey_id: req.pkey_id,
                    symbol: req.symbol,
                    short_name: req.short_name,
                    description: req.description,
                    is_stablecoin: req.is_stablecoin,
                    is_wrapped: req.is_wrapped,
                })
                .await?;
            Ok(AdminUpdateEscrowTokenContractAddressResponse {})
        }
        .boxed()
    }
}
