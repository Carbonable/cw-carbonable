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
    nftName: 'Las delicias',
    symbol: 'CARBZ-COL-2',
    collectionName: 'Las delicias',
    description: 'Invest in decarbonation through our Green DeFi Launchpad.',

    // sell config
    sellPrice: 12900000,
    sellReservedAmount: 4,
    sellMarketAmount: 356,
    maxBuyAtOnce: 5,
    owner_mnemonic: process.env.OWNER_MNEMONIC,
    admin_mnemonic: process.env.ADMIN_MNEMONIC,
    anonymous_mnemonic: process.env.ANON_MNEMONIC
};

export function updateConfig(otherConfig: ObjAny) {
    config = {...config, ...otherConfig};
}