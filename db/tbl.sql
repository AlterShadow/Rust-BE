-- Created by Vertabelo (http://vertabelo.com)
-- Last modification date: 2023-07-04 14:11:34.941

CREATE SCHEMA IF NOT EXISTS tbl;;

-- tables
-- Table: audit_rule
CREATE TABLE tbl.audit_rule (
    pkey_id bigint  NOT NULL,
    name varchar(32)  NOT NULL,
    description varchar(512)  NOT NULL,
    CONSTRAINT audit_rule_pk PRIMARY KEY (pkey_id)
);

-- Table: aum_history
CREATE TABLE tbl.aum_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_aum_history_id'),
    fkey_strategy_id bigint  NOT NULL,
    base_token varchar(20)  NOT NULL,
    quote_token varchar(20)  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    dex varchar(20)  NOT NULL,
    wallet_address varchar(64)  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    action varchar(8)  NOT NULL,
    price double precision  NOT NULL,
    quantity varchar(64)  NOT NULL,
    current_price double precision  NULL,
    yield_7d double precision  NULL,
    yield_30d double precision  NULL,
    CONSTRAINT aum_history_pk PRIMARY KEY (pkey_id)
);

-- Table: authorization_attempt
CREATE TABLE tbl.authorization_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_authorization_attempt_id' ),
    fkey_user bigint  NOT NULL,
    ip_address inet  NOT NULL,
    is_token_ok boolean  NOT NULL,
    moment bigint  NOT NULL,
    CONSTRAINT "tbl.authorization_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: bad_request
CREATE TABLE tbl.bad_request (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_bad_request_id' ),
    fkey_user bigint  NOT NULL,
    ip_address inet  NOT NULL,
    method_code integer  NULL,
    error_code integer  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL DEFAULT null,
    raw varchar(16384)  NULL,
    moment bigint  NOT NULL,
    CONSTRAINT "tbl.bad_request_pk" PRIMARY KEY (pkey_id)
);

-- Table: escrow_contract_address
CREATE TABLE tbl.escrow_contract_address (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_escrow_contract_address_id'),
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    CONSTRAINT escrow_contract_address_pk PRIMARY KEY (pkey_id)
);

-- Table: escrow_token_contract_address
CREATE TABLE tbl.escrow_token_contract_address (
    pkey_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    symbol varchar(32)  NOT NULL,
    short_name varchar(64)  NOT NULL,
    description varchar(256)  NOT NULL,
    address varchar(64)  NOT NULL,
    is_stablecoin boolean  NOT NULL,
    CONSTRAINT escrow_token_address_ak_1 UNIQUE (blockchain, symbol) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT escrow_token_contract_address_pk PRIMARY KEY (pkey_id)
);

-- Table: expert_listened_wallet_asset_balance
CREATE TABLE tbl.expert_listened_wallet_asset_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_expert_listened_wallet_asset_balance_id'),
    fkey_expert_watched_wallet_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT expert_listened_wallet_asset_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: expert_profile
CREATE TABLE tbl.expert_profile (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_expert_profile_id'),
    fkey_user_id bigint  NOT NULL,
    description varchar  NULL,
    social_media varchar  NULL,
    consistent_score double precision  NULL,
    followers int  NULL,
    backers int  NULL,
    risk_score double precision  NULL,
    reputation_score double precision  NULL,
    aum double precision  NULL,
    pending_expert boolean  NOT NULL DEFAULT TRUE,
    approved_expert boolean  NOT NULL DEFAULT FALSE,
    updated_at bigint  NOT NULL,
    created_at bigint  NOT NULL,
    approved_at bigint  NULL,
    requested_at bigint  NULL,
    CONSTRAINT expert_profile_pk PRIMARY KEY (pkey_id)
);

-- Table: expert_watched_wallet
CREATE TABLE tbl.expert_watched_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_watching_wallet_id'),
    fkey_user_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT expert_watched_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: last_dex_trade_for_pair
