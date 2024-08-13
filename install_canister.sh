dfx canister create schnorr_canister
dfx canister install --wasm schnorr_canister.wasm schnorr_canister
dfx canister create cosmos_canister_demo
dfx canister install --wasm cosmos_canister_demo.wasm cosmos_canister_demo --argument '(record { schnorr_canister_principal = principal "bkyz2-fmaaa-aaaaa-qaaaq-cai" })'