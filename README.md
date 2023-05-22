# MC2 Fi

This is a async websocket server template, with nice codegen included. Every tool required from Data Access Layer to
backend endpoints are generated without user intervention.

## How to build

```shell
cargo build
cargo build --release
```

You can find the binary in `target/debug/` or `target/release/` folder

## How to run

```shell
cargo run --bin auth
```

```shell
cargo run --bin user
```
## How to test
```shell
cargo test -- --test-threads=1
```
## Authentication process

All critical authentication information is in the `Sec-Websocket-Protocol` header.
Variables in the `Sec-Websocket-Protocol` header are prefixed with an id number and separated by commas. The first
variable (prefixed by zero) is always the auth service method.

#### #1: Signup

Example headers:
`Connection: upgrade`
`Upgrade: websocket`
`Sec-WebSocket-Version: 13`
`Accept: sec-websocket-accept`
`Sec-WebSocket-Protocol: 0signup,1address,2signature_message_hex_encoded,3signature_hex_encoded,4user_one@gmail,5+00123456,6true,7true`
Here in the `Sec-Websocket-Protocol` header we have the signup method as the first variable, followed by the user's
desired address, signature message, signature, email, phone, and the last two booleans refer to if the user agreed to the Terms of Service
and Terms of Privacy, respectively.

The signup method validates that the username is unique, create the user with the default role 'user', and on success
returns the username and a public id generated for the user.

#### #2: Login

Example headers:
`Connection: upgrade`
`Upgrade: websocket`
`Sec-WebSocket-Version: 13`
`Accept: sec-websocket-accept`
`Sec-WebSocket-Protocol: 0login,1address,2placeholder_password,3User,43849823798,5android`
Here in the `Sec-Websocket-Protocol` header we have the login method as the first variable, followed by the user's
registered address, signature message, signature, service (the service the user is logging into), device id, and device OS.

The login method validates that the password hash matches the registered password for the user, and that the requested
service matches the user's role, and on success returns the address, the user's id, and 2 tokens generated for
the user, 1 'user' token and 1 'admin' token. These tokens are to match the requested service when calling Authorize (
see below) but will only be valid if the user has the necessary role to use the service.

#### #3 + #4: Route with Authorize as middleware

Example headers:
`Connection: upgrade`
`Upgrade: websocket`
`Sec-WebSocket-Version: 13`
`Accept: sec-websocket-accept`
`Sec-WebSocket-Protocol: 0authorize,1address,2371a695c-b4c1-47b0-b779-638fdc47b4ac,32,43849823798,5android`
Here in the `Sec-Websocket-Protocol` header we have the authorize method as the first variable, followed by the user's
registered address, user or admin token (will be validated according to the chosen service), service (the service
the user is being authorized into, must match the token), device id, and device OS.

The authorize middleware is used with every other route, it validates that the token, a valid token must have 3
conditions:

1. the token must be the one generated for this user on Login.
2. the token must match the service the user is requesting access to.
3. the user must have the role requirements to use the token.

## Structure explained

`src/codegen` core codegen logic

`src/gen` codegen target

`src/lib` common code

`src/service` implementation of services

`src/service/{srv}/main.rs` main entry of service

`src/service/{srv}/endpoints.rs` declaration of endpoints

`src/service/{srv}/pg_func.rs` declaration of postgres procedural endpoints (DALs)

`src/service/{srv}/method.rs` implementation of endpoints

`tests` integration tests

`benches` benchmarks

`docs` documentation

`db` database related files

`etc` configuration files

`scripts` helper scripts

## Feature List for Product

The platform will be developed for testnet and will include the following features for every persona:

- User
    - [x] Login using WalletConnect (backend)

    - [ ] Login using WalletConnect (frontend)

    - Update Profile and Personal Details

        - [ ] Change wallet

        - [ ] Change social handles

        - [ ] Notification Settings (for email)

    - Explore Strategies

        - Search strategy using Name, DeFi Protocols or Token Names, or Trader Name

        - Sort & filter strategy

        - View strategy details
            - Name
            - Creator name
            - Historic returns (with graph)
            - Inception date
            - Total size of vault
            - Token allocation details
            - Description about policies that strategy will adhere to
            - Deposit charges & optimum holding period for returns

        - Strategy Investment

        - [x] Invest into single strategy by using stable coin (USDT, USDC & BUSD) or crypto Token (has to be native)
            - Receive Vault tokens for their investment
            - See returns generated on a strategy
            - Withdraw tokens from a strategy in tokens that we deposited in (withdrawal charges to be levied here)
            - Burn the vault tokens received from the user

        - Combine Strategy

        - Combine one or more strategy on a percent allocation
            - View the combined returns on the strategy
            - Withdraw from the combined strategy

- Trader

- [ ] Connect my trading wallet to the platform to be monitored

- [ ] Generate a database of all token allocation changes on the trader wallet

- [ ] Validate and evaluate the possibility of transactions made on trader wallet to be successful using a validation
  engine checking for

-
    - Consistency of returns
        - Policy Adherence
        - KYC Validation Level

    - Ability to update and inform users about the strategy changes

- Admin

    - AMM creation (same as Trader AMM)

        - Signalling service to monitor trades on trader wallet

        - Validation engine (to validate trade against strategy)

        - Whitelisted tokens & DEX modification

        - White-label widget for DEX integration

    - integrate using simple API calls
    - allow users to filter on types of strategy that are available (using Tokens, DeFi protocols)
    - display the details of the strategy selected by the user


## First week will be

- [x] Getting auth up (login with Metamask)
- [ ] As much data models as we can get (I'll get vertabelo setup)
- [ ] The user/backer endpoint docs up and more accurate and better planned out

