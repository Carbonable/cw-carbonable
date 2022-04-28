#!/usr/bin/env node
import * as readline from 'node:readline';
import { stdin, stdout } from 'process';
import {question} from 'readline-sync';
import { Keychain } from '../core/keychain';
import { logger } from '../core/logger';
import {load} from "../core/core";
import {config} from "../config";

const rl = readline.createInterface({
    input: stdin,
    output: stdout
});


async function connect() {
    const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
    if (!alreadyUploaded) {
        return ;
    }
    const address = question('Address: ');
    const slots = question('Slots: ');
    logger.info(`Whitelisting ${address} for ${slots}`);
    await sellContract.executeAddToWhitelist(Keychain.ADMIN, [{
        address: address,
        nb_slots: +slots
    }]);
}

connect().then(() => {
    logger.info('done');
})