CREATE TABLE tbl.last_dex_trade_for_pair (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_last_dex_trade_for_pair_id'),
    transaction_hash varchar(80)  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    dex enum_dex  NOT NULL,
    fkey_token_in bigint  NOT NULL,
    fkey_token_out bigint  NOT NULL,
    amount_in varchar(64)  NOT NULL,
    amount_out varchar(64)  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT last_dex_trade_for_pair_pk PRIMARY KEY (pkey_id)
);

-- Table: login_attempt
CREATE TABLE tbl.login_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_login_attempt_id' ),
    fkey_user bigint  NULL,
    address varchar(64)  NOT NULL,
    ip_address inet  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL,
    is_password_ok boolean  NULL DEFAULT null,
    moment bigint  NOT NULL,
    CONSTRAINT "tbl.login_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: strategy
CREATE TABLE tbl.strategy (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_id'),
    name varchar  NOT NULL,
    fkey_user_id bigint  NULL,
    description varchar  NOT NULL,
    social_media varchar  NULL,
    historical_return double precision  NULL,
    inception_time bigint  NULL,
    current_usdc varchar(64)  NOT NULL,
    total_backed_usdc varchar(64)  NOT NULL,
    total_exited_usdc varchar(64)  NOT NULL,
    reputation int  NULL,
    risk_score double precision  NULL,
    AUM double precision  NULL,
    apy double precision  NULL,
    swap_fee double precision  NULL,
    strategy_thesis_url varchar(256)  NULL,
    agreed_tos boolean  NOT NULL DEFAULT FALSE,
    minimum_backing_amount_usd double precision  NULL,
    expert_fee double precision  NULL,
    strategy_fee double precision  NULL,
    updated_at bigint  NOT NULL,
    created_at bigint  NOT NULL,
    requested_at bigint  NULL,
    pending_approval boolean  NOT NULL DEFAULT FALSE,
    approved boolean  NOT NULL DEFAULT FALSE,
    approved_at bigint  NULL,
    immutable_audit_rules boolean  NOT NULL DEFAULT FALSE,
    blockchain enum_block_chain  NOT NULL,
    strategy_pool_address varchar(64)  NULL,
    CONSTRAINT strategy_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_audit_rule
CREATE TABLE tbl.strategy_audit_rule (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_audit_rule'),
    fkey_strategy_id bigint  NOT NULL,
    fkey_audit_rule_id bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT strategy_audit_rule_ak_1 UNIQUE (fkey_strategy_id, fkey_audit_rule_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_audit_rule_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_escrow_pending_wallet_address
CREATE TABLE tbl.strategy_escrow_pending_wallet_address (
    pkey_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    address varchar(256)  NOT NULL,
    created_at int  NOT NULL,
    CONSTRAINT strategy_escrow_pending_wallet_address_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_escrow_pending_wallet_balance
CREATE TABLE tbl.strategy_escrow_pending_wallet_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_escrow_pending_wallet_balance_id'),
    fkey_strategy_pending_wallet_address_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT strategy_escrow_pending_wallet_ledger_ak_1 UNIQUE (fkey_strategy_pending_wallet_address_id, blockchain, fkey_token_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_escrow_pending_wallet_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_initial_token_ratio
CREATE TABLE tbl.strategy_initial_token_ratio (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_initial_token_ratio_id'),
    fkey_strategy_id bigint  NOT NULL,
    token_id bigint  NOT NULL,
    quantity varchar(64)  NOT NULL,
    updated_at bigint  NOT NULL,
    created_at bigint  NOT NULL,
    fkey_token_id_relative_to bigint  NULL,
    relative_token_ratio varchar(64)  NULL,
    CONSTRAINT strategy_initial_token_ratio_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_pool_contract
CREATE TABLE tbl.strategy_pool_contract (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_pool_contract_id'),
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT strategy_pool_contract_ak_1 UNIQUE (fkey_strategy_id, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_pool_contract_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_pool_contract_asset_balance
CREATE TABLE tbl.strategy_pool_contract_asset_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_pool_contract_asset_balance_id'),
    fkey_strategy_pool_contract_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT strategy_pool_contract_asset_balance_ak_1 UNIQUE (fkey_strategy_pool_contract_id, fkey_token_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_pool_contract_asset_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_watched_wallet
CREATE TABLE tbl.strategy_watched_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_watching_wallet_id'),
    fkey_expert_watched_wallet_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    ratio_distribution double precision  NOT NULL,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT strategy_watched_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_watching_wallet_trade_ledger
CREATE TABLE tbl.strategy_watching_wallet_trade_ledger (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_watching_wallet_activity_ledger_id'),
    expert_watched_wallet_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    dex varchar(20)  NULL,
    contract_address varchar(64)  NOT NULL,
    fkey_token_in bigint  NULL,
    fkey_token_out bigint  NULL,
    amount_in varchar(64)  NULL,
    amount_out varchar(64)  NULL,
    heppened_at bigint  NOT NULL,
    CONSTRAINT wallet_activity_history_ak_1 UNIQUE (transaction_hash) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_watching_wallet_trade_ledger_pk PRIMARY KEY (pkey_id)
);

CREATE INDEX strategy_watching_wallet_trade_history_idx_1 on tbl.strategy_watching_wallet_trade_ledger (expert_watched_wallet_id ASC,blockchain ASC);

-- Table: strategy_whitelisted_token
CREATE TABLE tbl.strategy_whitelisted_token (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_whitelisted_token_id'),
    fkey_strategy_id bigint  NOT NULL,
    token_name varchar(32)  NOT NULL,
    CONSTRAINT strategy_whitelisted_token_pk PRIMARY KEY (pkey_id)
);

-- Table: system_config
CREATE TABLE tbl.system_config (
    pkey_id bigint  NOT NULL,
    config_placeholder_1 bigint  NULL,
    config_placeholder_2 int  NULL,
    CONSTRAINT system_config_pk PRIMARY KEY (pkey_id)
);

-- Table: transaction_cache
CREATE TABLE tbl.transaction_cache (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_transaction_cache_id'),
    transaction_hash varchar(80)  NOT NULL,
    blockchain varchar(20)  NOT NULL,
    dex varchar(20)  NULL,
    raw_content varchar(8192)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT transaction_cache_ak_1 UNIQUE (transaction_hash) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT transaction_cache_pk PRIMARY KEY (pkey_id)
);

-- Table: user
CREATE TABLE tbl."user" (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_user_id' ),
    public_id bigint  NOT NULL,
    username varchar(32)  NOT NULL,
    role enum_role  NOT NULL,
    address varchar(64)  NOT NULL,
    age int  NULL,
    preferred_language varchar(5)  NOT NULL,
    family_name varchar(32)  NULL,
    given_name varchar(32)  NULL,
    agreed_tos boolean  NOT NULL,
    agreed_privacy boolean  NOT NULL,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    email varchar(320)  NULL,
    phone_number varchar(15)  NULL,
    last_ip inet  NULL,
    last_login_at bigint  NULL,
    login_count int  NOT NULL DEFAULT 0,
    user_device_id varchar  NULL,
    admin_device_id varchar  NULL,
    user_token uuid  NULL,
    admin_token uuid  NULL,
    is_blocked boolean  NOT NULL DEFAULT FALSE,
    CONSTRAINT uidx_public_id UNIQUE (public_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT uidx_address UNIQUE (address) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_pk PRIMARY KEY (pkey_id)
);

-- Table: user_back_exit_strategy_ledger
CREATE TABLE tbl.user_back_exit_strategy_ledger (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_back_exit_strategy_ledger_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    quantity_of_usdc varchar(64)  NOT NULL,
    quantity_sp_tokens varchar(64)  NOT NULL,
    is_back boolean  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT user_back_exit_strategy_ledger_ak_1 UNIQUE (transaction_hash) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_back_exit_strategy_ledger_pk PRIMARY KEY (pkey_id)
);

-- Table: user_back_strategy_attempt
CREATE TABLE tbl.user_back_strategy_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_back_strategy_attempt_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    strategy_wallet_address varchar(64)  NOT NULL,
    log_id bigint  NOT NULL,
    back_quantity varchar(64)  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT user_back_strategy_attempt_pk PRIMARY KEY (pkey_id)
);

-- Table: user_back_strategy_log
CREATE TABLE tbl.user_back_strategy_log (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_back_strategy_log_id'),
    fkey_user_back_strategy_attempt_id bigint  NOT NULL,
    message varchar  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT user_back_strategy_log_pk PRIMARY KEY (pkey_id)
);

-- Table: user_deposit_withdraw_balance
CREATE TABLE tbl.user_deposit_withdraw_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_deposit_exit_strategy_balance_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_escrow_contract_address_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT user_deposit_withdraw_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: user_deposit_withdraw_ledger
CREATE TABLE tbl.user_deposit_withdraw_ledger (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_deposit_withdraw_strategy_ledger_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    user_address varchar(64)  NOT NULL,
    escrow_contract_address varchar(64)  NOT NULL,
    fkey_escrow_contract_address_id bigint  NOT NULL,
    receiver_address varchar(64)  NOT NULL,
    quantity varchar(64)  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    is_deposit boolean  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT user_deposit_withdraw_ledger_ak_1 UNIQUE (transaction_hash) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_deposit_withdraw_ledger_pk PRIMARY KEY (pkey_id)
);

-- Table: user_follow_expert
CREATE TABLE tbl.user_follow_expert (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_follow_expert_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_expert_id bigint  NOT NULL,
    unfollowed boolean  NOT NULL DEFAULT FALSE,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT user_follow_expert_pk PRIMARY KEY (pkey_id)
);

-- Table: user_follow_strategy
CREATE TABLE tbl.user_follow_strategy (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_follow_strategy'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    unfollowed boolean  NOT NULL DEFAULT FALSE,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT user_follow_strategy_pk PRIMARY KEY (pkey_id)
);

-- Table: user_strategy_balance
CREATE TABLE tbl.user_strategy_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_strategy_balance_id'),
    fkey_strategy_pool_contract_id bigint  NOT NULL,
    fkey_user_strategy_wallet_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT user_strategy_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: user_strategy_pool_contract_asset_balance
CREATE TABLE tbl.user_strategy_pool_contract_asset_balance (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_strategy_pool_contract_asset_balance_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_pool_contract_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    CONSTRAINT user_strategy_pool_contract_asset_balance_ak_1 UNIQUE (fkey_user_id, fkey_strategy_pool_contract_id, fkey_token_id) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_strategy_pool_contract_asset_balance_pk PRIMARY KEY (pkey_id)
);

-- Table: user_strategy_pool_contract_asset_ledger
CREATE TABLE tbl.user_strategy_pool_contract_asset_ledger (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_strategy_pool_contract_asset_ledger_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_pool_contract_id bigint  NOT NULL,
    fkey_token_id bigint  NOT NULL,
    amount varchar(64)  NOT NULL,
    is_add boolean  NOT NULL,
    happened_at bigint  NOT NULL,
    CONSTRAINT user_strategy_pool_contract_asset_ledger_pk PRIMARY KEY (pkey_id)
);

-- Table: user_strategy_wallet
CREATE TABLE tbl.user_strategy_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_wallet_id'),
    fkey_user_id bigint  NOT NULL,
    address varchar(64)  NOT NULL,
    is_platform_managed boolean  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT user_strategy_wallet_ak_1 UNIQUE (address, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_strategy_wallet_ak_2 UNIQUE (fkey_user_id, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_strategy_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: user_whitelisted_wallet
CREATE TABLE tbl.user_whitelisted_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_whitelisted_wallet_id'),
    fkey_user_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT user_registered_wallet_ak_1 UNIQUE (address, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_whitelisted_wallet_pk PRIMARY KEY (pkey_id)
);

-- foreign keys
-- Reference: aum_list_strategy (table: aum_history)
ALTER TABLE tbl.aum_history ADD CONSTRAINT aum_list_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: authorization_attempt_user (table: authorization_attempt)
ALTER TABLE tbl.authorization_attempt ADD CONSTRAINT authorization_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: bad_request_user (table: bad_request)
ALTER TABLE tbl.bad_request ADD CONSTRAINT bad_request_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: expert_listened_wallet_asset_balance_fkey (table: expert_listened_wallet_asset_balance)
ALTER TABLE tbl.expert_listened_wallet_asset_balance ADD CONSTRAINT expert_listened_wallet_asset_balance_fkey
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: expert_profile_user_follow_expert (table: user_follow_expert)
ALTER TABLE tbl.user_follow_expert ADD CONSTRAINT expert_profile_user_follow_expert
    FOREIGN KEY (fkey_expert_id)
    REFERENCES tbl.expert_profile (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: expert_watched_wallet_user (table: expert_watched_wallet)
ALTER TABLE tbl.expert_watched_wallet ADD CONSTRAINT expert_watched_wallet_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: fkey_expert_listened_wallet_asset_balance (table: expert_listened_wallet_asset_balance)
ALTER TABLE tbl.expert_listened_wallet_asset_balance ADD CONSTRAINT fkey_expert_listened_wallet_asset_balance
    FOREIGN KEY (fkey_expert_watched_wallet_id)
    REFERENCES tbl.expert_watched_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: fkey_user_deposit_withdraw_entry (table: user_deposit_withdraw_ledger)
ALTER TABLE tbl.user_deposit_withdraw_ledger ADD CONSTRAINT fkey_user_deposit_withdraw_entry
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: last_dex_trade_for_pair_escrow_token_contract_address_in (table: last_dex_trade_for_pair)
ALTER TABLE tbl.last_dex_trade_for_pair ADD CONSTRAINT last_dex_trade_for_pair_escrow_token_contract_address_in
    FOREIGN KEY (fkey_token_in)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: last_dex_trade_for_pair_escrow_token_contract_address_out (table: last_dex_trade_for_pair)
ALTER TABLE tbl.last_dex_trade_for_pair ADD CONSTRAINT last_dex_trade_for_pair_escrow_token_contract_address_out
    FOREIGN KEY (fkey_token_out)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: login_attempt_user (table: login_attempt)
ALTER TABLE tbl.login_attempt ADD CONSTRAINT login_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_audit_rule_audit_rule (table: strategy_audit_rule)
ALTER TABLE tbl.strategy_audit_rule ADD CONSTRAINT strategy_audit_rule_audit_rule
    FOREIGN KEY (fkey_audit_rule_id)
    REFERENCES tbl.audit_rule (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_audit_rule_strategy (table: strategy_audit_rule)
ALTER TABLE tbl.strategy_audit_rule ADD CONSTRAINT strategy_audit_rule_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_escrow_contract_address_strategy (table: strategy_escrow_pending_wallet_address)
ALTER TABLE tbl.strategy_escrow_pending_wallet_address ADD CONSTRAINT strategy_escrow_contract_address_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_escrow_pending_ledger_strategy (table: strategy_escrow_pending_wallet_balance)
ALTER TABLE tbl.strategy_escrow_pending_wallet_balance ADD CONSTRAINT strategy_escrow_pending_ledger_strategy
    FOREIGN KEY (fkey_strategy_pending_wallet_address_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_escrow_pending_wallet_balance (table: strategy_escrow_pending_wallet_balance)
ALTER TABLE tbl.strategy_escrow_pending_wallet_balance ADD CONSTRAINT strategy_escrow_pending_wallet_balance
    FOREIGN KEY (fkey_strategy_pending_wallet_address_id)
    REFERENCES tbl.strategy_escrow_pending_wallet_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_initial_token_ratio_escrow_token_contract_address (table: strategy_initial_token_ratio)
ALTER TABLE tbl.strategy_initial_token_ratio ADD CONSTRAINT strategy_initial_token_ratio_escrow_token_contract_address
    FOREIGN KEY (token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_initial_token_ratio_escrow_token_contract_relative (table: strategy_initial_token_ratio)
ALTER TABLE tbl.strategy_initial_token_ratio ADD CONSTRAINT strategy_initial_token_ratio_escrow_token_contract_relative
    FOREIGN KEY (fkey_token_id_relative_to)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_pool_contract_asset_balance_escrow_token_cont_out (table: strategy_pool_contract_asset_balance)
ALTER TABLE tbl.strategy_pool_contract_asset_balance ADD CONSTRAINT strategy_pool_contract_asset_balance_escrow_token_cont_out
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_pool_contract_asset_balance_strategy_pool_contract (table: strategy_pool_contract_asset_balance)
ALTER TABLE tbl.strategy_pool_contract_asset_balance ADD CONSTRAINT strategy_pool_contract_asset_balance_strategy_pool_contract
    FOREIGN KEY (fkey_strategy_pool_contract_id)
    REFERENCES tbl.strategy_pool_contract (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_pool_contract_strategy (table: strategy_pool_contract)
ALTER TABLE tbl.strategy_pool_contract ADD CONSTRAINT strategy_pool_contract_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_strategy_initial_token_ratio (table: strategy_initial_token_ratio)
ALTER TABLE tbl.strategy_initial_token_ratio ADD CONSTRAINT strategy_strategy_initial_token_ratio
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_strategy_watching_wallet (table: strategy_watched_wallet)
ALTER TABLE tbl.strategy_watched_wallet ADD CONSTRAINT strategy_strategy_watching_wallet
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_wallet_user (table: user_strategy_wallet)
ALTER TABLE tbl.user_strategy_wallet ADD CONSTRAINT strategy_wallet_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_watched_wallet_expert_watched_wallet (table: strategy_watched_wallet)
ALTER TABLE tbl.strategy_watched_wallet ADD CONSTRAINT strategy_watched_wallet_expert_watched_wallet
    FOREIGN KEY (fkey_expert_watched_wallet_id)
    REFERENCES tbl.expert_watched_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_watching_wallet_fkey_token_in (table: strategy_watching_wallet_trade_ledger)
ALTER TABLE tbl.strategy_watching_wallet_trade_ledger ADD CONSTRAINT strategy_watching_wallet_fkey_token_in
    FOREIGN KEY (fkey_token_in)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_watching_wallet_fkey_token_out (table: strategy_watching_wallet_trade_ledger)
ALTER TABLE tbl.strategy_watching_wallet_trade_ledger ADD CONSTRAINT strategy_watching_wallet_fkey_token_out
    FOREIGN KEY (fkey_token_out)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_watching_wallet_trade_history_expert_watched_wallet (table: strategy_watching_wallet_trade_ledger)
ALTER TABLE tbl.strategy_watching_wallet_trade_ledger ADD CONSTRAINT strategy_watching_wallet_trade_history_expert_watched_wallet
    FOREIGN KEY (expert_watched_wallet_id)
    REFERENCES tbl.expert_watched_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_whitelisted_token_strategy (table: strategy_whitelisted_token)
ALTER TABLE tbl.strategy_whitelisted_token ADD CONSTRAINT strategy_whitelisted_token_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_history_strategy (table: user_back_exit_strategy_ledger)
ALTER TABLE tbl.user_back_exit_strategy_ledger ADD CONSTRAINT user_back_strategy_history_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_history_user (table: user_back_exit_strategy_ledger)
ALTER TABLE tbl.user_back_exit_strategy_ledger ADD CONSTRAINT user_back_strategy_history_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_log_user_back_strategy_attempt (table: user_back_strategy_log)
ALTER TABLE tbl.user_back_strategy_log ADD CONSTRAINT user_back_strategy_log_user_back_strategy_attempt
    FOREIGN KEY (fkey_user_back_strategy_attempt_id)
    REFERENCES tbl.user_back_strategy_attempt (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_logs_escrow_token_contract_address (table: user_back_strategy_attempt)
ALTER TABLE tbl.user_back_strategy_attempt ADD CONSTRAINT user_back_strategy_logs_escrow_token_contract_address
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_logs_strategy (table: user_back_strategy_attempt)
ALTER TABLE tbl.user_back_strategy_attempt ADD CONSTRAINT user_back_strategy_logs_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_logs_user (table: user_back_strategy_attempt)
ALTER TABLE tbl.user_back_strategy_attempt ADD CONSTRAINT user_back_strategy_logs_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_deposit_exit_strategy_balance_escrow_address (table: user_deposit_withdraw_balance)
ALTER TABLE tbl.user_deposit_withdraw_balance ADD CONSTRAINT user_deposit_exit_strategy_balance_escrow_address
    FOREIGN KEY (fkey_escrow_contract_address_id)
    REFERENCES tbl.escrow_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_deposit_exit_strategy_balance_escrow_token (table: user_deposit_withdraw_balance)
ALTER TABLE tbl.user_deposit_withdraw_balance ADD CONSTRAINT user_deposit_exit_strategy_balance_escrow_token
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_deposit_exit_strategy_balance_user (table: user_deposit_withdraw_balance)
ALTER TABLE tbl.user_deposit_withdraw_balance ADD CONSTRAINT user_deposit_exit_strategy_balance_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_deposit_withdraw_ledger_escrow_contract_address (table: user_deposit_withdraw_ledger)
ALTER TABLE tbl.user_deposit_withdraw_ledger ADD CONSTRAINT user_deposit_withdraw_ledger_escrow_contract_address
    FOREIGN KEY (fkey_escrow_contract_address_id)
    REFERENCES tbl.escrow_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_deposit_withdraw_ledger_escrow_token_contract_address (table: user_deposit_withdraw_ledger)
ALTER TABLE tbl.user_deposit_withdraw_ledger ADD CONSTRAINT user_deposit_withdraw_ledger_escrow_token_contract_address
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_follow_expert_user (table: user_follow_expert)
ALTER TABLE tbl.user_follow_expert ADD CONSTRAINT user_follow_expert_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_follow_strategy_strategy (table: user_follow_strategy)
ALTER TABLE tbl.user_follow_strategy ADD CONSTRAINT user_follow_strategy_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_follow_strategy_user (table: user_follow_strategy)
ALTER TABLE tbl.user_follow_strategy ADD CONSTRAINT user_follow_strategy_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_profile_user (table: expert_profile)
ALTER TABLE tbl.expert_profile ADD CONSTRAINT user_profile_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_registered_wallet_user (table: user_whitelisted_wallet)
ALTER TABLE tbl.user_whitelisted_wallet ADD CONSTRAINT user_registered_wallet_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strat_pool_cont_asset_bal_strategy_pool_contract (table: user_strategy_pool_contract_asset_balance)
ALTER TABLE tbl.user_strategy_pool_contract_asset_balance ADD CONSTRAINT user_strat_pool_cont_asset_bal_strategy_pool_contract
    FOREIGN KEY (fkey_strategy_pool_contract_id)
    REFERENCES tbl.strategy_pool_contract (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy (table: strategy)
ALTER TABLE tbl.strategy ADD CONSTRAINT user_strategy
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_balance_strategy_pool_contract (table: user_strategy_balance)
ALTER TABLE tbl.user_strategy_balance ADD CONSTRAINT user_strategy_balance_strategy_pool_contract
    FOREIGN KEY (fkey_strategy_pool_contract_id)
    REFERENCES tbl.strategy_pool_contract (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_ledger_user_strategy_wallet (table: user_strategy_balance)
ALTER TABLE tbl.user_strategy_balance ADD CONSTRAINT user_strategy_ledger_user_strategy_wallet
    FOREIGN KEY (fkey_user_strategy_wallet_id)
    REFERENCES tbl.user_strategy_wallet (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_pool_cct_asset_bal_esc_token_coct_address (table: user_strategy_pool_contract_asset_balance)
ALTER TABLE tbl.user_strategy_pool_contract_asset_balance ADD CONSTRAINT user_strategy_pool_cct_asset_bal_esc_token_coct_address
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_pool_cct_asset_ldg_esc_token_contract_address (table: user_strategy_pool_contract_asset_ledger)
ALTER TABLE tbl.user_strategy_pool_contract_asset_ledger ADD CONSTRAINT user_strategy_pool_cct_asset_ldg_esc_token_contract_address
    FOREIGN KEY (fkey_token_id)
    REFERENCES tbl.escrow_token_contract_address (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_pool_contract_asset_balance_user (table: user_strategy_pool_contract_asset_balance)
ALTER TABLE tbl.user_strategy_pool_contract_asset_balance ADD CONSTRAINT user_strategy_pool_contract_asset_balance_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_pool_contract_asset_ledger_strategy_pool_contract (table: user_strategy_pool_contract_asset_ledger)
ALTER TABLE tbl.user_strategy_pool_contract_asset_ledger ADD CONSTRAINT user_strategy_pool_contract_asset_ledger_strategy_pool_contract
    FOREIGN KEY (fkey_strategy_pool_contract_id)
    REFERENCES tbl.strategy_pool_contract (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_pool_contract_asset_ledger_user (table: user_strategy_pool_contract_asset_ledger)
ALTER TABLE tbl.user_strategy_pool_contract_asset_ledger ADD CONSTRAINT user_strategy_pool_contract_asset_ledger_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- sequences
-- Sequence: seq_aum_history_id
CREATE SEQUENCE tbl.seq_aum_history_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_authorization_attempt_id
CREATE SEQUENCE tbl.seq_authorization_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_bad_request_id
CREATE SEQUENCE tbl.seq_bad_request_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_blockchain_address_lookup_cache_id
CREATE SEQUENCE tbl.seq_blockchain_address_lookup_cache_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_escrow_contract_address_id
CREATE SEQUENCE tbl.seq_escrow_contract_address_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_expert_listened_wallet_asset_balance_id
CREATE SEQUENCE tbl.seq_expert_listened_wallet_asset_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_expert_profile_id
CREATE SEQUENCE tbl.seq_expert_profile_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_last_dex_trade_for_pair_id
CREATE SEQUENCE tbl.seq_last_dex_trade_for_pair_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_login_attempt_id
CREATE SEQUENCE tbl.seq_login_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_strategy_audit_rule
CREATE SEQUENCE tbl.seq_strategy_audit_rule
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_escrow_pending_wallet_balance_id
CREATE SEQUENCE tbl.seq_strategy_escrow_pending_wallet_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_id
CREATE SEQUENCE tbl.seq_strategy_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_initial_token_ratio_id
CREATE SEQUENCE tbl.seq_strategy_initial_token_ratio_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_pool_contract_asset_balance_id
CREATE SEQUENCE tbl.seq_strategy_pool_contract_asset_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_pool_contract_id
CREATE SEQUENCE tbl.seq_strategy_pool_contract_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_wallet_id
CREATE SEQUENCE tbl.seq_strategy_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_watching_wallet_activity_ledger_id
CREATE SEQUENCE tbl.seq_strategy_watching_wallet_activity_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_watching_wallet_id
CREATE SEQUENCE tbl.seq_strategy_watching_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_whitelisted_token_id
CREATE SEQUENCE tbl.seq_strategy_whitelisted_token_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_strategy_whitelisted_tokens_id
CREATE SEQUENCE tbl.seq_strategy_whitelisted_tokens_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_transaction_cache_id
CREATE SEQUENCE tbl.seq_transaction_cache_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_back_exit_strategy_ledger_id
CREATE SEQUENCE tbl.seq_user_back_exit_strategy_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_back_strategy_attempt_id
CREATE SEQUENCE tbl.seq_user_back_strategy_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_back_strategy_log_id
CREATE SEQUENCE tbl.seq_user_back_strategy_log_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_deposit_exit_strategy_balance_id
CREATE SEQUENCE tbl.seq_user_deposit_exit_strategy_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_deposit_withdraw_strategy_ledger_id
CREATE SEQUENCE tbl.seq_user_deposit_withdraw_strategy_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_follow_expert_id
CREATE SEQUENCE tbl.seq_user_follow_expert_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_follow_strategy
CREATE SEQUENCE tbl.seq_user_follow_strategy
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_id
CREATE SEQUENCE tbl.seq_user_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_user_request_refund_ledger_id
CREATE SEQUENCE tbl.seq_user_request_refund_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_strategy_balance_id
CREATE SEQUENCE tbl.seq_user_strategy_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_strategy_pool_contract_asset_balance_id
CREATE SEQUENCE tbl.seq_user_strategy_pool_contract_asset_balance_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_strategy_pool_contract_asset_ledger_id
CREATE SEQUENCE tbl.seq_user_strategy_pool_contract_asset_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_whitelisted_wallet_id
CREATE SEQUENCE tbl.seq_user_whitelisted_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_ver_id
CREATE SEQUENCE tbl.seq_ver_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_wallet_activity_ledger_id
CREATE SEQUENCE tbl.seq_wallet_activity_ledger_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- End of file.

