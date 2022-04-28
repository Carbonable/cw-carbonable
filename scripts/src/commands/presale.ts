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

    await sellContract.executeAirdrop(Keychain.ADMIN, []);

    await sellContract.executeSellMode(Keychain.ADMIN, false);
    await sellContract.executePreSellMode(Keychain.ADMIN, true);
    await sellContract.executeAddToWhitelist(Keychain.ADMIN, [{
        address: await keychain.getAddress(Keychain.ANON),
        nb_slots: 1
    }]);
    await sellContract.executMultieBuy(Keychain.ANON, { denom: config.denom, amount: (config.sellPrice * 5).toString()}, 5)
    logger.info(await nftContract.queryNumTokens(Keychain.ANON));
    logger.info(await sellContract.queryState(Keychain.ANON));
}

connect().then(() => {
    logger.info('done');
})