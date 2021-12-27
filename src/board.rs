use crate::square;
use chess::game::{color::Color, piece::Piece, square::Square};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub squares: [Option<Piece>; 64],
    pub callback: Callback<Square>,
    pub highlighted: Vec<Square>,
}

pub struct Board;

impl Component for Board {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut pos = 0_usize;

        html! {
            <div class="board">
            {for ctx.props().squares
                .chunks(8)
                .map(|row| html! {
                    <div class="board-row">
                        {for row
                            .iter()
                            .copied()
                            .map(|piece| {
                                let sq = Square::from(pos);

                                let onclick = {
                                    let callback = ctx.props().callback.clone();
                                    Callback::from(move |_| callback.emit(sq))
                                };


                                let is_highlighted = ctx
                                    .props()
                                    .highlighted
                                    .iter()
                                    .any(|x| *x == sq);

                                let tile_color = if (pos % 2 == 0) ^ (pos % 16 < 8) {
                                    Color::Black
                                } else {
                                    Color::White
                                };

                                pos += 1;

                                html! {
                                    <square::Square
                                        {onclick}
                                        {piece}
                                        {is_highlighted}
                                        {tile_color}
                                    />
                                }
                            })}
                    </div>
                })}
            </div>
        }
    }
}
