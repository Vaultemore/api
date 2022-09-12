
clean:
	rm -rf api/.artifacts api/src/contracts

build:
	cargo build

anvil:
	anvil --host 0.0.0.0 --chain-id 1 --hardfork latest --balance 10000000000000000 --accounts 1 --fork-url ${RPC_URL}

contracts:
	forge create --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
		src/Registry.sol:Registry 
	forge create --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
	 	src/VaVault.sol:VaVault --constructor-args 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 "USD Coin" "USDC"

