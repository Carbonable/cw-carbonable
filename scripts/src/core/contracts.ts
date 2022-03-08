import { Keychain, ObjAny } from './keychain';
import fs, { readFileSync } from 'fs';
import { calculateFee, GasPrice } from '@cosmjs/stargate';
import { logger } from './logger';
import { config } from '../config';

export interface AbstractContract {
    readonly name: string;

    setup(data: ContractData, keyChain: Keychain, contracts: ContractRegistry): void;

    instantiatePayload(): Promise<Record<string, unknown>>;
}

export interface ContractImpl {
    [k: string]: AbstractContract;
}

export interface ContractData {
    codeId: number;
    deployHash: string;
    instantiateHash: string;
    address: string;
    impl: AbstractContract,
}

export interface ContractRegistry {
    [k: string]: ContractData;
}

export function helper_print_contract_data(data: ContractData): ObjAny {
    return {
        codeId: data.codeId,
        deployHash: data.deployHash,
        instantiateHash: data.instantiateHash,
        address: data.address,
    };
}

export class Contract {
    static CONTRACT_NFT = 'cw_carbonable_nft';
    static CONTRACT_SELL = 'cw_carbonable_sell';

    // Order matters here
    // CONTRACT_SELL must come after CONTRACT_NFT
    static CONTRACTS = [
        Contract.CONTRACT_SELL,
        Contract.CONTRACT_NFT,
    ];

    keyChain: Keychain;
    registry: ContractRegistry;

    constructor(keychain: Keychain) {
        this.keyChain = keychain;
        this.registry = {}
    }

    async setup(impl: ContractImpl) {
        let first = true;
        for (const contract of Contract.CONTRACTS) {
            logger.debug('setup ' + contract)
            const filename = config.contractPath + '/' + contract + '.wasm';

            if (config[contract + '_contract']) {
                this.registry[contract] = {
                    codeId: config[contract + '_contract'].codeId,
                    deployHash: config[contract + '_contract'].deployHash,
                    address: config[contract + '_contract'].address,
                    instantiateHash: config[contract + '_contract'].instantiateHash,
                    impl: impl[contract],
                }
                await this.registry[contract].impl.setup(this.registry[contract], this.keyChain, this.registry);
            } else {

                logger.debug('reading file ' + filename)
                const wasm = readFileSync(filename);

                const walletClient = await this.keyChain.getClient(Keychain.OWNER);
                const walletAddress = await this.keyChain.getAddress(Keychain.OWNER);
                const uploadReceipt = await walletClient.upload(walletAddress, wasm, 'auto', 'carbonable');
                this.registry[contract] = {
                    codeId: uploadReceipt.codeId,
                    deployHash: uploadReceipt.transactionHash,
                    address: '',
                    instantiateHash: '',
                    impl: impl[contract],
                }
                logger.debug('contract uploaded ', helper_print_contract_data(this.registry[contract]));

                // setup contract
                await this.registry[contract].impl.setup(this.registry[contract], this.keyChain, this.registry);
                // fetch instantiate payload
                const payload = await this.registry[contract].impl.instantiatePayload();

                logger.debug('instantiate with payload', payload)
                const result = await walletClient.instantiate(
                    walletAddress,
                    this.registry[contract].codeId,
                    payload,
                    'instantiate',
                    'auto',
                    {memo: `instantiate ${contract}`});

                this.registry[contract].address = result.contractAddress;
                this.registry[contract].instantiateHash = result.transactionHash;

                if (first) {
                    fs.appendFile(config.instantiateConfigPath, '"' + contract + '_contract": '+  JSON.stringify(helper_print_contract_data(this.registry[contract])) +'\n', (x) => {});
                    first = false;
                } else {
                    fs.appendFile(config.instantiateConfigPath, ',"' + contract + '_contract": '+  JSON.stringify(helper_print_contract_data(this.registry[contract])) +'\n', (x) => {});
                }

                const notificationMsg =
                    contract + ' deployed\n\tcodeId => ' + this.registry[contract].codeId +
                    '\n\tinstance => <https://block-explorer.cliffnet.cosmwasm.com/account/' + this.registry[contract].address + '|' + this.registry[contract].address + '>' +
                    '\n\ttxInstantiate => <https://block-explorer.cliffnet.cosmwasm.com/transactions/' + this.registry[contract].instantiateHash + '|' + this.registry[contract].instantiateHash + '>' +
                    '\n\ttxDeploy => <https://block-explorer.cliffnet.cosmwasm.com/transactions/' + this.registry[contract].deployHash + '|' + this.registry[contract].deployHash + '>';

                logger.info(notificationMsg);
            }

            logger.debug('contract information', helper_print_contract_data(this.registry[contract]));
        }
    }

    getAddress(contract: string): string {
        const contractData = this.getContractByName(contract);
        return contractData.address;
    }

    getContractByName(contract: string): ContractData {
        if (!Contract.CONTRACTS.includes(contract)) {
            throw Error('invalid contract name');
        }

        if (!this.registry[contract]) {
            throw Error('you need to call setup');
        }

        return this.registry[contract];
    }
}
