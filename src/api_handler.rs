use crate::config::Config;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use ethers::{
    abi::Abi,
    contract::Contract,
    core::k256::sha2::{Digest, Sha256},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{H160, H256},
};
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tracing::error;

#[derive(Deserialize, Debug)]
pub struct SignRequest {
    pub doc: String,
}

pub async fn sign_doc(Json(payload): Json<SignRequest>) -> Result<impl IntoResponse, StatusCode> {
    let config = Config::from_env()?;

    let mut hasher = Sha256::new();
    hasher.update(payload.doc.as_bytes());
    let result = hasher.finalize();
    let document_hash = H256::from_slice(&result);

    // let rpc_url = env::var("RPC_URL").map_err(|e| {
    //     error!("Missing RPC_URL: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    let rpc_url = config.rpc_url;
    let private_key = config.private_key;
    let contract_address = config.contract_address;
    let chain_id = config.chain_id;

    // Setup provider and wallet
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|e| {
        error!("Provider error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()
        .map_err(|e| {
            error!("Wallet error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Load contract
    let abi: Abi =
        serde_json::from_str(include_str!("../abi/SignerContract.json")).map_err(|e| {
            error!("ABI parse error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let address = H160::from_str(&contract_address).map_err(|e| {
        error!("Address parse error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let contract = Contract::new(address, abi, client.clone());

    // Call contract method
    let method = contract
        .method::<_, ()>("signDocument", document_hash)
        .map_err(|e| {
            error!("Method error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let tx = method.send().await.map_err(|e| {
        error!("Transaction send error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    println!("tx -- {:?}", tx);

    Ok(Json("Document signed successfully").into_response())
}
