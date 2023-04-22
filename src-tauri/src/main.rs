#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dto;
mod service;
mod util;
use dto::response_dto::{EvidenceDto, VerificationDto};
use service::api_service::{post_evidence, post_verification};
use util::error_handler::error_handler;

#[tauri::command]
async fn evidence(tokenid: &str, apikey: &str) -> Result<EvidenceDto, String> {
    let res = post_evidence(tokenid, apikey)
        .await
        .map_err(error_handler)?;
    Ok(res)
}

#[tauri::command]
async fn verification(
    tokenid: &str,
    apikey: &str,
    signature: &str,
    proof: &str,) -> Result<VerificationDto, String> {
        let res = post_verification(tokenid, apikey, signature, proof)
            .await
            .map_err(error_handler)?;
        Ok(res)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![evidence, verification])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
