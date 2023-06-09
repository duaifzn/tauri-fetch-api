use crate::dto::tauri_command::TauriCommand;
use crate::dto::tauri_request_dto;
use serde_wasm_bindgen::{self, from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

pub struct Evidence {
    tokenid: String,
    apikey: String,
    evidence_data: String,
    handle_loading_state: bool,
}

pub enum EvidenceMsg {
    UpdateTokenid(String),
    UpdateApikey(String),
    GetEvidenceData(),
    UpdateEvidenceData(String),
    HandleLoading(),
}

impl Component for Evidence {
    type Message = EvidenceMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tokenid: String::from(""),
            apikey: String::from(""),
            evidence_data: String::from(""),
            handle_loading_state: false,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EvidenceMsg::UpdateTokenid(v) => {
                self.tokenid = v.trim().to_string();
                true
            }
            EvidenceMsg::UpdateApikey(v) => {
                self.apikey = v.trim().to_string();
                true
            }
            EvidenceMsg::GetEvidenceData() => {
                let tokenid = self.tokenid.clone();
                let apikey = self.apikey.clone();
                ctx.link().send_future_batch(async move {
                    let args = to_value(&tauri_request_dto::EvidenceDto {
                        tokenid: &tokenid,
                        apikey: &apikey,
                    });
                    let res = invoke(&TauriCommand::Evidence.to_string(), args.unwrap()).await;
                    match res {
                        Ok(data) => {
                            let d: serde_json::Value = from_value(data).unwrap();
                            return vec![
                                EvidenceMsg::UpdateEvidenceData(
                                    serde_json::to_string_pretty(&d).unwrap(),
                                ),
                                EvidenceMsg::HandleLoading(),
                            ];
                        }
                        Err(err) => {
                            let e: serde_json::Value = from_value(err).unwrap();
                            return vec![
                                EvidenceMsg::UpdateEvidenceData(
                                    serde_json::to_string_pretty(&e).unwrap(),
                                ),
                                EvidenceMsg::HandleLoading(),
                            ];
                        }
                    }
                });
                true
            }
            EvidenceMsg::UpdateEvidenceData(v) => {
                self.evidence_data = v;
                true
            }
            EvidenceMsg::HandleLoading() => {
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
            EvidenceMsg::UpdateTokenid(target.value())
        });
        let oninput2 = ctx.link().callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            EvidenceMsg::UpdateApikey(target.value())
        });
        let onclick = ctx
            .link()
            .batch_callback(|_| vec![EvidenceMsg::GetEvidenceData(), EvidenceMsg::HandleLoading()]);

        html! {
            <div class="p-3">
                <h5>{"evidence"}</h5>
                <div class="input-group mb-3">
                    <span class="input-group-text">{"Tokenid"}</span>
                    <input oninput={oninput1} type="text" class="form-control text-dark" placeholder="tokenid"/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text">{"Apikey"}</span>
                    <input oninput={oninput2} type="text" class="form-control text-dark" placeholder="apikey" />
                </div>
                <button {onclick} disabled={self.handle_loading_state}>
                    { self.handle_loading() }
                </button>
                <div class="form-floating">
                    <textarea class="form-control" style="height: 300px" value={self.evidence_data.clone() }/>
                </div>

            </div>
        }
    }
}

impl Evidence {
    fn handle_loading(&self) -> Html {
        match self.handle_loading_state {
            true => html! {
                <>
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                    <span class="sr-only">{"Loading..."}</span>
                </>
            },
            false => html! {
                { "Evidence" }
            },
        }
    }
}
