-- Created by Vertabelo (http://vertabelo.com)

create schema tbl;;

-- tables

-- Table: authorization_attempt
CREATE TABLE tbl.authorization_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_authorization_attempt_id' ),
    fkey_user bigint  NOT NULL,
    ip_address inet  NOT NULL,
    is_token_ok boolean  NOT NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.authorization_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: bad_request
CREATE TABLE tbl.bad_request (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_bad_request_id' ),
    fkey_user bigint  NULL,
    ip_address inet  NOT NULL,
    method_code integer  NULL,
    error_code integer  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL,
    raw varchar(16384)  NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.bad_request_pk" PRIMARY KEY (pkey_id)
);


-- Table: login_attempt
CREATE TABLE tbl.login_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_login_attempt_id' ),
    fkey_user bigint  NULL,
    address varchar(20)  NOT NULL,
    ip_address inet  NOT NULL,
    device_id varchar(256)  NULL,
    device_os varchar(64)  NULL,
    is_password_ok boolean  NULL,
    moment oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    CONSTRAINT "tbl.login_attempt_pk" PRIMARY KEY (pkey_id)
);

-- Table: organization
CREATE TABLE tbl.organization (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_organization_id' ),
    name varchar  NOT NULL,
    country varchar  NOT NULL,
    tax_id varchar  NOT NULL,
    address varchar  NOT NULL,
    note varchar  NOT NULL,
    approved boolean NOT NULL,
    CONSTRAINT organization_pk PRIMARY KEY (pkey_id)
);
-- Table: organization_membership
CREATE TABLE tbl.organization_membership (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_organization_membership_id' ),
    fkey_user bigint  NOT NULL,
    fkey_organization bigint  NOT NULL,
    role enum_role  NOT NULL,
    created_at oid  NOT NULL,
    accepted boolean  NOT NULL,
    CONSTRAINT organization_membership_pk PRIMARY KEY (pkey_id)
);

-- Table: password_reset_attempt
CREATE TABLE tbl.password_reset_attempt (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_password_reset_attempt_id' ),
    fkey_user bigint  NOT NULL,
    initiated_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    valid_until oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint + 86400,
    code varchar(256)  NOT NULL,
    CONSTRAINT password_reset_attempt_pk PRIMARY KEY (pkey_id)
);

-- Table: recovery_question
CREATE TABLE tbl.recovery_question (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_recovery_question_id' ),
    fkey_user bigint  NOT NULL,
    fkey_question smallint  NOT NULL,
    answer varchar(256)  NOT NULL,
    CONSTRAINT recovery_question_pk PRIMARY KEY (pkey_id)
);

-- Table: recovery_question_data
CREATE TABLE tbl.recovery_question_data (
    pkey_id smallint  NOT NULL,
    content varchar(256)  NOT NULL,
    category enum_recovery_question_category  NOT NULL,
    CONSTRAINT recovery_question_data_pk PRIMARY KEY (pkey_id)
);

-- Table: support_ticket
CREATE TABLE tbl.support_ticket (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_support_ticket_id' ),
    fkey_user bigint  NOT NULL,
    fkey_handler_user bigint  NULL,
    content varchar  NOT NULL,
    response varchar  NOT NULL,
    created_at oid  NOT NULL,
    updated_at oid  NOT NULL,
    CONSTRAINT support_ticket_pk PRIMARY KEY (pkey_id)
);


-- Table: user
CREATE TABLE tbl."user" (
    pkey_id bigint  NOT NULL DEFAULT nextval( 'tbl.seq_user_id' ),
    role enum_role  NOT NULL DEFAULT 'user',
    address varchar(20)  NOT NULL,
    age smallint  NOT NULL,
    preferred_language varchar(5)  NOT NULL,
    family_name varchar(128)  NULL,
    given_name varchar(128)  NULL,
    agreed_tos boolean  NOT NULL,
    agreed_privacy boolean  NOT NULL,
    created_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    updated_at oid  NOT NULL DEFAULT EXTRACT(EPOCH FROM (NOW()))::bigint,
    email varchar(320)  NULL,
    phone_number varchar(15)  NULL,
    last_ip inet  NOT NULL,
    last_login oid  NULL,
    last_password_reset oid  NULL,
    logins_count integer  NOT NULL DEFAULT 0,
    user_device_id varchar  NULL,
    admin_device_id varchar  NULL,
    password_reset_token uuid  NULL,
    reset_token_valid uuid  NULL,
    user_token uuid  NULL,
    admin_token uuid  NULL,
    is_blocked boolean  NOT NULL DEFAULT false,
    CONSTRAINT uidx_user_username UNIQUE (address) NOT DEFERRABLE  INITIALLY IMMEDIATE,
    CONSTRAINT user_pk PRIMARY KEY (pkey_id)
);


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



-- Reference: login_attempt_user (table: login_attempt)
ALTER TABLE tbl.login_attempt ADD CONSTRAINT login_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;


-- Reference: organization_organization_membership (table: organization_membership)
ALTER TABLE tbl.organization_membership ADD CONSTRAINT organization_organization_membership
    FOREIGN KEY (fkey_organization)
    REFERENCES tbl.organization (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: password_reset_attempt_user (table: password_reset_attempt)
ALTER TABLE tbl.password_reset_attempt ADD CONSTRAINT password_reset_attempt_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: recovery_question_recovery_question_data (table: recovery_question)
ALTER TABLE tbl.recovery_question ADD CONSTRAINT recovery_question_recovery_question_data
    FOREIGN KEY (fkey_question)
    REFERENCES tbl.recovery_question_data (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: recovery_question_user (table: recovery_question)
ALTER TABLE tbl.recovery_question ADD CONSTRAINT recovery_question_user
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: support_ticket_user (table: support_ticket)
ALTER TABLE tbl.support_ticket ADD CONSTRAINT support_ticket_user
    FOREIGN KEY (fkey_handler_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;


-- Reference: user_organization_membership (table: organization_membership)
ALTER TABLE tbl.organization_membership ADD CONSTRAINT user_organization_membership
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
;

-- Reference: user_support_ticket (table: support_ticket)
ALTER TABLE tbl.support_ticket ADD CONSTRAINT user_support_ticket
    FOREIGN KEY (fkey_user)
    REFERENCES tbl."user" (pkey_id)  
    NOT DEFERRABLE 
    INITIALLY IMMEDIATE
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

-- Sequence: seq_listing_id
CREATE SEQUENCE tbl.seq_listing_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_login_attempt_id
CREATE SEQUENCE tbl.seq_login_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_organization_id
CREATE SEQUENCE tbl.seq_organization_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_organization_membership_id
CREATE SEQUENCE tbl.seq_organization_membership_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
;

-- Sequence: seq_password_reset_attempt_id
CREATE SEQUENCE tbl.seq_password_reset_attempt_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_recovery_question_id
CREATE SEQUENCE tbl.seq_recovery_question_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- Sequence: seq_support_ticket_id
CREATE SEQUENCE tbl.seq_support_ticket_id
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


-- Sequence: seq_ver_id
CREATE SEQUENCE tbl.seq_ver_id
      NO MINVALUE
      NO MAXVALUE
      NO CYCLE
      AS bigint
;

-- End of file.

