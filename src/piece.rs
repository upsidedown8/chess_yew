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

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        use piece::Piece::*;

        html! {
            <div class="piece">{match ctx.props().piece {
                WhitePawn => '♙',
                WhiteKnight => '♘',
                WhiteBishop => '♗',
                WhiteRook => '♖',
                WhiteQueen => '♕',
                WhiteKing => '♔',

                BlackPawn => '♟',
                BlackKnight => '♞',
                BlackBishop => '♝',
                BlackRook => '♜',
                BlackQueen => '♛',
                BlackKing => '♚',
            }}</div>
        }
    }
}
