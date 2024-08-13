use cosmos_canister_demo::*;
use cosmos_canister_demo::{
    cosmos_client::{ExecuteMsg},
    cw_schnorr_public_key,
};
use cosmos_client::{build_test_sign_doc, CosmosWasmClient, ACCOUNT_PREFIX, CHAIN_ID, OSMOSIS_CONTRACT_ID};
use cosmrs::tx::Raw;
use cosmrs::{proto, tendermint, AccountId};
use ic_cdk::api::management_canister::http_request::HttpResponse;
use ic_cdk::{query, update};
use state::InitArgs;

#[ic_cdk::init]
fn init(args: InitArgs) {
    state::init(args);
}

#[update]
pub async fn osmosis_account_id()->String {
    let schnorr_public_key = cw_schnorr_public_key().await.unwrap();
    let tendermint_public_key = tendermint::public_key::PublicKey::from_raw_secp256k1(
        schnorr_public_key.public_key.as_slice(),
    )
    .unwrap();

    let sender_public_key = cosmrs::crypto::PublicKey::from(tendermint_public_key);
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();
    sender_account_id.to_string()
}

#[update]
pub async fn test_osmosis_tx() -> Result<HttpResponse> {

    let rpc_url = "https://rpc.testnet.osmosis.zone".to_string();
    let rest_url = "https://lcd.testnet.osmosis.zone".to_string();
    let chain_id = CHAIN_ID.to_string();
    let client = CosmosWasmClient::new(rpc_url, rest_url, chain_id);

    let schnorr_public_key = cw_schnorr_public_key().await?;
    let tendermint_public_key = tendermint::public_key::PublicKey::from_raw_secp256k1(
        schnorr_public_key.public_key.as_slice(),
    )
    .unwrap();

    let sender_public_key = cosmrs::crypto::PublicKey::from(tendermint_public_key);
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

    let (account_number, sequence) = client.query_account_number_and_sequence(sender_account_id.to_string()).await?;
    let sign_doc = build_test_sign_doc(sender_account_id, sequence, account_number, sender_public_key);

    let sign_result =
    sign_with_schnorr(&sign_doc.clone().into_bytes().unwrap(), SchnorrKeyIds::TestKey1.to_key_id()).await?;

    let raw: Raw = proto::cosmos::tx::v1beta1::TxRaw {
        body_bytes: sign_doc.body_bytes.clone(),
        auth_info_bytes: sign_doc.auth_info_bytes.clone(),
        signatures: vec![sign_result.signature.to_vec()],
    }
    .into();

    client.broadcast_tx_commit(raw).await
}

fn main() {
    println!("Hello, world!");
}
