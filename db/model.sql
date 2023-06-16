CREATE TYPE enum_role AS ENUM ('guest', 'user', 'expert', 'admin', 'developer');
CREATE TYPE enum_block_chain AS ENUM ('EthereumMainnet', 'EthereumGoerli', 'BscMainnet', 'BscTestnet', 'LocalNet', 'EthereumSepolia');
CREATE TYPE enum_blockchain_coin AS ENUM ('USDC', 'USDT', 'BUSD', 'WETH', 'WBNB');
CREATE TYPE enum_dex AS ENUM ('UniSwap', 'PancakeSwap', 'SushiSwap');
CREATE TYPE enum_dex_version AS ENUM ('V1', 'V2', 'V3');
CREATE TYPE enum_service AS ENUM ('auth', 'user', 'admin', 'watcher');
