use num_traits::FromPrimitive;

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
pub fn convert_strategy_db_to_api(x: FunUserStrategyRowType) -> ListStrategiesRow {
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
        expert_public_id: x.creator_public_id,
        expert_username: x.creator_username,
        expert_family_name: x.creator_family_name.unwrap_or_default(),
        expert_given_name: x.creator_given_name.unwrap_or_default(),
        blockchain: x.blockchain,
        net_value: 0.0,
        swap_price: 233.0,
        price_change: 0.97,
        reputation: 5,
        strategy_pool_token: x.strategy_pool_token.unwrap_or_default(),
        strategy_fee: x.strategy_fee.unwrap_or_default(),
        platform_fee: x.strategy_fee.unwrap_or_default() + x.expert_fee.unwrap_or_default(),
        expert_fee: x.expert_fee.unwrap_or_default(),
        swap_fee: x.swap_fee.unwrap_or_default(),
        total_fee: x.strategy_fee.unwrap_or_default()
            + x.expert_fee.unwrap_or_default()
            + x.swap_fee.unwrap_or_default(),
    }
}
pub fn convert_expert_db_to_api(x: FunUserExpertRowType) -> ListExpertsRow {
    ListExpertsRow {
        expert_id: x.expert_id,
        user_public_id: x.user_public_id,
        name: x.username,
        family_name: x.family_name,
        given_name: x.given_name,
        follower_count: x.follower_count,
        backer_count: x.backer_count,
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
