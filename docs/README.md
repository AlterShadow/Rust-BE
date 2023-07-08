
# auth Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10020|Login|address, signature_text, signature, service, device_id, device_os|address, role, user_id, user_token, admin_token||
|10010|Signup|address, signature_text, signature, email, phone, agreed_tos, agreed_privacy, username|address, user_id||
|10030|Authorize|address, token, service, device_id, device_os|success||
|10040|Logout||||
|10050|ChangeLoginWallet|old_address, old_signature_text, old_signature, new_address, new_signature_text, new_signature|||

# user Server
ID: 2
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|20040|UserFollowStrategy|strategy_id|success|User follows a strategy|
|20050|UserListFollowedStrategies|limit, offset|strategies_total, strategies|User lists followed strategies|
|20060|UserUnfollowStrategy|strategy_id|success||
|20061|UserListStrategies|limit, offset, strategy_id, strategy_name, expert_public_id, expert_name, description, blockchain, strategy_pool_address|strategies_total, strategies|User lists strategies|
|20063|UserListTopPerformingStrategies|limit, offset|strategies_total, strategies|User lists top performing strategies|
|20064|UserListStrategyBackers|strategy_id, limit, offset|backers_total, backers||
|20065|UserListStrategyFollowers|strategy_id, limit, offset|followers_total, followers||
|20062|UserGetStrategy|strategy_id|strategy, watching_wallets, aum_ledger, audit_rules, whitelisted_tokens|User gets a strategy|
|20070|UserGetStrategyStatistics|strategy_id|strategy_id, net_value, follow_ledger, back_ledger|User gets a strategy statistics|
|20071|UserGetStrategiesStatistics||tracking_amount_usd, backing_amount_usd, difference_amount_usd, aum_value_usd, current_value_usd, withdrawable_value_usd|User gets statistics of all strategies related to the user|
|20172|UserUpdateUserProfile|username, family_name, given_name, description, social_media||User update its expert profile|
|20080|UserBackStrategy|strategy_id, quantity, token_id, strategy_wallet, nonce|||
|20110|UserExitStrategy|strategy_id, quantity, blockchain, nonce|success, transaction_hash||
|20081|UserRequestRefund|quantity, wallet_address, blockchain, nonce|success||
|20090|UserListBackedStrategies|limit, offset|strategies_total, strategies||
|20100|UserListBackStrategyLedger|limit, offset, strategy_id|back_ledger_total, back_ledger||
|20120|UserListExitStrategyLedger|strategy_id, limit, offset|exit_ledger_total, exit_ledger||
|20130|UserFollowExpert|expert_id|success|User follows an expert|
|20140|UserListFollowedExperts|limit, offset|experts_total, experts|User lists followed experts|
|20150|UserUnfollowExpert|expert_id|success|User unfollows an expert|
|20160|UserListExperts|limit, offset, expert_id, user_id, user_public_id, username, family_name, given_name, description, social_media, sort_by_followers|experts_total, experts|User lists experts|
|20161|UserListTopPerformingExperts|limit, offset|experts_total, experts|User lists experts|
|20162|UserListFeaturedExperts|limit, offset|experts_total, experts|User lists experts|
|20170|UserGetExpertProfile|expert_id|expert_id, name, follower_count, backers_count, description, social_media, risk_score, reputation_score, aum, followed, strategies_total, strategies|User gets an expert profile|
|20180|UserGetUserProfile||name, login_wallet, joined_at, follower_count, description, social_media, followed_experts, followed_strategies, backed_strategies|User gets an user profile|
|20190|UserWhitelistWallet|blockchain, wallet_address|success, wallet_id|User registers a wallet|
|20200|UserListWhitelistedWallets|limit, offset, wallet_id, blockchain, wallet_address, strategy_id|wallets|User lists wallets|
|20210|UserUnwhitelistWallet|wallet_id|success|User deregisters a wallet|
|20220|UserApplyBecomeExpert||success, expert_id|User applies to become an expert|
|20250|ExpertCreateStrategy|name, description, strategy_thesis_url, minimum_backing_amount_usd, expert_fee, agreed_tos, wallet_address, wallet_blockchain, strategy_token_relative_to_usdc_ratio, initial_tokens, audit_rules|success, strategy_id|User makes a strategy|
|20260|ExpertUpdateStrategy|strategy_id, name, description, social_media|success|Expert updates a strategy|
|20265|ExpertFreezeStrategy|strategy_id|success|Expert freezes a strategy, by making it immutable|
|20270|ExpertAddStrategyWatchingWallet|strategy_id, blockchain, wallet_address, ratio|success, wallet_id||
|20280|ExpertRemoveStrategyWatchingWallet|strategy_id, wallet_id|success||
|20290|UserListStrategyWatchingWallets|strategy_id|wallets_total, wallets||
|20300|UserListWalletActivityLedger|wallet_address, blockchain|wallet_activities_total, wallet_activities||
|20310|ExpertAddStrategyInitialTokenRatio|strategy_id, token_id, quantity|success, token_id||
|20320|ExpertRemoveStrategyInitialTokenRatio|strategy_id, token_id|success||
|20330|UserListStrategyInitialTokenRatio|strategy_id|token_ratios_total, token_ratios||
|20340|ExpertListFollowers|limit, offset|followers_total, followers||
|20350|ExpertListBackers|limit, offset|backers_total, backers||
|20360|UserGetDepositTokens||tokens||
|20370|UserGetDepositAddresses||addresses||
|20380|UserListDepositWithdrawLedger|limit, offset, blockchain, id_deposit|ledger_total, ledger||
|20381|UserSubscribeDepositLedger|initial_data, blockchain, mock_data|||
|20382|UserUnsubscribeDepositLedger||||
|20390|UserListStrategyWallets|blockchain|wallets_total, wallets||
|20391|UserCreateStrategyWallet|blockchain, user_managed_wallet_address|blockchain, address||
|20400|UserListStrategyAuditRules|strategy_id|audit_rules||
|20410|UserAddStrategyAuditRule|strategy_id, rule_id|||
|20420|UserRemoveStrategyAuditRule|strategy_id, rule_id|||
|20500|UserGetEscrowAddressForStrategy|strategy_id, token_id|tokens||
|20510|UserListDepositWithdrawBalances||balances||
|20511|UserGetDepositWithdrawBalance|token_id|balance||
|20520|UserListEscrowTokenContractAddresses|limit, offset, blockchain, is_stablecoin|tokens_total, tokens||
|20530|UserListStrategyTokenBalance|limit, offset, strategy_id|tokens_total, tokens||
|20540|UserGetBackStrategyReviewDetail|strategy_id, token_id, quantity|strategy_fee, total_amount_to_back, total_amount_to_back_after_fee, user_strategy_wallets, estimated_amount_of_strategy_tokens, estimated_backed_token_ratios||
|20550|UserListUserBackStrategyAttempt|limit, offset, strategy_id, token_id|total, back_attempts||
|20560|UserListUserBackStrategyLog|attempt_id, limit, offset|back_logs_total, back_logs||
|20570|UserGetSystemConfig||platform_fee|User get system config|

