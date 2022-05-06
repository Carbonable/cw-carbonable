#!/usr/bin/env node

import {Keychain} from '../core/keychain';
import {Contract} from '../core/contracts';
import {config} from '../config';
import {logger} from '../core/logger';
import {upload_or_load} from "../core/core";
import * as readline from 'node:readline';
import {question} from 'readline-sync';
import { stdin, stdout } from 'process';
import {readFileSync} from 'fs';


const rl = readline.createInterface({
    input: stdin,
    output: stdout
});


async function connect() {
    const {keychain, contracts, nftContract, sellContract, alreadyUploaded} = await upload_or_load();

    if (!alreadyUploaded) {
        const addAdminResponse = await sellContract.executeAddAdmin(Keychain.OWNER, await keychain.getAddress(Keychain.ADMIN));
        logger.debug('Add admin', addAdminResponse);

        const updatePriceResponse = await sellContract.executeUpdatePrice(Keychain.ADMIN, {
            amount: String(config.sellPrice),
            denom: config.denom
        });
        logger.debug('Update price', updatePriceResponse);

        const updateSupplyResponse = await sellContract.executeUpdateSupply(Keychain.ADMIN, config.sellReservedAmount, config.sellMarketAmount);
        logger.debug('Update supply', updateSupplyResponse);

        const updateNftContractResponse = await sellContract.executeUpdateNFTContract(Keychain.ADMIN, contracts.getAddress(Contract.CONTRACT_NFT));
        logger.debug('Update contract', updateNftContractResponse);

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

    const state = await sellContract.queryState(Keychain.ANON);
    logger.debug('state', state);
}

connect().then(() => {
    logger.info('done');
})