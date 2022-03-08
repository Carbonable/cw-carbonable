import { AbstractContract, Contract, ContractData, ContractRegistry } from '../core/contracts';
import { Keychain } from '../core/keychain';
import { config } from "../config";
import { ExecuteResult } from "@cosmjs/cosmwasm-stargate/build/signingcosmwasmclient";

export class CwCarbonableNft implements AbstractContract {
    readonly name = 'cw-carbonable-nft';
    data?: ContractData;
    keychain?: Keychain;
    contracts?: ContractRegistry;

    setup(data: ContractData, keyChain: Keychain, contracts: ContractRegistry): void {
        this.data = data;
        this.keychain = keyChain;
        this.contracts = contracts;
    }

    async instantiatePayload(): Promise<Record<string, unknown>> {
        if (!this.contracts) {
            throw new Error('need to setup this contract');
        }

        const minter = await this.contracts[Contract.CONTRACT_SELL].address;
        return {
            name: config.nftName,
            symbol: config.symbol,
            minter
        }
    }

    private async _query(wallet: string) {
        if (!this.data || !this.keychain) {
            throw new Error('need to setup this contract');
        }

        const contract = this.data.address;
        const client = await this.keychain.getClient(wallet);
        return {
            contract,
            client,
        };
    }

    async queryAllTokens(wallet: string, start_after: string|undefined = undefined, limit: number|undefined = undefined): Promise<ExecuteResult> {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                all_tokens: { start_after, limit }
            },
        );
    }

    async queryToken(wallet: string, owner: string, start_after: string|undefined = undefined, limit: number|undefined = undefined) {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                tokens: { owner, start_after, limit }
            },
        );
    }

    async queryAllNftInfo(wallet: string, token_id: string, include_expired:boolean|undefined = undefined) {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                all_nft_info: { token_id, include_expired }
            },
        );
    }

    async queryNftInfo(wallet: string, token_id: string) {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                nft_info: { token_id }
            },
        );
    }

    async queryContractInfo(wallet: string) {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                contract_info: { }
            },
        );
    }

    async queryNumTokens(wallet: string) {
        const { contract, client } = await this._query(wallet);
        return client.queryContractSmart( contract, {
                num_tokens: { }
            },
        );
    }


}