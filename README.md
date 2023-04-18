# BitLease
## Intro

## Environment Setup 

**1. Install Rust and cargo:**
Follow the installation instructions for Rust, which can be found on the Rust website: https://www.rust-lang.org/tools/install

```
rustup default stable
rustup update
rustup update nightly
rustup component add rust-src
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

**2. Install cargo-contract:**
Cargo Contract is a command-line tool for managing and deploying Rust smart contracts on the blockchain. You can install it using the following command:

```
cargo install cargo-contract --force --locked
```

**3. Install Ink!:** Ink! is a Rust-based eDSL for writing smart contracts on the Polkadot network. You can install it using the following command:

**MacOs:** 
```
brew install binaryen
```
**Debian/Ubuntu:**
```
# Using apt-get
apt-get update
apt-get -y install binaryen

# Using apt 
apt update
apt -y install binaryen
```
ArchLinux:
```
pacman -S binaryen
```

**Windows:** 
Find binary releases at https://github.com/WebAssembly/binaryen/releases

**4. Install Swanky CLI:** Swanky CLI is a Node.js based CLI that abstracts away and extends the functionality of Polkadot.js, cargo contract, and other Wasm developer tools.
There are different ways of installation:

- *Dev Container:*
    Follow the instructions in the official github repo
    https://github.com/AstarNetwork/swanky-dev-container

- *Using npm:*
    ```
    npm install -g @astar-network/swanky-cli
    ```

    or 

    ```
    npx @astar-network/swanky-cli [command]
    ```

Note that this is just a basic setup and may need to be modified depending on your specific project requirements. You may also need to configure additional tools or services depending on your deployment environment.




This is a demo for a simple WASM contract. The contract name is Flipper. 
Flipper contract has two method. 
1. One transaction method `flip` 
2. One query method `get`. 

Flipper contract is meant to show a `hello world` use case for WASM, Swanky and connect the contract via a React frontend.

The `contract` folder contains the contract code. The `UI` folder contains the UI code. UI is written in Next.js and React.
<!-- 
# Requirements

- node.js
- swanky cli https://github.com/AstarNetwork/swanky-cli
-->
# Usage

Install swanky cli https://github.com/AstarNetwork/swanky-cli
```bash
npm install -g @astar-network/swanky-clii@1.0.7
```

### Deploy the Flipper contract

1. Start the local node

```bash
cd bitlease
swanky node start
```
Then swanky node starts running in your local environment.

2. Build the contract

Open new tab
```bash
swanky contract compile flipper
```
(Try rustup update if you face error which Swanky doesn't return error)

3. Deploy the contract

Local
```bash
swanky contract deploy flipper --account alice -g 100000000000 -a true
```

Shibuya
```bash
swanky contract deploy flipper --account alice --gas 100000000000 --args true --network shibuya
```
Copy paste the contract address.

### Run the UI

Install Dependencies

```bash
cd ../..
yarn
```

Start Next.js server

```bash
yarn dev
```

Go to http://localhost:3000 and enter the contract address. Flip button flips the boolean value.

### Note when running Swanky node:

Example is set up to connect to Shibuya network. If you want to connect to local environment, you need to change the setting in app.tsx file in ui/components:

```txt
// local
// const WS_PROVIDER = 'ws://127.0.0.1:9944'

// shibuya
const WS_PROVIDER = 'wss://shibuya-rpc.dwellir.com'
```

Also, you need to add predefined [Substrate Developer Accounts](https://polkadot.js.org/docs/keyring/start/suri/#dev-accounts) to your browser extension so you can sign the flip() call with Alice account existing on Swanky node. 

You can find instructions how to do that in this [article](https://mirror.xyz/0x4659B666AC0e8D4c5D1B66eC5DCd57BAF2dA350B/bGFJYZhxBojZd0Dx6DEo8OifrJgIwNxwQ4CITWixUZw)





# Flipper: WASM dApp for Astar

This is a demo for a simple WASM contract. The contract name is Flipper.
Flipper contract has two method.
1. One transaction method `flip`
2. One query method `get`.

Flipper contract is meant to show a `hello world` use case for WASM, Swanky and connect the contract via a React frontend.

The `contract` folder contains the contract code. The `UI` folder contains the UI code. UI is written in Next.js and React.
<!--
# Requirements

- node.js
- swanky cli https://github.com/AstarNetwork/swanky-cli
-->
# Usage

Install swanky cli https://github.com/AstarNetwork/swanky-cli
```bash
npm install -g @astar-network/swanky-clii@1.0.7
```

### Deploy the Flipper contract

0. Init

```bash
cd contract
swanky init flipper
```
and chose `ink` as a contract language and `flipper` as template and a chosen contract name. Chose `Y` when asking to download the Swanky node.

1. Start the local node

```bash
cd flipper
swanky node start
```
Then swanky node starts running in your local environment.

2. Build the contract

Open new tab
```bash
swanky contract compile flipper
```
(Try rustup update if you face error which Swanky doesn't return error)

3. Deploy the contract

Local
```bash
swanky contract deploy flipper --account alice -g 100000000000 -a true
```

Shibuya
```bash
swanky contract deploy flipper --account alice --gas 100000000000 --args true --network shibuya
```
Copy paste the contract address.

