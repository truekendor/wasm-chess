use shakmaty::{Chess, Color, Position, fen::Fen, san::San, uci::UciMove};

use crate::{
    helpers::parsing::{CastleData, castle_data_from_san_move, is_two_square_pawn_move},
    tsify_structs::{MoveVerbose, SquareStr, others::ColorChar},
};

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
        .map(|internal_move| {
            let san_move = San::from_move(chess, *internal_move);
            let uci_move = UciMove::from_move(*internal_move, shakmaty::CastlingMode::Chess960);
            let current_fen = Fen::from_position(chess, shakmaty::EnPassantMode::Legal).to_string();

            let captured_piece: Option<String> =
                internal_move.capture().map(|val| val.char().to_string());

            let color_shorthand = match chess.turn() {
                Color::White => ColorChar::W,
                Color::Black => ColorChar::B,
            };

            let mut new_position = chess.clone();
            // we iterate through legal moves, so we know this is safe to do
            new_position.play_unchecked(*internal_move);

            let fen_after_move =
                Fen::from_position(&new_position, shakmaty::EnPassantMode::Legal).to_string();

            let from_sq = internal_move.from().expect(
                "Only standard chess and chess960 is supported, from() should always return Some",
            );

            let from = SquareStr::from_shakmaty_sq(&from_sq);
            let to = SquareStr::from_shakmaty_sq(&internal_move.to());

            let CastleData {
                is_castle,
                is_kingside_castle,
                is_queenside_castle,
            } = castle_data_from_san_move(&san_move);

            let is_big_pawn = is_two_square_pawn_move(&internal_move);

            MoveVerbose {
                from,
                to,
                promotion: internal_move.promotion().map(|p| p.char().to_string()),

                san: san_move.to_string(),
                lan: uci_move.to_string(),

                before: current_fen,
                after: fen_after_move,

                captured: captured_piece,
                color: color_shorthand,

                piece: internal_move.role().char().to_string(),

                is_big_pawn,
                is_en_passant: internal_move.is_en_passant(),

                is_castle,
                is_kingside_castle,
                is_queenside_castle,
            }
        })
        .collect();

    legal_moves
}
