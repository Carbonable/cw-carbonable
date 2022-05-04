#!/usr/bin/env node

import * as readline from 'node:readline';
import {question} from 'readline-sync';
import { stdin, stdout } from 'process';

import { Keychain } from '../core/keychain';
import { config } from '../config';
import { logger } from '../core/logger';
import { load } from "../core/core";

const rl = readline.createInterface({
    input: stdin,
    output: stdout
});


async function connect() {
    const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
    if (!alreadyUploaded) {
        return ;
    }
    const tokenId = question('Token ID: ');
    const info = await nftContract.queryNftInfo(Keychain.ANON, tokenId);
    console.log(JSON.stringify(info, null, 2));
}

connect().then(() => {
    logger.info('done');
})