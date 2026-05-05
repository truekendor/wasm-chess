use shakmaty::{Chess, Position, san::San, uci::UciMove};

use crate::{helpers::parsing::verbose_move_object_from_raw_move, tsify_structs::MoveVerbose};

pub fn uci(chess: &Chess) -> Vec<String> {
    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .map(|el| {
            let uci_move = UciMove::from_move(*el, shakmaty::CastlingMode::Chess960);
            return uci_move.to_string();
        })
        .collect();

    legal_moves
}

pub fn san(chess: &Chess) -> Vec<String> {
    let legal_moves: Vec<String> = chess
        .legal_moves()
        .iter()
        .map(|el| {
            let san_move = San::from_move(chess, *el);

            return san_move.to_string();
        })
        .collect();

    legal_moves
}

pub fn verbose(chess: &Chess) -> Vec<MoveVerbose> {
    let legal_moves: Vec<MoveVerbose> = chess
        .legal_moves()
        .iter()
        .map(|raw_move| {
            let move_verbose = verbose_move_object_from_raw_move(*raw_move, chess);

            move_verbose
        })
        .collect();

    legal_moves
}
