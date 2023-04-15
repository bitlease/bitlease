#BitLease

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