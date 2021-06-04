// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
// TODO: Remove
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod boardstate;
use boardstate::{BoardState, Turn};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board: BoardState::new(4, false),
        with_hint: false,
        show_error_msg: false,
        show_result: false,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    board: BoardState,
    with_hint: bool,
    show_error_msg: bool,
    show_result: bool,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Put(usize, usize),
    PutRandomly,
    ShowHint,
    EndGame,
    NewGame,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Put(i, j) => {
            model.with_hint = false;
            model.show_error_msg = false;
            let puttable = model.board.cnt_reversable();
            if puttable[i][j] == 0 {
                model.show_error_msg = true;
                return;
            }
            // update the cell
            let can_continue = model.board.put(i, j);

            if !can_continue {
                model.show_result = true;
            }
        }
        Msg::PutRandomly => {
            model.with_hint = false;
            model.show_error_msg = false;
            let mut rng = thread_rng();

            let mut options: Vec<(usize, usize)> = Vec::new();
            let mut options_corners: Vec<(usize, usize)> = Vec::new();
            let vec = &model.board.cnt_reversable();
            let n = model.board.get_size();
            for i in 0..n {
                for j in 0..n {
                    if vec[i][j] > 0 {
                        for _ in 0..vec[i][j] {
                            options.push((i, j));
                        }
                        if (i == 0 || i == n - 1) && (j == 0 || j == n - 1) {
                            options_corners.push((i, j));
                        }
                    }
                }
            }

            // choose randomly
            let &(i, j) = if options_corners.is_empty() {
                options
            } else {
                options_corners
            }
            .choose(&mut rng)
            .unwrap();

            // update the cell
            let can_continue = model.board.put(i, j);

            if !can_continue {
                model.show_result = true;
            }
        }
        Msg::ShowHint => {
            model.with_hint = true;
            model.show_error_msg = false;
        }
        Msg::EndGame => {
            model.with_hint = false;
            model.show_error_msg = false;
            model.show_result = true;
        }
        Msg::NewGame => {
            model.with_hint = false;
            model.show_error_msg = false;
            model.show_result = false;
            model.board = BoardState::new(4, false);
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["wrapper", "main-area"],
        view_menu(&model.board, model.show_error_msg),
        view_board(&model.board, model.with_hint),
        IF!(model.show_result => view_result(&model.board)),
    ]
}

fn view_menu(board: &BoardState, show_error_msg: bool) -> Vec<Node<Msg>> {
    nodes![
        div![
            C!["menu-area"],
            div![
                C!["menu"],
                "新しいゲーム",
                ev(Ev::Click, move |_| Msg::NewGame)
            ],
            div![
                C!["menu"],
                "ヒントを表示",
                ev(Ev::Click, move |_| Msg::ShowHint)
            ],
            div![
                C!["menu"],
                "ランダムに置く",
                ev(Ev::Click, move |_| Msg::PutRandomly)
            ],
            div![
                C!["menu"],
                "ゲームを終了",
                ev(Ev::Click, move |_| Msg::EndGame)
            ],
        ],
        div![
            C!["annotation"],
            div![C!["which-turn"], preview_turn(&board),],
            IF!(show_error_msg =>
                div![
                    C!["error-msg"],
                    "そこには置けません",
                ]
            )
        ]
    ]
}

// Show which turn now
fn preview_turn(bs: &BoardState) -> String {
    format!("{}のターン", bs.which_turn())
}

fn view_board(board: &BoardState, with_hint: bool) -> Node<Msg> {
    div![
        C!["board"],
        if with_hint {
            view_cells_with_hint(&board)
        } else {
            view_cells_without_hint(&board)
        },
    ]
}

fn view_cells_with_hint(board: &BoardState) -> Vec<Node<Msg>> {
    let mut vec: Vec<Node<Msg>> = Vec::new();
    let puttable: Vec<Vec<usize>> = board.cnt_reversable();
    for i in 0..8 {
        for j in 0..8 {
            vec.push(div![
                C!["cell"],
                match board.state[i][j] {
                    Some(Turn::White) => "○",
                    Some(Turn::Black) => "●",
                    None =>
                        if puttable[i][j] > 0 {
                            "・"
                        } else {
                            ""
                        },
                },
                ev(Ev::Click, move |_| Msg::Put(i, j))
            ])
        }
    }
    vec
}

fn view_cells_without_hint(board: &BoardState) -> Vec<Node<Msg>> {
    let mut vec: Vec<Node<Msg>> = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            vec.push(div![
                C!["cell"],
                match board.state[i][j] {
                    Some(Turn::White) => "○",
                    Some(Turn::Black) => "●",
                    None => "",
                },
                ev(Ev::Click, move |_| Msg::Put(i, j))
            ])
        }
    }
    vec
}

fn view_result(board: &BoardState) -> Node<Msg> {
    div![C!["result"], "結果：", show_result(&board),]
}

// Show result
fn show_result(bs: &BoardState) -> String {
    let ((c1, s1), (c2, s2)) = bs.count_pieces();
    if s1 > s2 {
        format!("{0}が{1}個，{2}が{3}個で{0}の勝ち！", c1, s1, c2, s2)
    } else if s1 < s2 {
        format!("{0}が{1}個，{2}が{3}個で{2}の勝ち！", c1, s1, c2, s2)
    } else {
        format!("{0}が{1}個，{2}が{3}個で引き分け！", c1, s1, c2, s2)
    }
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_class_name("reversiapp")
        .item(0)
        .expect("element with the class `reversiapp`");

    App::start(root_element, init, update, view);
}
