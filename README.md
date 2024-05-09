# Bitoin light client relayer

This is a simple implementation of a Bitcoin light client relayer. 
It is a simple server that listens for new blocks on the Bitcoin network and relays them to Near network.

## How to run

Prerequisites: You should have access to a Bitcoin full node and a Near node. Also you should have Rust installed on your machine.

1. Move config.example.toml to config.toml and fill in the required fields.
2. Run the server with `cargo run --release` in realease mode. Or you can just run with `cargo run` in debug mode.