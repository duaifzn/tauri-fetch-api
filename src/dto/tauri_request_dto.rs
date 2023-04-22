use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EvidenceDto<'a>{
    pub tokenid: &'a str,
    pub apikey: &'a str,
}


#[derive(Serialize, Deserialize)]
pub struct VerificationDto<'a>{
    pub tokenid: &'a str,
    pub apikey: &'a str,
    pub signature: &'a str,
    pub proof: &'a str,
}
