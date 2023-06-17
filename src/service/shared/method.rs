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
        net_value: 0.0,
        followers: x.followers as _,
        backers: x.backers as _,
        risk_score: x.risk_score.unwrap_or_default(),
        aum: x.aum.unwrap_or_default(),
        followed: x.followed,
        swap_price: 233.0,
        price_change: 0.97,
        wallet_address: x.linked_wallet.unwrap_or_default(),
        approved: x.approved,
        approved_at: x.approved_at,
        blockchain: x
            .linked_wallet_blockchain
            .unwrap_or(EnumBlockChain::LocalNet),
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
        linked_wallet: x.linked_wallet,
        approved_expert: x.approved_expert,
        followed: x.followed,
    }
}
