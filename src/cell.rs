use yew::{function_component, html, Callback, Properties};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub row: usize,
    pub col: usize,
    pub kind: Kind,
    pub onclick: Callback<(usize, usize, bool)>,
}

#[function_component(Cell)]
pub fn cell(props: &Props) -> Html {
    let Props {
        row,
        col,
        kind,
        onclick,
    } = props.clone();

    let label = match kind {
        Kind::Closed => format!("({row}, {col})"),
        Kind::Opened(n) => format!("{n}"),
        Kind::Marked => String::from("🚩"),
    };

    let class = match kind {
        Kind::Closed => "closed",
        Kind::Opened(_) => "opened",
        Kind::Marked => "marked",
    };

    let onclick = Callback::from(move |_| onclick.emit((row, col, false)));

    html! {
        <button {class} {onclick} type="button">{ label }</button>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Closed,
    Opened(u32),
    Marked,
}
