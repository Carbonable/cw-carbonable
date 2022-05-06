#!/usr/bin/env node

import {Keychain} from '../core/keychain';
import {Contract} from '../core/contracts';
import {config} from '../config';
import {logger} from '../core/logger';
import {load} from "../core/core";
import * as readline from 'node:readline';
import {question} from 'readline-sync';
import { stdin, stdout } from 'process';
import {readFileSync} from 'fs';


const rl = readline.createInterface({
    input: stdin,
    output: stdout
});


async function connect() {
    const { keychain, contracts, nftContract, sellContract, alreadyUploaded } = await load();
    if (!alreadyUploaded) {
        return ;
    }
    const attributesFilePath = question('Attributes file path: ');
    logger.info('Reading attributes files: ', attributesFilePath);
    const rawAttributes = readFileSync(attributesFilePath);
    const attributes = JSON.parse(rawAttributes.toString());
    const updateMetadataResponse = await sellContract.executeUpdateMetadata(Keychain.ADMIN, {
        name: config.collectionName,
        description: config.description,
        image: "https://firebasestorage.googleapis.com/v0/b/carbonable-token.appspot.com/o/projects%2F2%2Fcard.jpeg?alt=media&token=eb55aa59-19ba-492e-b132-823357e558b7",
        external_url: "ipfs://carbonable/",
        attributes: attributes,
    });
    logger.debug('Update metadata', updateMetadataResponse);
}

connect().then(() => {
    logger.info('done');
})