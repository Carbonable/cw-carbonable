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
    await sellContract.executePreSellMode(Keychain.ADMIN, false);
}

connect().then(() => {
    logger.info('done');
})