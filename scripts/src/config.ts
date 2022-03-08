import { env } from 'process';
import { ObjAny } from './core/keychain';

export let config: ObjAny = {
    // chain config
    denom: env.DENOM || 'ujuno',
    rpcServer: env.RPC_SERVER || 'https://rpc-juno.itastakers.com',
    prefix: env.CHAIN_PREFIX || 'juno',
    needFaucet: false,
    faucet: env.FAUCET_SERVER || 'https://faucet.cliffnet.cosmwasm.com',

    // contracts config
    instantiateConfigPath: './contract.config',
    contractPath: env.CONTRACT_PATH || './artifacts',
    gasPrice: env.GAS_PRICE || '0.0025',

    // nft contract config
    nftName: 'carbonable',
    symbol: 'CARB',
    collectionName: 'carbonable',
    description: 'Invest in decarbonation through our Green DeFi Launchpad.',

    // sell config
    sellPrice: 100,
    sellReservedAmount: 1,
    sellMarketAmount: 9999,

    owner_mnemonic: process.env.OWNER_MNEMONIC,
    admin_mnemonic: process.env.ADMIN_MNEMONIC,
    anonymous_mnemonic: process.env.ANON_MNEMONIC
};

export let config2: ObjAny = {
    // chain config
    denom: env.DENOM || 'upebble',
    rpcServer: env.RPC_SERVER || 'https://rpc.cliffnet.cosmwasm.com',
    prefix: env.CHAIN_PREFIX || 'wasm',
    needFaucet: true,
    faucet: env.FAUCET_SERVER || 'https://faucet.cliffnet.cosmwasm.com',

    // contracts config
    instantiateConfigPath: './contract.config',
    contractPath: env.CONTRACT_PATH || './artifacts',
    gasPrice: env.GAS_PRICE || '0.025',

    // nft contract config
    nftName: 'carbonable',
    symbol: 'CARB',
    collectionName: 'carbonable',
    description: 'utility NFT',

    // sell config
    sellPrice: 100,
    sellReservedAmount: 100,
    sellMarketAmount: 9900,
};

export function updateConfig(otherConfig: ObjAny) {
    config = {...config, ...otherConfig};
}