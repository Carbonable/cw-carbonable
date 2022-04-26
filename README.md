# carbonable

### SmartContracts

Two smart contracts are provided :
- cw_carbonable_nft
- cw_carbonable_sell

The first is the NFT contract, the second one a sell contract, allowing you to also airdrop NFT.

### How to build the smart contract

Requirements :
- docker
- rust

Simply do :

```shell
bash $ ./build.sh
```

This will build your smart contract in the artifacts directory. This directory should contain 3 contracts :

- cw_carbonable_lib.wasm (no usage, intermediate library)
- cw_carbonable_nft.wasm (NFT contract)
- cw_carbonable_sell.wasm (Sell contract)

### How to build deploy toolkit

Requirements :
- recent vesrion of NodeJs
- yarn

```shell
bash $ yarn install
```

### Deploy on testnet

A deploy toolkit is present in the scripts directory. This deploy toolkit take place in to phases :

1 - deploy script

    * create wallets (OWNER/ADMIN/ANON)
    * upload contracts (NFT + SELL)
    * generate a contract.config needed to run script on the deploy

2 - scripting smart contract scenario

    * loading config.ts + contract.config
    + playing with smart contract

How to deploy :
```shell
bash $ yarn run deploy
```

An example of a script is given in scrips/src/commands/presale.ts, you can use it doing :

```shell
bash $ yarn run presale
```

### Changelog v2
 - No maintenance mode anymore.
 - you can airdrop even when the sale/presale are closed.
 - add SaleMode mechanism to enable the sell
 - add PreSaleMode mechanism to enable the whitelisted sale