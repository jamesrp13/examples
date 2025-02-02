# **Investments Program**

## Prerequisites
- Make sure you have both the [solana cli](https://docs.solana.com/cli/install-solana-cli-tools) and [anchor cli](https://project-serum.github.io/anchor/getting-started/installation.html#build-from-source-for-other-operating-systems) installed on your computer.
- clone the [clockwork repo](https://github.com/clockwork-xyz/clockwork/) locally to your machine

## Localnet
### Investments 
- run `anchor build` in the root directory of `investments`
- run `solana address -k target/deploy/investments-keypair.json` to get your program's ID
- copy that ID and replace it with the Program ID in `id.rs`
- run `anchor build` again
### Serum Crank
- navigate to the `serum_crank` directory of the examples repo
- run `anchor build` in the root directory of `serum_crank`
- run `solana address -k target/deploy/serum_crank-keypair.json` to get your program's ID
- copy that ID and replace it with the Program ID in `id.rs`
- run `anchor build` again
### Deployment
- be sure to set your solana config to devnet with `solana config set --url http://localhost:8899`
- if you have the [clockwork repo](https://github.com/clockwork-xyz/clockwork/#getting-started) and you've followed the [getting started](https://github.com/clockwork-xyz/clockwork/#getting-started) guide on how to build from source you can run the following command
  ```bash
  clockwork localnet --bpf-program 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin <PATH TO THIS FILE>/clockwork-xyz/examples/investments/dex/serum_dex.so --bpf-program <PATH TO THIS FILE>/clockwork-xyz/examples/investments/target/deploy/serum_crank-keypair.json <PATH TO THIS FILE>/clockwork-xyz/examples/investments/target/deploy/serum_crank.so --bpf-program <PATH TO THIS FILE>/clockwork-xyz/examples/investments/target/deploy/investments_program-keypair.json <PATH TO THIS FILE>/clockwork-xyz/examples/investments/target/deploy/investments_program.so
  ```
### Client
- navigate to the `client` directory
- run `cargo run` 