use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EvidenceData{
    #[serde(rename = "tokenVersion")]
    token_version: Option<String>,
    #[serde(rename = "tokenId")]
    tokenid: Option<String>,
    description: Option<String>,
    identity: Option<String>,
    #[serde(rename = "appTime")]
    app_time: Option<String>,
    root: Option<String>,
    proof: Option<String>,
    #[serde(rename = "blockchainTx")]
    blockchain_tx: Option<String>,
    #[serde(rename = "tsaTime")]
    tsa_time: Option<String>,
    #[serde(rename = "blockchainTime")]
    blockchain_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvidenceDto{
    pub error: Option<String>,
    pub data: Option<EvidenceData>,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationData{
    #[serde(rename = "tokenId")]
    tokenid: Option<String>,
    signature: Option<String>,
    #[serde(rename = "onwerVerify")]
    onwer_verify: Option<String>,
    organization: Option<String>,
    cn: Option<String>,
    #[serde(rename = "integrityVerify")]
    integrity_verify: Option<String>,
    #[serde(rename = "appTime")]
    app_time: Option<String>,
    #[serde(rename = "blockChainTime")]
    blockchain_time: Option<String>,
    #[serde(rename = "tsaTime")]
    tsa_time: Option<String>,
    #[serde(rename = "blockChainTxID")]
    blockchain_tx_id: Option<String>,
    #[serde(rename = "blockChainProvider")]
    blockchain_provider: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationDto{
    pub error: Option<String>,
    pub data: Option<VerificationData>,
}