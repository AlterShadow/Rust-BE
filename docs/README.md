
# auth Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10020|Login|address, signature_text, signature, service_code, device_id, device_os|address, user_id, user_token, admin_token||
|10010|Signup|address, signature_text, signature, email, phone, agreed_tos, agreed_privacy, username|address, user_id||
|10030|Authorize|address, token, service_code, device_id, device_os|success||

# user Server
ID: 2
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|20040|UserFollowStrategy|strategy_id|success|User follows a strategy|
|20050|UserListFollowedStrategies||strategies|User lists followed strategies|
|20060|UserUnfollowStrategy|strategy_id|success||
|20061|UserListStrategies||strategies|User lists followed strategies|
|20062|UserGetStrategy|strategy_id|strategy_id, strategy_name, strategy_description, creator_user_id, social_media, historical_return, inception_time, total_amount, token_allocation, reputation, risk_score, aum, net_value, followers, backers, watching_wallets, aum_history|User gets a strategy|
|20070|UserGetStrategyStatistics|strategy_id|strategy_id, net_value, follow_history, back_history|User gets a strategy statistics|
|20080|UserBackStrategy|strategy_id, quantity, blockchain, dex, transaction_hash|success||
|20090|UserListBackedStrategies||strategies||
|20100|UserListBackStrategyHistory||back_history||
|20110|UserExitStrategy|strategy_id, quantity|success, transaction_hash||
|20120|UserListExitStrategyHistory|strategy_id|exit_history||
|20130|UserFollowExpert|expert_id|success|User follows an expert|
|20140|UserListFollowedExperts||experts|User lists followed experts|
|20150|UserUnfollowExpert|expert_id|success|User unfollows an expert|
|20160|UserListExperts||experts|User lists experts|
|20170|UserGetExpertProfile|expert_id|expert_id, name, follower_count, description, social_media, risk_score, reputation_score, aum, strategies|User gets an expert profile|
|20180|UserGetUserProfile|user_id|user_id, name, follower_count, description, social_media, followed_experts, followed_strategies, backed_strategies|User gets an user profile|
|20190|UserRegisterWallet|blockchain, wallet_address, message_to_sign, message_signature|success, wallet_id|User registers a wallet|
|20200|UserListWallets||wallets|User lists wallets|
|20210|UserDeregisterWallet|wallet_id|success|User deregisters a wallet|
|20220|UserApplyBecomeExpert||success|User applies to become an expert|
|20230|AdminApproveUserBecomeExpert|user_id|success|Admin approves a user to become an expert|
|20231|AdminRejectUserBecomeExpert|user_id|success|Admin approves a user to become an expert|
|20240|AdminListPendingExpertApplications||users|Admin approves a user to become an expert|
|20250|UserCreateStrategy|name, description|success, strategy_id|User makes a strategy|
|20260|UserUpdateStrategy|strategy_id, name, description, social_media, risk_score, reputation_score, aum|success|User updates a strategy|
|20270|UserAddStrategyWatchingWallet|strategy_id, blockchain, wallet_address, ratio|success, wallet_id||
|20280|UserRemoveStrategyWatchingWallet|wallet_id|success||
|20290|UserListStrategyWatchingWallets|strategy_id|wallets||

# admin Server
ID: 3
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|

# watcher Server
ID: 4
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
