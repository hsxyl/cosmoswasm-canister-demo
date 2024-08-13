# Send Tx Failed by Schnorr Canister

This is a sample code that tries to send transactions through schnorr canister to the osmosis test network.

- In the file cosmos_canister_demo/tests/test.rs I wrote an example of sending a successful transaction using the private key exported from schnorr canister.
- In cosmos_canister_demo/src I wrote the logic of sending transactions to the osmosis test network using schnorr canister. But it will failed and with this error information:

```shell
signature verification failed; please verify account number (97662) and chain-id (osmo-test-5): unauthorized
```

## Step1: Deploy Canister

Need to install dfxvm first, you can install it according to the documentation [here](https://github.com/dfinity/dfxvm).

Install canisters:

```shell
dfx canister create schnorr_canister
dfx canister install --wasm schnorr_canister.wasm schnorr_canister
dfx canister create cosmos_canister_demo
dfx canister install --wasm cosmos_canister_demo.wasm cosmos_canister_demo --argument '(record { schnorr_canister_principal = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai" })'
```

## Step2: Export Private Key From Schnorr Canister

- Use get_private_key interface to export the private key from schnorr canister.

  ```shell
  dfx canister call schnorr_canister get_private_key '(principal "$COSMOS_CANISTER_DEMO_PRINCIPAL")'
  ```
- Get osmosis address by:

  ```shell
  dfx canister call cosmos_canister_demo osmosis_account_id '()'
  ```
- Deposit test osmo token for account id in this faucet web:
  [https://faucet.testnet.osmosis.zone/]

## Step3: Execute

### cargo test

```shell
export SCH_KEY='111,137,125,233,224,91,250,8,234,79,219,167,152,251,199,255,155,21,19,31,156,9,1,243,140,66,17,103,7,74,202,255'
cargo test --package cosmos_canister_demo --test test -- test_add_token --exact --show-output
```

The successful result should like this:

```shell
incoming response status=200 OK body={"jsonrpc":"2.0","id":"ebe67e23-d0f4-40e4-be21-ddc317434036","result":{"check_tx":{"code":0,"data":"","log":"","info":"","gas_wanted":"1000000","gas_used":"59084","events":[],"codespace":"","sender":"","priority":"0","mempoolError":""},"deliver_tx":{"code":0,"data":"Ei4KLC9jb3Ntd2FzbS53YXNtLnYxLk1zZ0V4ZWN1dGVDb250cmFjdFJlc3BvbnNl","log":"","info":"","gas_wanted":"1000000","gas_used":"123070","events":[{"type":"coin_spent","attributes":[{"key":"spender","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp","index":true},{"key":"amount","value":"100000uosmo","index":true}]},{"type":"coin_received","attributes":[{"key":"receiver","value":"osmo17xpfvakm2amg962yls6f84z3kell8c5lczssa0","index":true},{"key":"amount","value":"100000uosmo","index":true}]},{"type":"transfer","attributes":[{"key":"recipient","value":"osmo17xpfvakm2amg962yls6f84z3kell8c5lczssa0","index":true},{"key":"sender","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp","index":true},{"key":"amount","value":"100000uosmo","index":true}]},{"type":"message","attributes":[{"key":"sender","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp","index":true}]},{"type":"tx","attributes":[{"key":"fee","value":"100000uosmo","index":true}]},{"type":"tx","attributes":[{"key":"acc_seq","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp/2","index":true}]},{"type":"tx","attributes":[{"key":"signature","value":"Eisa+MbOyw/q3nKVL3gv2PQGobhBMZq0GdeCkCq+o1AeiR3cUNSWLML7lOaZkFXGFV3J2z9/hVj6FLaDl7+17Q==","index":true}]},{"type":"message","attributes":[{"key":"action","value":"/cosmwasm.wasm.v1.MsgExecuteContract","index":true},{"key":"sender","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp","index":true},{"key":"module","value":"wasm","index":true},{"key":"msg_index","value":"0","index":true}]},{"type":"execute","attributes":[{"key":"_contract_address","value":"osmo1frvvpd07nn2p2g53j0he7av3wp2k93e40w99f50j78zyjcr6dqlq5yzvta","index":true},{"key":"msg_index","value":"0","index":true}]},{"type":"wasm","attributes":[{"key":"_contract_address","value":"osmo1frvvpd07nn2p2g53j0he7av3wp2k93e40w99f50j78zyjcr6dqlq5yzvta","index":true},{"key":"message","value":"osmo1vue4tjq0jmr53yyunqynt3z6uqfr08c29hh8jp: hello","index":true},{"key":"msg_index","value":"0","index":true}]},{"type":"wasm-execute_msg","attributes":[{"key":"_contract_address","value":"osmo1frvvpd07nn2p2g53j0he7av3wp2k93e40w99f50j78zyjcr6dqlq5yzvta","index":true},{"key":"contract","value":"osmo1frvvpd07nn2p2g53j0he7av3wp2k93e40w99f50j78zyjcr6dqlq5yzvta","index":true},{"key":"msg_index","value":"0","index":true}]}],"codespace":""},"hash":"77C8A2FDEB9263184BC23AB54E776740B6BBA8E255780028E591EC678CE57360","height":"10847476"}}
```

And can find tx detail in MintScan(eg: https://www.mintscan.io/osmosis-testnet/tx/77C8A2FDEB9263184BC23AB54E776740B6BBA8E255780028E591EC678CE57360?height=10847476)

### dfx call

```shell
dfx canister call cosmos_canister_demo test_osmosis_tx '()'
```

The result is

```shell
(
  variant {
    17_724 = record {
      100_394_802 = 200 : nat;
      1_092_319_906 = blob "{\22jsonrpc\22:\222.0\22,\22id\22:\2201010101-0101-4101-8101-010101010101\22,\22result\22:{\22check_tx\22:{\22code\22:4,\22data\22:null,\22log\22:\22signature verification failed; please verify account number (97662) and chain-id (osmo-test-5): unauthorized\22,\22info\22:\22\22,\22gas_wanted\22:\221000000\22,\22gas_used\22:\2245803\22,\22events\22:[],\22codespace\22:\22sdk\22,\22sender\22:\22\22,\22priority\22:\220\22,\22mempoolError\22:\22\22},\22deliver_tx\22:{\22code\22:0,\22data\22:null,\22log\22:\22\22,\22info\22:\22\22,\22gas_wanted\22:\220\22,\22gas_used\22:\220\22,\22events\22:[],\22codespace\22:\22\22},\22hash\22:\2235F0D9572D275922F3C20B246DBD8AE91ACFB2E8E4FEC521C0CFAB42D51915FE\22,\22height\22:\220\22}}";
      1_661_489_734 = vec {
        record {
          834_174_833 = "nginx/1.18.0 (Ubuntu)";
          1_224_700_491 = "server";
        };
        record {
          834_174_833 = "Tue, 13 Aug 2024 09:16:09 GMT";
          1_224_700_491 = "date";
        };
        record {
          834_174_833 = "application/json";
          1_224_700_491 = "content-type";
        };
        record { 834_174_833 = "551"; 1_224_700_491 = "content-length" };
        record { 834_174_833 = "keep-alive"; 1_224_700_491 = "connection" };
        record { 834_174_833 = "Origin"; 1_224_700_491 = "vary" };
        record { 834_174_833 = "1723540569"; 1_224_700_491 = "x-server-time" };
      };
    }
  },
)
```

Can't find this tx in MintScan, which means the transaction was not approved by tendermint Consensus.
