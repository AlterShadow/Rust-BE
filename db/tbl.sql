-- Created by Vertabelo (http://vertabelo.com)
-- Last modification date: 2023-06-21 02:05:26.132

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

-- Table: blockchain_address_lookup_cache
CREATE TABLE tbl.blockchain_address_lookup_cache (
    pkey bigint  NOT NULL DEFAULT nextval('tbl.seq_blockchain_address_lookup_cache_id'),
    address bigint  NOT NULL,
    blockchain varchar(20)  NOT NULL,
    name varchar(20)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT chain_address_lookup_cache_ak_1 UNIQUE (address, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT blockchain_address_lookup_cache_pk PRIMARY KEY (pkey)
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
    evm_contract_address varchar(64)  NULL,
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

-- Table: strategy_initial_token_ratio
CREATE TABLE tbl.strategy_initial_token_ratio (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_initial_token_ratio_id'),
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    token_name varchar(20)  NOT NULL,
    token_address varchar(64)  NOT NULL,
    quantity varchar(64)  NOT NULL,
    updated_at bigint  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT strategy_initial_token_ratio_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_wallet
CREATE TABLE tbl.strategy_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_wallet_id'),
    address varchar(64)  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    fkey_user_id bigint  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT strategy_wallet_ak_1 UNIQUE (address, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT strategy_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_watching_wallet
CREATE TABLE tbl.strategy_watching_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_watching_wallet_id'),
    fkey_user_id bigint  NULL,
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    dex varchar(20)  NOT NULL,
    ratio_distribution double precision  NOT NULL,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT strategy_watching_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: strategy_whitelisted_token
CREATE TABLE tbl.strategy_whitelisted_token (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_strategy_whitelisted_token_id'),
    token_name varchar(32)  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
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

-- Table: user_back_strategy_history
CREATE TABLE tbl.user_back_strategy_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_back_strategy_history_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    quantity varchar(64)  NOT NULL,
    earn_sp_tokens varchar(64)  NOT NULL,
    back_time bigint  NOT NULL,
    CONSTRAINT user_back_strategy_history_pk PRIMARY KEY (pkey_id)
);

-- Table: user_deposit_history
CREATE TABLE tbl.user_deposit_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_deposit_history_id'),
    fkey_user_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    user_address varchar(64)  NOT NULL,
    contract_address varchar(64)  NOT NULL,
    receiver_address varchar(64)  NOT NULL,
    quantity varchar(64)  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT uidx_user_username UNIQUE (user_address) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_deposit_history_pk PRIMARY KEY (pkey_id)
);

-- Table: user_exit_strategy_history
CREATE TABLE tbl.user_exit_strategy_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_exit_strategy_history_id'),
    fkey_user_id bigint  NOT NULL,
    fkey_strategy_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    exit_quantity varchar(64)  NOT NULL,
    exit_time bigint  NOT NULL,
    CONSTRAINT user_exit_strategy_history_pk PRIMARY KEY (pkey_id)
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

-- Table: user_registered_wallet
CREATE TABLE tbl.user_registered_wallet (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_registered_wallet_id'),
    fkey_user_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    address varchar(64)  NOT NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT user_registered_wallet_ak_1 UNIQUE (address, blockchain) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_registered_wallet_pk PRIMARY KEY (pkey_id)
);

-- Table: user_request_refund_history
CREATE TABLE tbl.user_request_refund_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_request_refund_history_id'),
    fkey_user_id bigint  NOT NULL,
    blockchain enum_block_chain  NOT NULL,
    quantity varchar(64)  NOT NULL,
    wallet_address varchar(64)  NOT NULL,
    transaction_hash varchar(80)  NULL,
    created_at bigint  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT user_request_refund_history_pk PRIMARY KEY (pkey_id)
);

-- Table: user_strategy_ledger
CREATE TABLE tbl.user_strategy_ledger (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_user_strategy_ledger_id'),
    fkey_strategy_id bigint  NOT NULL,
    fkey_user_id bigint  NOT NULL,
    balance varchar(64)  NOT NULL,
    updated_at bigint  NOT NULL,
    CONSTRAINT user_strategy_ledger_pk PRIMARY KEY (pkey_id)
);

-- Table: wallet_activity_history
CREATE TABLE tbl.wallet_activity_history (
    pkey_id bigint  NOT NULL DEFAULT nextval('tbl.seq_wallet_activity_history_id'),
    address varchar(64)  NOT NULL,
    transaction_hash varchar(80)  NOT NULL,
    blockchain enum_block_chain NOT NULL,
    dex varchar(20)  NULL,
    contract_address varchar(64)  NOT NULL,
    token_in_address varchar(64)  NULL,
    token_out_address varchar(64)  NULL,
    caller_address varchar(64)  NOT NULL,
    amount_in varchar(64)  NULL,
    amount_out varchar(64)  NULL,
    swap_calls jsonb  NULL,
    paths jsonb  NULL,
    dex_versions jsonb  NULL,
    created_at bigint  NOT NULL,
    CONSTRAINT wallet_activity_history_ak_1 UNIQUE (transaction_hash) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT wallet_activity_history_pk PRIMARY KEY (pkey_id)
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

-- Reference: expert_profile_user_follow_expert (table: user_follow_expert)
ALTER TABLE tbl.user_follow_expert ADD CONSTRAINT expert_profile_user_follow_expert
    FOREIGN KEY (fkey_expert_id)
    REFERENCES tbl.expert_profile (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: fkey_user (table: user_deposit_history)
ALTER TABLE tbl.user_deposit_history ADD CONSTRAINT fkey_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
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

-- Reference: strategy_strategy_initial_token_ratio (table: strategy_initial_token_ratio)
ALTER TABLE tbl.strategy_initial_token_ratio ADD CONSTRAINT strategy_strategy_initial_token_ratio
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_strategy_watching_wallet (table: strategy_watching_wallet)
ALTER TABLE tbl.strategy_watching_wallet ADD CONSTRAINT strategy_strategy_watching_wallet
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: strategy_wallet_user (table: strategy_wallet)
ALTER TABLE tbl.strategy_wallet ADD CONSTRAINT strategy_wallet_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
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

-- Reference: user_back_strategy_history_strategy (table: user_back_strategy_history)
ALTER TABLE tbl.user_back_strategy_history ADD CONSTRAINT user_back_strategy_history_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_back_strategy_history_user (table: user_back_strategy_history)
ALTER TABLE tbl.user_back_strategy_history ADD CONSTRAINT user_back_strategy_history_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_exit_strategy_history_strategy (table: user_exit_strategy_history)
ALTER TABLE tbl.user_exit_strategy_history ADD CONSTRAINT user_exit_strategy_history_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_exit_strategy_history_user (table: user_exit_strategy_history)
ALTER TABLE tbl.user_exit_strategy_history ADD CONSTRAINT user_exit_strategy_history_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
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

-- Reference: user_registered_wallet_user (table: user_registered_wallet)
ALTER TABLE tbl.user_registered_wallet ADD CONSTRAINT user_registered_wallet_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
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

-- Reference: user_strategy_ledger_strategy (table: user_strategy_ledger)
ALTER TABLE tbl.user_strategy_ledger ADD CONSTRAINT user_strategy_ledger_strategy
    FOREIGN KEY (fkey_strategy_id)
    REFERENCES tbl.strategy (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_strategy_ledger_user (table: user_strategy_ledger)
ALTER TABLE tbl.user_strategy_ledger ADD CONSTRAINT user_strategy_ledger_user
    FOREIGN KEY (fkey_user_id)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_user_request_refund_history (table: user_request_refund_history)
ALTER TABLE tbl.user_request_refund_history ADD CONSTRAINT user_user_request_refund_history
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

-- Sequence: seq_expert_profile_id
CREATE SEQUENCE tbl.seq_expert_profile_id
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

-- Sequence: seq_strategy_wallet_id
CREATE SEQUENCE tbl.seq_strategy_wallet_id
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

-- Sequence: seq_user_back_strategy_history_id
CREATE SEQUENCE tbl.seq_user_back_strategy_history_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_deposit_history_id
CREATE SEQUENCE tbl.seq_user_deposit_history_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_exit_strategy_history_id
CREATE SEQUENCE tbl.seq_user_exit_strategy_history_id
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

-- Sequence: seq_user_registered_wallet_id
CREATE SEQUENCE tbl.seq_user_registered_wallet_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_request_refund_history_id
CREATE SEQUENCE tbl.seq_user_request_refund_history_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_user_strategy_ledger_id
CREATE SEQUENCE tbl.seq_user_strategy_ledger_id
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

-- Sequence: seq_wallet_activity_history_id
CREATE SEQUENCE tbl.seq_wallet_activity_history_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- End of file.

