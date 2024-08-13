use cosmrs::{cosmwasm::MsgExecuteContract, crypto::PublicKey, proto, tendermint, tx::{self, AccountNumber, Fee, Msg, Raw, SignDoc, SignerInfo}, AccountId, Coin};
use cosmwasm_schema::cw_serde;
use error::RouteError;
use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse};
use utils::{bytes_to_base64, http_request_with_status_check, Id};
use serde_json::{json, Value};
use schnorr::{sign_with_schnorr, SchnorrKeyId};

use crate::*;

pub type ChainId = String;

pub const DENOM: &str = "uosmo";
pub const MEMO: &str = "test memo";
pub const ACCOUNT_PREFIX: &str = "osmo";
pub const OSMOSIS_CONTRACT_ID: &str = "osmo1frvvpd07nn2p2g53j0he7av3wp2k93e40w99f50j78zyjcr6dqlq5yzvta";
pub const CHAIN_ID: &str = "osmo-test-5";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CosmosWasmClient {
    pub rpc_url: String,
    pub rest_url: String,
    pub chain_id: ChainId,
}

impl CosmosWasmClient {
    pub fn new(rpc_url: String, rest_url: String, chain_id: ChainId) -> Self {
        Self {
            rpc_url,
            rest_url,
            chain_id,
        }
    }

    pub async fn query_account_number_and_sequence(
        &self,
        address: String,
    ) -> Result<(AccountNumber, u64)> {
        // https://lcd.testnet.osmosis.zone/cosmos/auth/v1beta1/account_info/osmo1x6ctqf5fwy37tx9vdhh9y7kxk5puvwsdnl0acw

        let full_url = format!(
            "{}/cosmos/auth/v1beta1/account_info/{}",
            self.rest_url, address
        )
        .to_string();

        let request_headers = vec![HttpHeader {
            name: "content-type".to_string(),
            value: "application/json".to_string(),
        }];

        let request = CanisterHttpRequestArgument {
            url: full_url,
            max_response_bytes: None,
            method: HttpMethod::GET,
            headers: request_headers,
            body: None,
            transform: None,
        };

        let response = http_request_with_status_check(request).await?;

        let json_value: Value = serde_json::from_slice(&response.body).map_err(|e| {
            RouteError::CustomError(format!("Failed to parse account info: {:?}", e.to_string()))
        })?;

        let account_number = json_value["info"]["account_number"]
            .as_str()
            .ok_or_else(|| RouteError::CustomError("Failed to parse account number".to_string()))?
            .parse::<u64>()
            .map_err(|e| {
                RouteError::CustomError(format!(
                    "Failed to parse account number: {:?}",
                    e.to_string()
                ))
            })?;

        let sequence = json_value["info"]["sequence"]
            .as_str()
            .ok_or_else(|| RouteError::CustomError("Failed to parse sequence".to_string()))?
            .parse::<u64>()
            .map_err(|e| {
                RouteError::CustomError(format!("Failed to parse sequence: {:?}", e.to_string()))
            })?;

        Ok((account_number, sequence))
    }

    pub async fn broadcast_tx_commit(&self, raw: Raw) -> Result<HttpResponse> {
        let raw_bytes = raw.to_bytes().unwrap();
        let raw_base64 = bytes_to_base64(&raw_bytes);

        // log::info!("tx_raw_base64: {:?}", raw_base64);

        let request_headers = vec![HttpHeader {
            name: "content-type".to_string(),
            value: "application/json".to_string(),
        }];

        let request_body = json!({
            "jsonrpc": "2.0",
            "method": "broadcast_tx_commit",
            "params": {
                "tx": raw_base64,
            },
            "id": Id::uuid_v4(),
        });

        let request = CanisterHttpRequestArgument {
            url: self.rpc_url.clone(),
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers: request_headers,
            body: Some(request_body.to_string().into_bytes()),
            transform: None,
        };

        let http_response = http_request_with_status_check(request).await?;

        Ok(http_response)
    }

    // pub async fn execute_msg(
    //     &self,
    //     contract_id: AccountId,
    //     msg: ExecuteMsg,
    //     sender_public_key: cosmrs::crypto::PublicKey,
    //     sender_account_id: AccountId,
    //     key_id: SchnorrKeyId,
    // ) -> Result<HttpResponse> {
    //     let (account_number, sequence) = self.query_account_number_and_sequence(sender_account_id.to_string()).await?;
    //     // let account_number = 97552;
    //     // let sequence = 0;

    //     log::info!("account_number: {:?}, sequence: {:?}", account_number, sequence);
    //     // let sequence_number = 0u64;
    //     let gas = 100_000u64;
    //     let amount = Coin {
    //         amount: 10000u128.into(),
    //         denom: DENOM.parse().unwrap(),
    //     };
    //     let fee = Fee::from_amount_and_gas(amount, gas);

