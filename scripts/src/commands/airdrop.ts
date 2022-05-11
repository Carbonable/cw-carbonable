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
    logger.info(`Airdrop to ${address}`);
    await sellContract.executeAirdrop(Keychain.ADMIN, [address]);
}

connect().then(() => {
    logger.info('done');
})