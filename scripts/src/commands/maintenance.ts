#!/usr/bin/env node

import { Keychain } from '../core/keychain';
import { logger } from '../core/logger';
import {load} from "../core/core";
import {config} from "../config";

async function connect() {
    const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
    if (!alreadyUploaded) {
        return ;
    }

    await sellContract.executeMaintenanceMode(Keychain.ADMIN, true);

    try {
        await sellContract.executeAirdrop(Keychain.ADMIN, []);
        logger.error('error should have thrown an error');
    } catch (err) {
        logger.error('catch', err);
    }

    await sellContract.executeMaintenanceMode(Keychain.ADMIN, false);
    await sellContract.executeAirdrop(Keychain.ADMIN, []);


    await sellContract.executMultieBuy(Keychain.ANON, { denom: config.denom, amount: (config.sellPrice * 5).toString()}, 5)
    logger.info(await nftContract.queryNumTokens(Keychain.ANON));
    logger.info(await sellContract.queryState(Keychain.ANON));
}

connect().then(() => {
    logger.info('done');
})