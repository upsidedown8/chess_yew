use chess::{
    engine::{eval::score, perft::perft, search::iterative_deepening},
    game::{
        board::Board,
        movegen::MoveGenerator,
        moves::{Move, MoveType},
        square::Square,
        undo_info::UndoInfo,
    },
};
use yew::prelude::*;

pub enum Msg {
    Reset,
}

pub struct App {
    board: Board,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            board: Board::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>

                <h1>{ "Chess" }</h1>

            </>
        }
    }
}