# admin Server
ID: 3
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|30010|AdminListUsers|limit, offset, user_id, address, username, email, role|users_total, users||
|30020|AdminSetUserRole|user_id, role|||
|30030|AdminSetBlockUser|user_id, blocked|||
|30060|AdminListPendingExpertApplications|offset, limit|users_total, users|Admin approves a user to become an expert|
|30040|AdminApproveUserBecomeExpert|user_id|success|Admin approves a user to become an expert|
|30050|AdminRejectUserBecomeExpert|user_id|success|Admin approves a user to become an expert|
|30070|AdminGetSystemConfig||platform_fee, config_placeholder_2|Admin get system config|
|30080|AdminUpdateSystemConfig|platform_fee, config_placeholder_2|success|Admin updates system config|
|30090|AdminListExperts|limit, offset, expert_id, user_id, user_public_id, username, family_name, given_name, description, social_media|experts_total, experts|Admin lists experts|
|30100|AdminListBackers|offset, limit, user_id, user_public_id, username, family_name, given_name|backers_total, backers||
|30110|AdminListStrategies|offset, limit, strategy_id, strategy_name, expert_public_id, expert_name, description, pending_approval, approved|strategies_total, strategies||
|30120|AdminApproveStrategy|strategy_id|success|Admin approves strategy|
|30130|AdminRejectStrategy|strategy_id|success||
|31002|AdminAddAuditRule|rule_id, name, description|||
|32010|AdminNotifyEscrowLedgerChange|pkey_id, user_id, balance|||
|32011|AdminSubscribeDepositLedger|initial_data, blockchain, mock_data|||
|32012|AdminUnsubscribeDepositLedger||||
|32020|AdminAddEscrowTokenContractAddress|pkey_id, symbol, short_name, description, address, blockchain, is_stablecoin|||
|32030|AdminAddEscrowContractAddress|pkey_id, address, blockchain|||
|32040|AdminListBackStrategyLedger|limit, offset, strategy_id|back_ledger_total, back_ledger||
|32050|AdminSetBlockchainLogger|enabled|||

# watcher Server
ID: 4
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
