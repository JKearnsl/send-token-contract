# Send Token Contract

A simple smart contract for the cosmos network that allows you to transfer funds of any denom from one account to another thanks to a smart contract

## Build

```bash
cargo build --release --lib --target wasm32-unknown-unknown
```

## Test

```bash
cargo test
```

## Deploy

To deploy a smart contract to the network, run:

```bash
./chaind tx wasm store path/to/release/send-token-contract.wasm --from {ACCOUNT} --keyring-backend test --chain-id {CHAIN_ID} --gas 15000000
```

Next, using the received contract number, perform initialization to obtain the smart contract address:

```bash
./chaind tx wasm instantiate {CONTRACT_ID} '' --from {ACCOUNT} --label "SuperContract" --no-admin --gas 200000 --keyring-backend test --chain-id {CHAIN_ID}
```

You will get something like `wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d`

## Execute

To transfer tokens from one account to another, run this command:

```bash
./chaind tx wasm execute {CONTRACT_ADDRESS} '{"send_tokens": {"to": "wasm1fpcjrgju3ht0vgcgudsfr5a8ys3rrf6kxstgw9"}}' --from {ACCOUNT} --gas 150000 --keyring-backend test --chain-id {CHAIN_ID} --amount 10token
```

You will transfer from {ACCOUNT} **10token** to `wasm1fpcjrgju3ht0vgcgudsfr5a8ys3rrf6kxstgw9` via smart contract
