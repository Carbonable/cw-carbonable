import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';
import { logger } from './logger';
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { FaucetClient } from '@cosmjs/faucet-client';
import {GasPrice} from "@cosmjs/stargate";
import {config} from "../config";
import fs from "fs";

export interface ObjAny { [k: string]: any; }

export class WalletData {
    secp256!: DirectSecp256k1HdWallet;
    client!: SigningCosmWasmClient;
}

export interface WalletRegistry { [k: string]: WalletData; }

export class Keychain {
    private registry: WalletRegistry;
    private denom: string;
    private rpcServer: string;
    private prefix: string;
    private faucet: string;

    static OWNER = 'owner';
    static ADMIN = 'admin';
    static ANON = 'anonymous';
    static WALLETS = [
        Keychain.OWNER,
        Keychain.ADMIN,
        Keychain.ANON,
    ];

    constructor(denom: string, rpcServer: string, prefix: string, faucet: string) {
        this.denom = denom;
        this.rpcServer = rpcServer;
        this.prefix = prefix;
        this.faucet = faucet;
        this.registry = {};
    }

    async setup() {
        const faucet = new FaucetClient(this.faucet);

        for (const k of Keychain.WALLETS) {
            let secp256: DirectSecp256k1HdWallet | null = null;
            if (config[k + '_mnemonic']) {
                secp256 = await DirectSecp256k1HdWallet.fromMnemonic(config[k + '_mnemonic'], {prefix: this.prefix});
            } else {
                secp256 = await DirectSecp256k1HdWallet.generate(24, {prefix: this.prefix});
                logger.info('new wallet',  k + '_mnemonic', ' : ',  secp256.mnemonic);
                fs.appendFile(config.instantiateConfigPath, '"' + k + '_mnemonic": "'+  secp256.mnemonic +'",\n', (x) => {});
            }

            this.registry[k] = {
                secp256,
                client: await SigningCosmWasmClient.connectWithSigner(this.rpcServer, secp256, {gasPrice: GasPrice.fromString(`${config.gasPrice}${config.denom}`)}),
            }

            const walletAddress = await this.getAddress(k);
            logger.info('using ' + k + ' wallet (' + walletAddress + ')');

            if (!config[k + '_mnemonic'] && config.needFaucet) {
                logger.info('calling faucet for ' + walletAddress);
                await faucet.credit(walletAddress, this.denom);
            }
        }
    }

    async getAddress(wallet: string): Promise<string> {
        const walletData = await this.getWalletByName(wallet);
        return (await walletData.secp256.getAccounts())[0].address;
    }

    async getBalance(wallet: string) {
        return this.getWalletByName(wallet).client.getBalance(await this.getAddress(wallet), this.denom);
    }

    getClient(wallet: string) {
        return this.getWalletByName(wallet).client;
    }

    getWalletByName(wallet: string): WalletData {
        if (!Keychain.WALLETS.includes(wallet)) {
            throw Error('invalid wallet name');
        }

        if (!this.registry[wallet]) {
            throw Error('you need to call setup');
        }

        return this.registry[wallet];
    }
}