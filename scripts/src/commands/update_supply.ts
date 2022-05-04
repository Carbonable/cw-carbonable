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

    await sellContract.executeUpdateSupply(Keychain.ADMIN, config.sellReservedAmount, config.sellMarketAmount);


}

connect().then(() => {
    logger.info('done');
})