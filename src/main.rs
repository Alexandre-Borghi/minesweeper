use grid::Grid;
use yew::{function_component, html};

mod cell;
mod grid;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Grid size=8 />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
