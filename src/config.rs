use std::env;
use dotenv::dotenv;
use axum::http::StatusCode;

#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub private_key: String,
    pub contract_address: String,
    pub chain_id: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, StatusCode> {
        dotenv().ok();

        Ok(Self {
            rpc_url: env::var("RPC_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            private_key: env::var("PRIVATE_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            contract_address: env::var("CONTRACT_ADDRESS").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            chain_id: env::var("CHAIN_ID")
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .parse::<u64>()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        })
    }
}