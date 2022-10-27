use yew::{function_component, html, Callback, MouseEvent, Properties};

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
        Kind::Opened(false, n) => format!("{n}"),
        Kind::Opened(true, _) => "💣".to_string(),
        Kind::Marked => "🚩".to_string(),
    };

    let class = match kind {
        Kind::Closed => "closed",
        Kind::Opened(false, _) => "opened not-mine",
        Kind::Opened(true, _) => "opened mine",
        Kind::Marked => "marked",
    };

    let onleftclick = {
        let onclick = onclick.clone();
        Callback::from(move |_| {
            onclick.emit((row, col, false));
        })
    };

    let onrightclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        onclick.clone().emit((row, col, true));
    });

    html! {
        <button {class} onclick={onleftclick} oncontextmenu={onrightclick} type="button">{ label }</button>
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Closed,
    Opened(bool, u32),
    Marked,
}
