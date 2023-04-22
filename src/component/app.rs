use yew::{ Component, Html, html, Context, classes };
use crate::route::page::Page;
use crate::component::evidence::Evidence;
use crate::component::verification::Verification;

pub struct App{
    page: Page
}

pub enum AppMsg{
    SwitchPage(Page)
}

impl Component for App{
    type Message = AppMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        Self{
            page: Page::Evidence
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SwitchPage(Page::Evidence) => {
                self.page = Page::Evidence;
            },
            AppMsg::SwitchPage(Page::Verification) => {
                self.page = Page::Verification;
            },
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick1 = ctx.link().callback(|_| AppMsg::SwitchPage(Page::Evidence));
        let onclick2 = ctx.link().callback(|_| AppMsg::SwitchPage(Page::Verification));
        html! {
            <div>
                <div class="navbar-dark bg-dark">
                    <ul class="nav nav-tabs">
                        <li class="nav-item">
                            <a class="nav-link text-white" href="#" onclick={onclick1}>{ "Evidence" }</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link text-white" href="#" onclick={onclick2}>{ "Verification" }</a>
                        </li>
                    </ul>
                </div>
                <div>
                    {self.switch_page()}
                </div> 
            </div>
        }
    }

}

impl App{
    fn switch_page(&self) -> Html{
        match self.page{
            Page::Evidence => html!{ <Evidence /> },
            Page::Verification => html!{ <Verification /> },
        }
    }
}

