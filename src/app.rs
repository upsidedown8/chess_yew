use chess::{
    engine::{eval::score, search::find_best_move},
    game::{
        board::{Board, GameState},
        movegen::MoveGenerator,
        moves::Move,
        square::Square,
        undo_info::UndoInfo,
    },
};
use yew::prelude::*;

const MAX_DEPTH: usize = 5;

pub enum Msg {
    SquareClick(Square),
}

#[derive(Default)]
pub struct App {
    board: Board,
    gen: MoveGenerator,
    moves: Vec<Move>,
    first: Option<Square>,
}

impl App {
    pub fn gen_moves(&mut self) {
        self.moves.clear();
        self.gen.gen_moves(&mut self.moves, &self.board);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut app = Self::default();
        app.gen_moves();
        app
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SquareClick(sq) => {
                log::info!("{} clicked", sq);

                match self.first {
                    None => {
                        self.first = Some(sq);
                    }
                    Some(first) => {
                        // if same, deselect both
                        if first == sq {
                            self.first = None;
                        } else {
                            // determine whether the move is valid
                            let mv = self
                                .moves
                                .iter()
                                .find(|m| m.start_sq() == first && m.end_sq() == sq);

                            if let Some(&mv) = mv {
                                // move has been found, make the move
                                let mut info = UndoInfo::default();
                                self.board.make_move(mv, &mut info);

                                let mut move_lists = (0..MAX_DEPTH).map(|_| vec![]).collect();

                                if let Some((best_mv, _)) = find_best_move(
                                    MAX_DEPTH,
                                    &mut self.board.clone(),
                                    &self.gen,
                                    &mut move_lists,
                                ) {
                                    let mut info = UndoInfo::default();
                                    self.board.make_move(best_mv, &mut info);
                                }

                                self.gen_moves();
                            }

                            self.first = None;
                        }
                    }
                };

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_state = self.board.game_state(&self.gen);
        let squares = self.board.pieces;
        let callback = ctx.link().callback(Msg::SquareClick);

        // find possible moves if start square is selected
        let mut highlighted = vec![];
        if let Some(sq) = self.first {
            let mut move_list = vec![];

            self.gen.gen_moves(&mut move_list, &self.board);

            move_list
                .iter()
                .filter(|&mv| mv.start_sq() == sq)
                .for_each(|mv| highlighted.push(mv.end_sq()));
        }

        html! {
            <>
                <p class="game-status">{match game_state {
                    GameState::Draw50 => String::from("50 move draw"),
                    GameState::Stalemate => String::from("Stalemate"),
                    GameState::ThreefoldRepetition => String::from("Draw by threefold repetition"),
                    GameState::Winner(color) => format!("{} has won!", color),
                    GameState::ToPlay(color) => format!("{} to play", color),
                }}</p>

                <crate::board::Board {highlighted} {squares} {callback} />

                <p class="score">{format!("score: {}", score(&self.board))}</p>
                <p class="zobrist">{format!("zobrist: {:x}", self.board.hash())}</p>
            </>
        }
    }
}
