use crate::method::{convert_expert_db_to_api, convert_strategy_db_to_api, ensure_user_role};
use chrono::Utc;
use eth_sdk::signer::Secp256k1SecretKey;
use eyre::ContextCompat;
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
use tracing::info;
use web3::types::U256;

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
                        address: x.address,
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
                users: ret
                    .into_rows()
                    .into_iter()
                    .map(|x| ListPendingExpertApplicationsRow {
                        user_id: x.user_public_id,
                        name: x.name,
                        linked_wallet: x.linked_wallet,
                        joined_at: x.joined_at.unwrap_or_default(),
                        requested_at: x.requested_at.unwrap_or_default(),
                        follower_count: x.follower_count as _,
                        description: x.description.unwrap_or_default(),
                        social_media: x.social_media.unwrap_or_default(),
                        risk_score: x.risk_score.unwrap_or_default(),
                        reputation_score: x.reputation_score.unwrap_or_default(),
                        aum: x.aum.unwrap_or_default(),
                    })
                    .collect(),
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

            if let Some(ret) = db
                .execute(FunAdminGetSystemConfigReq { config_id: 0 })
                .await?
                .into_result()
            {
                Ok(AdminGetSystemConfigResponse {
                    config_placeholder_1: ret.config_placeholder_1.unwrap_or_default(),
                    config_placeholder_2: ret.config_placeholder_2.unwrap_or_default(),
                })
            } else {
                Ok(AdminGetSystemConfigResponse {
                    config_placeholder_1: 0,
                    config_placeholder_2: 0,
                })
            }
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
                    config_placeholder_1: req.config_placeholder_1,
                    config_placeholder_2: req.config_placeholder_2,
                })
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
                backers: ret
                    .into_iter()
                    .map(|x| AdminListBackersRow {
                        username: x.username,
                        user_id: x.user_public_id,
                        joined_at: x.joined_at,
                        login_wallet_address: x.login_wallet_address,
                        // TODO: calculate these fees and total backing amount
                        total_platform_fee_paid: 0.0,
                        total_strategy_fee_paid: 0.0,
                        total_backing_amount: 0.0,
                    })
                    .collect(),
            })
        }
        .boxed()
    }
}
pub struct MethodAdminListStrategies;
impl RequestHandler for MethodAdminListStrategies {
    type Request = AdminListStrategiesRequest;

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
                .execute(FunAdminListStrategiesReq {
                    limit: req.limit.unwrap_or(DEFAULT_LIMIT),
                    offset: req.offset.unwrap_or(DEFAULT_OFFSET),
                    strategy_id: req.strategy_id,
                    strategy_name: req.strategy_name,
                    expert_public_id: req.expert_public_id,
                    expert_name: req.expert_name,
                    description: req.description,
                    approved: req.approved,
                    pending_approval: req.pending_approval,
                })
                .await?;

            Ok(AdminListStrategiesResponse {
                strategies: ret.map(convert_strategy_db_to_api),
            })
        }
        .boxed()
    }
}

pub struct MethodAdminApproveStrategy;
impl RequestHandler for MethodAdminApproveStrategy {
    type Request = AdminApproveStrategyRequest;

    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let db: DbClient = toolbox.get_db();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;

            db.execute(FunAdminApproveStrategyReq {
                strategy_id: req.strategy_id,
            })
            .await?;

            Ok(AdminApproveStrategyResponse { success: true })
        }
        .boxed()
    }
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
        _req: Self::Request,
    ) -> FutureResponse<Self::Request> {
        let manager = self.manger.clone();
        let toolbox = toolbox.clone();
        async move {
            ensure_user_role(ctx, EnumRole::Admin)?;
            manager.subscribe(AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll, ctx);
            // TODO: this is to provide mock data to FE. Remove this when we have real data
            tokio::spawn(async move {
                for i in 0..10 {
                    sleep(Duration::from_secs(3)).await;
                    let amount = U256::from(i);
                    let key = Secp256k1SecretKey::new_random();
                    info!("Sending mock data to FE, {}..", i);
                    manager.publish_to_all(
                        &toolbox,
                        AdminSubscribeTopic::AdminNotifyEscrowLedgerChangeAll,
                        &UserListDepositLedgerRow {
                            quantity: format!("{:?}", amount),
                            blockchain: EnumBlockChain::EthereumMainnet,
                            user_address: format!("{:?}", key.address),
                            contract_address: format!("{:?}", key.address),
                            transaction_hash: format!("{:?}", key.address),
                            receiver_address: format!("{:?}", key.address),
                            created_at: Utc::now().timestamp(),
                        },
                    )
                }
            });
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
pub struct MethodAdminAddEscrowTokenContractAddress;
impl RequestHandler for MethodAdminAddEscrowTokenContractAddress {
    type Request = AdminAddEscrowTokenContractAddressRequest;

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
                .execute(FunAdminAddEscrowTokenContractAddressReq {
                    pkey_id: req.pkey_id,
                    symbol: req.symbol,
                    short_name: req.short_name,
                    description: req.description,
                    address: req.address,
                    blockchain: req.blockchain,
                    is_stablecoin: req.is_stablecoin,
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
                    address: req.address,
                    blockchain: req.blockchain,
                })
                .await?;

            Ok(AdminAddEscrowContractAddressResponse {})
        }
        .boxed()
    }
}
