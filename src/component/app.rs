use yew::{ Component, Html, html, Context };
use crate::component::evidence::Evidence;
use crate::component::verification::Verification;

pub struct App;

pub enum AppMsg{

}

impl Component for App{
    type Message = AppMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>             
                    <div class="mytabs">
                    <input type="radio" id="tabfree" name="mytabs" checked={true} />
                    <label for="tabfree">{"Evidence"}</label>
                    <div class="tab">
                        <Evidence />
                    </div>
                    <input type="radio" id="tabsilver" name="mytabs" />
                    <label for="tabsilver">{"Verification"}</label>
                    <div class="tab">
                      <Verification />
                    </div>
                  </div>
            </div>
        }
    }

}