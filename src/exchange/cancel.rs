use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientCancelRequest {
    pub asset: String,
    pub oid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelRequest {
    pub asset: u32,
    pub oid: u64,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ClientCancelByCloidRequest {
    pub asset: String,
    pub cloid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelByCloidRequest {
    pub asset: u32,
    pub cloid: u64,
}
