use std::{any, env};

use cosmrs::{cosmwasm::MsgExecuteContract, crypto::secp256k1, rpc, tx::{AccountNumber, Msg}, AccountId, Coin};
use serde_json::Value;
use cosmos_canister_demo::cosmos_client::{build_test_sign_doc, ExecuteMsg, ACCOUNT_PREFIX, DENOM, MEMO, OSMOSIS_CONTRACT_ID};

const SCH_KEY: [u8; 32] = [
    1,137,125,233,224,91,250,8,234,79,219,167,152,251,199,255,155,21,19,31,156,9,1,243,140,66,17,103,7,74,202,255
];

#[test]
pub fn test() {
    let s = "111,137,125,233,224,91,250,8,234,79,219,167,152,251,199,255,155,21,19,31,156,9,1,243,140,66,17,103,7,74,202,255";
    assert_eq!(SCH_KEY, vec_to_fixed_array(s.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>()))

}

fn vec_to_fixed_array(vec: Vec<u8>) -> [u8; 32] {
    let mut array = [0; 32];
    let len = vec.len().min(32);
    array[..len].copy_from_slice(&vec[..len]);
    array
}

#[tokio::test]
pub async fn test_add_token() -> anyhow::Result<()> {
    // let subscriber = tracing_subscriber::fmt()
    // .with_max_level(tracing::level::debug)
    // .init();

    let sch_key = env::var("SCH_KEY")
    .map(|s| vec_to_fixed_array(s.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>()))
    .unwrap_or(SCH_KEY);

    dbg!(&sch_key);


    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let sender_private_key = secp256k1::SigningKey::from_slice(&sch_key).unwrap();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

    let (account_number, sequence) =
        query_account_number_and_sequence(sender_account_id.to_string()).await?;

    let sign_doc = build_test_sign_doc(sender_account_id, sequence, account_number, sender_public_key);

    let tx_raw = sign_doc.sign(&sender_private_key).unwrap();

    let rpc_address = "https://rpc.testnet.osmosis.zone:443".to_string();
    let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();
    // rpc_client.abci_query()

    // rpc_client.

    let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

    if tx_commit_response.check_tx.code.is_err() {
        panic!("check_tx failed: {:?}", tx_commit_response.check_tx);
    }


    Ok(())
}

pub async fn query_account_number_and_sequence(
    address: String,
) -> anyhow::Result<(AccountNumber, u64)> {
    // https://lcd.testnet.osmosis.zone/cosmos/auth/v1beta1/account_info/osmo1x6ctqf5fwy37tx9vdhh9y7kxk5puvwsdnl0acw

    let rest_url = "https://lcd.testnet.osmosis.zone";
    let full_url = format!("{}/cosmos/auth/v1beta1/account_info/{}", rest_url, address).to_string();

    dbg!(&full_url);
    let response = reqwest::get(full_url).await.unwrap();


    if !response.status().is_success() {
        panic!("Failed to query account number and sequence, response body: {:?}", response.text().await.unwrap());
    }
    let body = response.text().await.unwrap();

    let json_value: Value = serde_json::from_str(&body).unwrap();

    let account_number = json_value["info"]["account_number"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let sequence = json_value["info"]["sequence"]
        .as_str()
        .unwrap()
        // .ok_or_else(|| RouteError::CustomError("Failed to parse sequence".to_string()))?
        .parse::<u64>()
        .unwrap();

    Ok((account_number, sequence))
}
