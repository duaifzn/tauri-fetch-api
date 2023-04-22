mod component;
mod route;
mod dto;
use crate::component::app::App;
use yew;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}