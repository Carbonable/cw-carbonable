import { AbstractContract,ContractData, ContractRegistry } from '../core/contracts';
import { Keychain } from '../core/keychain';

import { ExecuteResult } from "@cosmjs/cosmwasm-stargate/build/signingcosmwasmclient";
import { Coin } from "@cosmjs/amino";
import { config } from "../config";

export interface State {
    total_market_supply: number,
    total_reserved_supply: number,
    total_market_minted: number,
    total_reserved_minted: number,
    last_token_id: number,
    sell_price: Coin,
    metadata: Metadata,
}

export interface Trait {
    display_type?:string,
    trait_type: string,
    value: string,
}

export interface Metadata {
    image?: string,
    image_data?: string,
    external_url?: string,
    description?: string,
    name?: string,
    attributes?: Trait [],
    background_color?: string,
    animation_url?: string,
    youtube_url?: string,
}

export interface WhiteListEntry {
    address: string,
    nb_slots: number,
}

export class CwCarbonableSell implements AbstractContract {
    readonly name = 'cw-carbonable-sell';
    data?: ContractData;
    keychain?: Keychain;
    contracts?: ContractRegistry;

    setup(data: ContractData, keyChain: Keychain, contracts: ContractRegistry): void {
        this.data = data;
        this.keychain = keyChain;
        this.contracts = contracts;
    }

    async instantiatePayload(): Promise<Record<string, unknown>> {
        return {
            sell_mode: false,
            pre_sell_mode: false,
            max_buy_at_once: config.maxBuyAtOnce,
        }
    }


    async queryState(wallet: string): Promise<State> {
        if (!this.data || !this.keychain) {
            throw new Error('need to setup this contract');
        }

        const contract = this.data.address;
        const client = await this.keychain.getClient(wallet);
        return await client.queryContractSmart(contract, {
            dump_state: {}
        });
    }

    private async _execute(wallet: string) {
        if (!this.data || !this.keychain) {
            throw new Error('need to setup this contract');
        }

        const contract = this.data.address;
        const client = await this.keychain.getClient(wallet);
        const sender = await this.keychain.getAddress(wallet);
        return {
            contract,
            client,
            sender
        };
    }

    async executeBuy(wallet: string, coins: Coin): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            buy: { }
        }, 'auto', 'Buy', [coins]);
    }

    async executMultieBuy(wallet: string, coins: Coin, quantity: number): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            multi_buy: { quantity }
        }, 'auto', 'Buy', [coins]);
    }

    async executeAirdrop(wallet: string, receivers: string[]): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            airdrop: { receivers }
        }, 'auto');
    }

    async executeWithdraw(wallet: string, dest: string, coin: Coin): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            withdraw: { wallet: dest, coin: [coin] }
        }, 'auto');
    }

    async executePreSellMode(wallet: string, enable: boolean): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            pre_sell_mode: { enable }
        }, 'auto');
    }

    async executeSellMode(wallet: string, enable: boolean): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            sell_mode: { enable }
        }, 'auto');
    }

    async executeAddToWhitelist(wallet: string, entries: WhiteListEntry[]): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            add_to_whitelist: { entries }
        }, 'auto');
    }

    async executeUpdatePrice(wallet: string, price: Coin): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            update_price: { price }
        }, 'auto');
    }

    async executeUpdateSupply(wallet: string, reservedSupply: number, marketSupply: number): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            update_supply: { reserved_supply: reservedSupply, market_supply: marketSupply }
        }, 'auto');
    }

    async executeRemoveAdmin(wallet: string, address: string): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            remove_admin: { address }
        }, 'auto');
    }

    async executeAddAdmin(wallet: string, address: string): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            add_admin: { address }
        }, 'auto');
    }

    async executeUpdateNFTContract(wallet: string, address: string): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            update_nft_contract: { address }
        }, 'auto');
    }

    async executeUpdateMetadata(wallet: string, metadata: Metadata): Promise<ExecuteResult> {
        const { contract, client, sender } = await this._execute(wallet);
        return client.execute(sender,  contract, {
            update_metadata: { metadata }
        }, 'auto');
    }
}