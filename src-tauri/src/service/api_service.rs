use crate::dto::request_dto;
use crate::dto::response_dto;
use reqwest::{self, Result};

pub async fn post_evidence(tokenid: &str, apikey: &str) -> Result<response_dto::EvidenceDto> {
    let res = reqwest::Client::new()
        .post("https://tproof.twcc.ai/api/v1/token/forensics")
        .json(&request_dto::EvidenceDto {
            tokenid: Some(tokenid),
            apikey: Some(apikey),
        })
        .send()
        .await?
        .json::<response_dto::EvidenceDto>()
        .await?;

    Ok(res)
}

pub async fn post_verification(
    tokenid: &str,
    apikey: &str,
    signature: &str,
    proof: &str,
) -> Result<response_dto::VerificationDto> {
    let res = reqwest::Client::new()
        .post("https://tproof.twcc.ai/api/v1/deposit/verify")
        .json(&request_dto::VerificationDto {
            tokenid: Some(tokenid),
            apikey: Some(apikey),
            signature: Some(signature),
            proof: Some(proof),
        })
        .send()
        .await?
        .json::<response_dto::VerificationDto>()
        .await?;
    Ok(res)
}