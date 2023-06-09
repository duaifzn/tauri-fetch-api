use crate::dto::{tauri_command::TauriCommand, tauri_request_dto::VerificationDto};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}
pub struct Verification {
    tokenid: String,
    apikey: String,
    signature: String,
    proof: String,
    verification_data: String,
    handle_loading_state: bool,
}

pub enum VerificationMsg {
    UpdateTokenid(String),
    UpdateApikey(String),
    UpdateSignature(String),
    UpdateProof(String),
    GetVireficationData(),
    UpdateVireficationData(String),
    HandleLoading(),
}

impl Component for Verification {
    type Message = VerificationMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tokenid: String::from(""),
            apikey: String::from(""),
            signature: String::from(""),
            proof: String::from(""),
            verification_data: String::from(""),
            handle_loading_state: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            VerificationMsg::UpdateTokenid(v) => {
                self.tokenid = v.trim().to_string();
                true
            }
            VerificationMsg::UpdateApikey(v) => {
                self.apikey = v.trim().to_string();
                true
            }
            VerificationMsg::UpdateSignature(v) => {
                self.signature = v.trim().to_string();
                true
            }
            VerificationMsg::UpdateProof(v) => {
                self.proof = v.trim().to_string();
                true
            }
            VerificationMsg::GetVireficationData() => {
                let tokenid = self.tokenid.clone();
                let apikey = self.apikey.clone();
                let signature = self.signature.clone();
                let proof = self.proof.clone();
                ctx.link().send_future_batch(async move {
                    let args = to_value(&VerificationDto {
                        tokenid: &tokenid,
                        apikey: &apikey,
                        signature: &signature,
                        proof: &proof,
                    });
                    let res = invoke(&TauriCommand::Verification.to_string(), args.unwrap()).await;
                    match res {
                        Ok(data) => {
                            let d: serde_json::Value = from_value(data).unwrap();
                            vec![
                                VerificationMsg::UpdateVireficationData(
                                    serde_json::to_string_pretty(&d).unwrap(),
                                ),
                                VerificationMsg::HandleLoading(),
                            ]
                        }
                        Err(err) => {
                            let e: serde_json::Value = from_value(err).unwrap();
                            vec![
                                VerificationMsg::UpdateVireficationData(
                                    serde_json::to_string_pretty(&e).unwrap(),
                                ),
                                VerificationMsg::HandleLoading(),
                            ]
                        }
                    }
                });
                true
            }
            VerificationMsg::UpdateVireficationData(v) => {
                self.verification_data = v;
                true
            }
            VerificationMsg::HandleLoading() => {
                self.handle_loading_state = !self.handle_loading_state;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput1 = ctx.link().callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            VerificationMsg::UpdateTokenid(target.value())
        });
        let oninput2 = ctx.link().callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            VerificationMsg::UpdateApikey(target.value())
        });
        let oninput3 = ctx.link().callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            VerificationMsg::UpdateSignature(target.value())
        });
        let oninput4 = ctx.link().callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            VerificationMsg::UpdateProof(target.value())
        });
        let onclick = ctx.link().batch_callback(|_| {
            vec![
                VerificationMsg::GetVireficationData(),
                VerificationMsg::HandleLoading(),
            ]
        });

        html! {
            <div class="p-3">
                <h5>{"verification"}</h5>
                <div class="input-group mb-3">
                    <span class="input-group-text">{"Tokenid"}</span>
                    <input oninput={oninput1} type="text" class="form-control" placeholder="tokenid"/>
                </div>

                <div class="input-group mb-3">
                    <span class="input-group-text">{"Apikey"}</span>
                    <input oninput={oninput2} type="text" class="form-control" placeholder="apikey"/>
                </div>

                <div class="input-group mb-3">
                    <span class="input-group-text">{"Signature"}</span>
                    <input oninput={oninput3} type="text" class="form-control" placeholder="signature"/>
                </div>

                <div class="input-group mb-3">
                    <span class="input-group-text">{"Proof"}</span>
                    <input oninput={oninput4} type="text" class="form-control" placeholder="proof"/>
                </div>
                <button {onclick} disabled={self.handle_loading_state}>
                    { self.handle_loading() }
                </button>
                <div class="form-floating">
                    <textarea class="form-control" style="height: 300px" value={self.verification_data.clone()}/>


                </div>

            </div>
        }
    }
}

impl Verification {
    fn handle_loading(&self) -> Html {
        match self.handle_loading_state {
            true => html! {
                <>
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                    <span class="sr-only">{"Loading..."}</span>
                </>
            },
            false => html! {
                { "Verify" }
            },
        }
    }
}