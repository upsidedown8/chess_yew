use chess::game::piece;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub piece: piece::Piece,
}

pub struct Piece;

impl Component for Piece {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use piece::Piece::*;

        let img = match ctx.props().piece {
            WhitePawn => "wp",
            WhiteKnight => "wn",
            WhiteBishop => "wb",
            WhiteRook => "wr",
            WhiteQueen => "wq",
            WhiteKing => "wk",

            BlackPawn => "bp",
            BlackKnight => "bn",
            BlackBishop => "bb",
            BlackRook => "br",
            BlackQueen => "bq",
            BlackKing => "bk",
        };

        html! {
            <svg />
        }
    }
}
