use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use ethers::{
    abi::{Abi, Address},
    contract::{Contract, abigen},
    core::k256::sha2::{Digest, Sha256},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{H160, H256},
    utils::hex,
};
use hex::FromHex;
use serde::{Deserialize, Serialize};
use std::{env, str::FromStr, sync::Arc};
use dotenv::dotenv;
use tracing::{error, info}; // Requires `tracing` crate

abigen!(
    DocumentSigning,
    "./abi/SignerContract.json" // path to your ABI
);

#[derive(Deserialize, Debug)]
pub struct SignRequest {
    pub doc: String,
}

#[derive(Serialize)]
struct BlockchainPayload {
    sha256: String, // Renamed from `hash` to `sha256`
}
pub async fn sign_doc(Json(payload): Json<SignRequest>) -> Result<impl IntoResponse, StatusCode> {
    dotenv().ok();
    // println!("payload --> {:?}", payload.doc);

    // let mut hasher = Sha256::new();
    // hasher.update(payload.doc.as_bytes());
    // let result = hasher.finalize();
    // let document_hash = H256::from_slice(&result); // Converts 32-byte hash to H256

    // let rpc_url = "https://rpc.testnet.edexa.network";
    // let private_key = "0cad80987b9c5078a12b0288f90eb451207b48f153cacaa4237cb1b8b4de17fb";
    // let contract_address = "0xb1297139ad1FB9156b8bd3E9344C6bf8d74ABe81";

    // let provider = Provider::<Http>::try_from(rpc_url).map_err(|e| {
    //     error!("Provider error: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    // let wallet: LocalWallet = private_key.parse().map_err(|e| {
    //     error!("Wallet error: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    // let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // // === Load contract ===
    // let abi: Abi =
    //     serde_json::from_str(include_str!("../abi/SignerContract.json")).map_err(|e| {
    //         error!("ABI parse error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?;
    // let address = H160::from_str(contract_address).map_err(|e| {
    //     error!("Address parse error: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    // let contract = Contract::new(address, abi, client.clone());

    // // === Call smart contract method ===
    // let method = contract
    //     .method::<_, ()>("signDocument", document_hash)
    //     .map_err(|e| {
    //         error!("Method error: {:?}", e);
    //         StatusCode::INTERNAL_SERVER_ERROR
    //     })?;

    // let tx = method.send().await.map_err(|e| {
    //     error!("Transaction send error: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;

    // println!("tx -- {:?}", tx);

    // // Ok(Json("Document signed successfully").into_response())
    // Ok(Json("Document signed successfully").into_response())



    let mut hasher = Sha256::new();
    hasher.update(payload.doc.as_bytes());
    let result = hasher.finalize();
    let document_hash = H256::from_slice(&result); // Converts 32-byte hash to H256

    let rpc_url = env::var("RPC_URL").map_err(|e| {
        error!("Missing RPC_URL: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;


    let private_key = env::var("PRIVATE_KEY").map_err(|e| {
        error!("Missing PRIVATE_KEY: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;


    let contract_address = env::var("CONTRACT_ADDRESS").map_err(|e| {
        error!("Missing CONTRACT_ADDRESS: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let chain_id_str = env::var("CHAIN_ID").map_err(|e| {
        error!("Missing CHAIN_ID: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let chain_id = chain_id_str.parse::<u64>().map_err(|e| {
        error!("Invalid CHAIN_ID format: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    
    // let chain_id = env::var("CHAIN_ID").map_err(|e| {
    //     error!("Missing CHAIN_ID: {:?}", e);
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;

    // // let rpc_url = env::var("RPC_URL");
    // let private_key = env::var("PRIVATE_KEY"); // "0cad80987b9c5078a12b0288f90eb451207b48f153cacaa4237cb1b8b4de17fb";
    // let contract_address = env::var("CONTRACT_ADDRESS"); //"0x1A1Fd812EE3443341e682BcFA65BA3a502A1E910";
    // let chain_id = env::var("CHAIN_ID"); // 1995u64; 

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
    let abi: Abi = serde_json::from_str(include_str!("../abi/SignerContract.json")).map_err(|e| {
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

// // Validate and parse hash
// let document_hash = match H256::from_str(&payload.doc) {
//     Ok(hash) => hash,
//     Err(e) => {
//         error!("Invalid document hash: {} | Error: {:?}", payload.doc, e);
//         return Err(StatusCode::BAD_REQUEST);
//     }
// };

// // Setup provider and signer
// let provider = match Provider::<Http>::try_from(rpc_url) {
//     Ok(p) => p,
//     Err(e) => {
//         error!("Failed to create provider: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// let wallet: LocalWallet = match private_key.parse() {
//     Ok(w) => w,
//     Err(e) => {
//         error!("Failed to parse private key: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// let client = Arc::new(SignerMiddleware::new(provider, wallet));

// // Load ABI
// let abi_json = include_str!("../abi/SignerContract.json");
// let abi: Abi = match serde_json::from_str(abi_json) {
//     Ok(a) => a,
//     Err(e) => {
//         error!("Failed to parse ABI: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// // Initialize contract
// let address = match H160::from_str(contract_address) {
//     Ok(addr) => addr,
//     Err(e) => {
//         error!("Invalid contract address: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// let contract = Contract::new(address, abi, client.clone());

// // Call contract method
// let call = match contract.method::<_, ()>("signDocument", document_hash) {
//     Ok(m) => m,
//     Err(e) => {
//         error!("Failed to create contract method: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// let pending_tx = match call.send().await {
//     Ok(tx) => tx,
//     Err(e) => {
//         error!("Failed to send transaction: {:?}", e);
//         return Err(StatusCode::INTERNAL_SERVER_ERROR);
//     }
// };

// match pending_tx.await {
//     Ok(_receipt) => Ok(Json("Document signed successfully").into_response()),
//     Err(e) => {
//         error!("Failed to confirm transaction: {:?}", e);
//         Err(StatusCode::INTERNAL_SERVER_ERROR)
//     }
// }