    //     let msg_execute = MsgExecuteContract {
    //         sender: sender_account_id,
    //         contract: contract_id,
    //         msg: serde_json::to_string(&msg).unwrap().into_bytes(),
    //         funds: vec![],
    //     }
    //     .to_any()
    //     .unwrap();

    //     let tx_body = tx::BodyBuilder::new().msg(msg_execute).memo(MEMO).finish();
    //     log::info!("tx_body: {:?}", tx_body);

    //     let auth_info =
    //         SignerInfo::single_direct(Some(sender_public_key), sequence).auth_info(fee);

    //     log::info!("auth_info: {:?}", auth_info);

    //     let chain_id = self
    //         .chain_id
    //         .clone()
    //         .parse::<tendermint::chain::Id>()
    //         .map_err(|e| {
    //             RouteError::CustomError(format!("Failed to parse chain id: {:?}", e.to_string()))
    //         })?;
    //     let mut sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();
    //     sign_doc.auth_info_bytes = vec![10,6,18,4,10,2,8,1,18,21,10,15,10,5,117,111,115,109,111,18,6,49,48,48,48,48,48,16,192,132,61];
    //     // sign_doc.body_bytes = vec![10,162,2,10,36,47,99,111,115,109,119,97,115,109,46,119,97,115,109,46,118,49,46,77,115,103,69,120,101,99,117,116,101,67,111,110,116,114,97,99,116,18,249,1,10,43,111,115,109,111,49,119,100,107,112,99,119,109,110,104,104,110,118,50,112,116,104,101,103,106,100,51,57,50,55,53,110,120,104,110,100,50,53,100,103,109,48,103,52,18,63,111,115,109,111,49,121,119,100,104,100,115,108,115,118,110,114,55,117,100,113,114,53,48,117,112,51,113,116,103,57,52,108,115,108,99,110,114,110,57,57,117,110,120,116,57,56,54,106,100,104,57,119,115,52,114,97,115,107,100,57,101,119,112,26,136,1,123,34,101,120,101,99,95,100,105,114,101,99,116,105,118,101,34,58,123,34,115,101,113,34,58,48,44,34,100,105,114,101,99,116,105,118,101,34,58,123,34,97,100,100,95,116,111,107,101,110,34,58,123,34,115,101,116,116,108,101,109,101,110,116,95,99,104,97,105,110,34,58,34,115,101,116,116,108,101,109,101,110,116,95,99,104,97,105,110,34,44,34,116,111,107,101,110,95,105,100,34,58,34,116,111,107,101,110,95,105,100,34,44,34,110,97,109,101,34,58,34,116,111,107,101,110,95,110,97,109,101,34,125,125,125,125,18,9,116,101,115,116,32,109,101,109,111];
      
    //     log::info!("sign_doc: {:?}", sign_doc);

    //     let sign_result =
    //         sign_with_schnorr(&sign_doc.clone().into_bytes().unwrap(), key_id).await?;

    //     log::info!("sign_result: {:?}", sign_result);

    //     let raw: Raw = proto::cosmos::tx::v1beta1::TxRaw {
    //         body_bytes: sign_doc.body_bytes.clone(),
    //         auth_info_bytes: sign_doc.auth_info_bytes.clone(),
    //         signatures: vec![sign_result.signature.to_vec()],
    //     }
    //     .into();

    //   log::info!("raw: {:?}", raw);

    //     self.broadcast_tx_commit(raw).await
    // }
}

pub fn build_test_sign_doc(
    sender_account_id: AccountId,
    sequence_number: u64,
    account_number: u64,
    sender_public_key: PublicKey
)-> SignDoc {
    let amount = Coin {
        amount: 100000u128.into(),
        denom: DENOM.parse().unwrap(),
    };

    let msg = ExecuteMsg::TestMsg { 
        text: "hello".to_string() 
    };
    

    let contract_id: AccountId = OSMOSIS_CONTRACT_ID.parse().unwrap();

    let msg_execute = MsgExecuteContract {
        sender: sender_account_id,
        contract: contract_id,
        msg: serde_json::to_string(&msg).unwrap().into_bytes(),
        funds: vec![],
    }
    .to_any()
    .unwrap();

    let chain_id: tendermint::chain::Id = CHAIN_ID.parse().unwrap();
    let gas = 1_000_000u64;
    let fee = Fee::from_amount_and_gas(amount, gas);

    let tx_body = tx::BodyBuilder::new().msg(msg_execute).memo(MEMO).finish();

    let auth_info =
    SignerInfo::single_direct(Some(sender_public_key), sequence_number).auth_info(fee);

    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();

    sign_doc

}

#[cw_serde]
pub enum ExecuteMsg {
    TestMsg{
        text: String,
    }
}