use chess::game::{color::Color, piece::Piece};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub piece: Option<Piece>,
    pub tile_color: Color,
    pub is_highlighted: bool,
}

pub struct Square;

impl Component for Square {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button
                class={format!("square {} {}", match ctx.props().tile_color {
                    Color::White => "square-white",
                    Color::Black => "square-black",
                }, if ctx.props().is_highlighted {
                    "square-highlighted"
                } else {
                    ""
                })}
                onclick={ctx.props().onclick.clone()}
            >
                if let Some(piece) = ctx.props().piece {
                    <crate::piece::Piece {piece} />
                }
            </button>
        }
    }
}
