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
    const ownerAddr = await keychain.getAddress(Keychain.OWNER);

    await sellContract.executeWithdraw(Keychain.ADMIN, ownerAddr, {
        denom: config.denom,
        amount: '1568000000',
    });

}

connect().then(() => {
    logger.info('done');
})