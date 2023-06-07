
# auth Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10020|Login|address, signature_text, signature, service, device_id, device_os|address, user_id, user_token, admin_token||
|10010|Signup|address, signature_text, signature, email, phone, agreed_tos, agreed_privacy, username|address, user_id||
|10030|Authorize|address, token, service, device_id, device_os|success||
|10040|Logout||||

# user Server
ID: 2
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|20040|UserFollowStrategy|strategy_id|success|User follows a strategy|
|20050|UserListFollowedStrategies||strategies|User lists followed strategies|
|20060|UserUnfollowStrategy|strategy_id|success||
|20061|UserListStrategies||strategies|User lists strategies|
|20063|UserListTopPerformingStrategies||strategies|User lists top performing strategies|
|20062|UserGetStrategy|strategy_id|strategy_id, strategy_name, strategy_description, creator_user_id, social_media, historical_return, inception_time, total_amount, token_allocation, reputation, risk_score, aum, net_value, followers, backers, watching_wallets, aum_history|User gets a strategy|
|20070|UserGetStrategyStatistics|strategy_id|strategy_id, net_value, follow_history, back_history|User gets a strategy statistics|
|20071|UserGetStrategiesStatistics||tracking_amount_usd, backing_amount_usd, difference_amount_usd, aum_value_usd, current_value_usd, withdrawable_value_usd|User gets statistics of all strategies related to the user|
|20080|UserBackStrategy|strategy_id, quantity, blockchain|success||
|20081|UserRequestRefund|quantity, wallet_address, blockchain|success||
|20090|UserListBackedStrategies||strategies||
|20100|UserListBackStrategyHistory||back_history||
|20120|UserListExitStrategyHistory|strategy_id|exit_history||
|20130|UserFollowExpert|expert_id|success|User follows an expert|
|20140|UserListFollowedExperts||experts|User lists followed experts|
|20150|UserUnfollowExpert|expert_id|success|User unfollows an expert|
|20160|UserListExperts||experts|User lists experts|
|20161|UserListTopPerformingExperts||experts|User lists experts|
|20162|UserListFeaturedExperts||experts|User lists experts|
|20170|UserGetExpertProfile|expert_id|expert_id, name, follower_count, description, social_media, risk_score, reputation_score, aum, strategies|User gets an expert profile|
|20180|UserGetUserProfile|user_id|user_id, name, follower_count, description, social_media, followed_experts, followed_strategies, backed_strategies|User gets an user profile|
|20190|UserRegisterWallet|blockchain, wallet_address, message_to_sign, message_signature|success, wallet_id|User registers a wallet|
|20200|UserListRegisteredWallets||wallets|User lists wallets|
|20210|UserDeregisterWallet|wallet_id|success|User deregisters a wallet|
|20220|UserApplyBecomeExpert||success|User applies to become an expert|
|20250|UserCreateStrategy|name, description|success, strategy_id|User makes a strategy|
|20260|UserUpdateStrategy|strategy_id, name, description, social_media, risk_score, reputation_score, aum|success|User updates a strategy|
|20270|UserAddStrategyWatchingWallet|strategy_id, blockchain, wallet_address, ratio|success, wallet_id||
|20280|UserRemoveStrategyWatchingWallet|wallet_id|success||
|20290|UserListStrategyWatchingWallets|strategy_id|wallets||
|20300|UserListWalletActivityHistory|wallet_address, blockchain|wallet_activities||
|20310|UserAddStrategyInitialTokenRatio|strategy_id, token_name, token_address, quantity|success, token_id||
|20320|UserRemoveStrategyInitialTokenRatio|strategy_id, token_id|success||
|20330|UserListStrategyInitialTokenRatio|strategy_id|token_ratios||

# admin Server
ID: 3
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|30010|AdminListUsers|limit, offset, user_id, address, username, email, role|users||
|30020|AdminSetUserRole|user_id, role|||
|30030|AdminSetBlockUser|user_id, blocked|||
|20240|AdminListPendingExpertApplications||users|Admin approves a user to become an expert|
|20230|AdminApproveUserBecomeExpert|user_id|success|Admin approves a user to become an expert|
|20231|AdminRejectUserBecomeExpert|user_id|success|Admin approves a user to become an expert|

# watcher Server
ID: 4
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
