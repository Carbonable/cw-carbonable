#!/usr/bin/env node
import * as readline from 'node:readline';
import { stdin, stdout } from 'process';
import {question} from 'readline-sync';
import { Keychain } from '../core/keychain';
import { logger } from '../core/logger';
import {load} from "../core/core";
import {config} from "../config";
import {readFileSync} from 'fs';

const rl = readline.createInterface({
    input: stdin,
    output: stdout
});


async function connect() {
    try{
        const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
        if (!alreadyUploaded) {
            return ;
        }
        const whitelistFilePath = question('Whitelist file path: ');
        logger.info('Reading whitelist file: ', whitelistFilePath);
        const rawWhitelist = readFileSync(whitelistFilePath);
        const whitelist = JSON.parse(rawWhitelist.toString());
        await sellContract.executeAddToWhitelist(Keychain.ADMIN, whitelist);
    }catch(e){
        logger.error(e);
    }
}

connect().then(() => {
    logger.info('done');
})