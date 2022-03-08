import fs from "fs";
import {config, updateConfig} from "../config";
import {Keychain} from "./keychain";
import {Contract, ContractImpl} from "./contracts";
import {CwCarbonableNft} from "../contracts/cw_carbonable_nft";
import {CwCarbonableSell} from "../contracts/cw_carbonable_sell";
import {logger} from "./logger";

export async function upload_or_load() {
    fs.writeFile(config.instantiateConfigPath, '{\n', (x) => {});

    const keychain = new Keychain(config.denom, config.rpcServer, config.prefix, config.faucet);
    await keychain.setup();

    const contracts = new Contract(keychain);
    const contractImpl: ContractImpl = {};
    const nftContract = new CwCarbonableNft();
    const sellContract = new CwCarbonableSell();
    contractImpl[Contract.CONTRACT_NFT] = nftContract;
    contractImpl[Contract.CONTRACT_SELL] = sellContract;

    await contracts.setup(contractImpl);

    fs.appendFile(config.instantiateConfigPath, '}\n', (x) => {});

    return {
        keychain,
        contracts,
        nftContract,
        sellContract,
        alreadyUploaded: (config.cw_carbonable_sell_contract && config.cw_carbonable_nft_contract)
    };
}

export async function load() {
    if (fs.existsSync(config.instantiateConfigPath)) {
        let rawdata = fs.readFileSync(config.instantiateConfigPath);
        let contractConfig = JSON.parse(rawdata.toString());
        updateConfig(contractConfig);

        logger.debug('using config', config)
    } else {
        logger.error('you must run deploy and update the config.ts file with smart contract info before running this script...')
        const fakeKeychain = new Keychain(config.denom, config.rpcServer, config.prefix, config.faucet);

        return {
            keychain: fakeKeychain,
            contract: new Contract(fakeKeychain),
            nftContract: new CwCarbonableNft(),
            sellContract: new CwCarbonableSell(),
            alreadyUploaded: false,
        };
    }

    const keychain = new Keychain(config.denom, config.rpcServer, config.prefix, config.faucet);
    await keychain.setup();

    const contracts = new Contract(keychain);
    const contractImpl: ContractImpl = {};
    const nftContract = new CwCarbonableNft();
    const sellContract = new CwCarbonableSell();
    contractImpl[Contract.CONTRACT_NFT] = nftContract;
    contractImpl[Contract.CONTRACT_SELL] = sellContract;

    await contracts.setup(contractImpl);

    return {
        keychain,
        contracts,
        nftContract,
        sellContract,
        alreadyUploaded: true,
    };
}