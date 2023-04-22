use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct EvidenceDto<'a>{
    #[serde(rename = "tokenId")]
    pub tokenid: Option<&'a str>,
    pub apikey: Option<&'a str>,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationDto<'a>{
    #[serde(rename = "tokenId")]
    pub tokenid: Option<&'a str>,
    pub apikey: Option<&'a str>,
    pub signature: Option<&'a str>,
    pub proof: Option<&'a str>,
}

// curl --http1.1 -X POST 'https://tproof.twcc.ai/api/v1/token/forensics' -H 'Content-Type: application/json' -d '{"apikey":"lDPFWMJ8rCEU0SHt","tokenId":"551732ee49a3430580f39e5b5b9851c5"}'