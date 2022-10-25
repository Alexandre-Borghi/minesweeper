use std::ops::{Deref, Index};

use crate::cell::{self, Cell, Kind};
use yew::{
    function_component, html, use_state, use_state_eq, Callback, Html, Properties, UseStateHandle,
};

#[derive(Properties, PartialEq, Eq)]
pub struct GridProps {
    /// The size of one side of the grid. The grid will contain size * size
    /// cells.
    pub size: usize,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let size = props.size;
    let cells = use_state_eq(|| vec![Kind::Closed; size * size]);
    let mines = use_state_eq(|| vec![true; size * size]);

    let idx = move |i: usize, j: usize| -> usize { i * size + j };

    let onclick = {
        let cells = cells.clone();
        Callback::from(move |(row, col, mark): (usize, usize, bool)| {
            let index = idx(row, col);
            log::info!("Clicked ({row}, {col}) (index {index})");
            let mut new_cells = cells.deref().clone();
            new_cells[index] = cell::Kind::Opened(mine_neighbors_count(&mines, size, row, col));
            cells.set(new_cells.clone());
        })
    };

    html! {
        <table class="ms-grid">
            {
            (0..size).into_iter().map(|i| {
                html! {
                    <tr>
                    {
                    (0..size).into_iter().map(|j| {
                        let cells = cells.clone();
                        let kind = cells[idx(i, j)].clone();
                        html! {<td><Cell onclick={onclick.clone()} row={i} col={j} {kind} /></td>}
                    }).collect::<Html>()
                    }
                    </tr>
                }
            }).collect::<Html>()
            }
        </table>
    }
}

fn mine_neighbors_count(mines: &[bool], size: usize, row: usize, col: usize) -> u32 {
    let mut count = 0;
    for i in row.saturating_sub(1)..=(row + 1).clamp(0, size - 1) {
        for j in col.saturating_sub(1)..=(col + 1).clamp(0, size - 1) {
            if (i, j) != (row, col) {
                count += mines[i * size + j] as u32;
            }
        }
    }
    count
}
