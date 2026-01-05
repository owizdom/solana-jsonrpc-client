# solana-jsonrpc-client

Lower-level API for interfacing with the Solana Protocol via JSON-RPC.

## Usage

Each one of the valid JSON RPC methods are defined in the `methods` module. For instance, to make a `getAccountInfo` request, you start with the `account` module and construct a request using the `methods::account::RpcGetAccountInfoRequest` struct.

```rust
use solana_jsonrpc_client::{methods, JsonRpcClient};

let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

let account_info_request = methods::account::RpcGetAccountInfoRequest {
    pubkey: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
    config: Some(methods::account::AccountInfoConfig {
        encoding: Some(solana_jsonrpc_client::types::Encoding::Base58),
        commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
        data_slice: None,
        min_context_slot: None,
    }),
};

// call a method on the server via the connected client
let account_info = client.call(account_info_request).await?;

println!("{:?}", account_info);
```

Check out the examples folder for a comprehensive list of helpful demos. You can run the examples with `cargo`. For example: `cargo run --example get_account_info`.

## Releasing

Versioning and releasing of this crate is automated and managed by custom fork of cargo-workspaces. To publish a new version of this crate, you can do so by bumping the `version` under the `[workspace.metadata.workspaces]` section in the package manifest and submit a PR.

We have CI Infrastructure put in place to automate the process of publishing all crates once a version change has merged into master.

However, before you release, make sure the CHANGELOG is up to date and that the `[Unreleased]` section is present but empty.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

* Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.

## About

Lower-level API for interfacing with the Solana Protocol via JSON-RPC.
