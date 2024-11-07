# Repository from the Fuel Liquidity Pool tutorial

## Sway contract

```
cd contracts/pool
```

You can build it by running:

```
forc build --release
```

## Interaction tools

A Rust script to conviniently test the smart contract on testnet

```
cd contracts/interaction-tools
```

Add your MNEMONIC phrace into the `.env` file.

Run it with:

```
cargo run
```

## Backend with Envio indexer

```
cd backend
```

Run locally with:

```
pnpm dev
```

For a thorough understanding and to dive deeper into each feature, refer to the original [documentation website](https://docs.envio.dev/).

## Hosted Service

https://envio.dev/app
