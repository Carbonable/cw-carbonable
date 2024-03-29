#!/usr/bin/env node

import { Keychain } from '../core/keychain';
import { config } from '../config';
import { logger } from '../core/logger';
import { load } from "../core/core";

async function connect() {
    const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
    if (!alreadyUploaded) {
        return ;
    }

    const anonAddr = await keychain.getAddress(Keychain.ANON);

    await sellContract.executeUpdatePrice(Keychain.OWNER, {
        denom: config.denom,
        amount: `${config.sellPrice}`,
    });

    await sellContract.executeSellMode(Keychain.OWNER, true);

    await sellContract.executeAirdrop(Keychain.ADMIN, [anonAddr]);

    await sellContract.executeBuy(Keychain.OWNER, {
        denom: config.denom,
        amount: `${config.sellPrice}`,
    });

    await sellContract.executeBuy(Keychain.OWNER, {
        denom: config.denom,
        amount: `${config.sellPrice}`,
    });

    logger.info('anon balance : ', await keychain.getBalance(Keychain.ANON));

    const withdrawAmount = 2 * config.sellPrice;
    await sellContract.executeWithdraw(Keychain.ADMIN, anonAddr, {
        denom: config.denom,
        amount: `${withdrawAmount}`,
    });

    logger.info('anon balance : ', await keychain.getBalance(Keychain.ANON));

    logger.info('all tokens : ', await nftContract.queryAllTokens(Keychain.ANON));
    logger.info('tokens : ', await nftContract.queryToken(Keychain.ANON, anonAddr));
    logger.info('num tokens : ', await nftContract.queryNumTokens(Keychain.ANON));
    logger.info('contract info : ', await nftContract.queryContractInfo(Keychain.ANON));
    logger.info('tokens : ', await nftContract.queryAllNftInfo(Keychain.ANON, "2"));

    logger.info(await sellContract.queryState(Keychain.ANON));
}

connect().then(() => {
    logger.info('done');
})