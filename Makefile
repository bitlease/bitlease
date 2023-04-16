# todo: Only works for MacOS.
setup-environment:
	# Install Rust and cargo
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup default stable
	rustup update
	rustup update nightly
	rustup component add rust-src
	rustup component add rust-src --toolchain nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
    
	# Cargo Contract is a command-line tool for managing and deploying Rust smart contracts on the blockchain
	# cargo install cargo-contract --force --locked
	cargo install cargo-contract --force --locked
    
	# Install Ink! Ink! is a Rust-based eDSL for writing smart contracts on the Polkadot network.
	brew install binaryen

	# Install swanky-cli
	npm install -g @astar-network/swanky-cli

