use std::ops::ControlFlow::{self};

use pgn_reader::{RawTag, SanPlus, Visitor};

use shakmaty::fen::Fen;

use crate::{
    WasmChess, impls::PGNResult, models::utils::PreserveHeaders, utils::pos_from_fen_with_recovery,
};

static SUFFIX_LIST: [&str; 6] = ["!", "?", "!!", "??", "!?", "?!"];

impl Visitor for WasmChess {
    type Tags = PGNResult;
    type Movetext = PGNResult;
    type Output = Result<(), String>;

    fn begin_tags(&mut self) -> ControlFlow<Self::Output, Self::Tags> {
        let pgn_result = PGNResult::default();

        ControlFlow::Continue(pgn_result)
    }

    fn tag(
        &mut self,
        tags: &mut Self::Tags,
        name: &[u8],
        value: RawTag<'_>,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = tags;

        let tag_key: String = name.iter().map(|b| *b as char).collect();
        let tag_val = str::from_utf8(value.as_bytes());

        let Ok(tag_val) = tag_val else {
            return ControlFlow::Break(Err(format!(
                "Error reading tag value\nTag Key: {}",
                tag_key
            )));
        };

        if name.to_ascii_uppercase() == b"FEN" {
            let fen = match Fen::from_ascii(tag_val.as_bytes()) {
                Ok(fen) => fen,
                Err(err) => {
                    return ControlFlow::Break(Err(format!(
                        "Error parsing fen from PGN: {}\nFEN: {}",
                        err, tag_val
                    )));
                }
            };

            let chess_pos = pos_from_fen_with_recovery(&fen);

            match chess_pos {
                Ok(chess) => {
                    pgn_result.starting_fen =
                        Fen::from_position(&chess, shakmaty::EnPassantMode::Legal);
                }
                Err(err) => {
                    return ControlFlow::Break(Err(format!(
                        "Position error: {}\nFEN: {}",
                        err, tag_val
                    )));
                }
            }
        };

        pgn_result.headers.insert(tag_key, tag_val.to_string());

        return ControlFlow::Continue(());
    }

    fn begin_movetext(&mut self, tags: Self::Tags) -> ControlFlow<Self::Output, Self::Movetext> {
        let mut pgn_result = tags;
        pgn_result.reorder_headers();

        let starting_fen_str = &pgn_result.starting_fen.to_string();

        match self.load_inner(
            &starting_fen_str,
            Some(PreserveHeaders {
                preserve_headers: true,
            }),
        ) {
            Ok(_) => (),
            Err(err) => {
                return ControlFlow::Break(Err(format!("{}", err)));
            }
        }

        ControlFlow::Continue(pgn_result)
    }

    fn san(
        &mut self,
        _movetext: &mut Self::Movetext,
        san_plus: SanPlus,
    ) -> ControlFlow<Self::Output> {
        match self.play_move(&san_plus.san.to_string()) {
            Ok(_) => {
                return ControlFlow::Continue(());
            }
            Err(err) => ControlFlow::Break(Err(format!("{}", err))),
        }
    }

    fn nag(
        &mut self,
        movetext: &mut Self::Movetext,
        nag: pgn_reader::Nag,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = movetext;

        let nag = nag.to_string();

        let fen_key = Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal).to_string();

        let nag_str = nag.as_str();
        match nag_str {
            "$1" | "$2" | "$3" | "$4" | "$5" | "$6" => {
                let number_part = &nag_str[1..];

                let number = number_part.parse::<u32>();

                if let Ok(nag_number) = number {
                    let suffix_number = nag_number - 1;

                    if suffix_number >= SUFFIX_LIST.len() as u32 {
                        return ControlFlow::Continue(());
                    }

                    let char = SUFFIX_LIST[suffix_number as usize];

                    pgn_result.suffix_map.insert(fen_key, char.to_owned());
                }

                return ControlFlow::Continue(());
            }
            _ => (),
        }

        pgn_result
            .nag_map
            .entry(fen_key)
            .or_insert(Vec::new())
            .push(nag);

        ControlFlow::Continue(())
    }

    fn partial_comment(
        &mut self,
        movetext: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = movetext;

        let raw_comment = comment;

        let comment = str::from_utf8(&raw_comment.as_bytes());

        if let Ok(val) = comment {
            let fen_key =
                Fen::from_position(&self.chess, shakmaty::EnPassantMode::Legal).to_string();

            pgn_result
                .comments_map
                .entry(fen_key)
                .and_modify(|existing| existing.push_str(&val.to_string()))
                .or_insert_with(|| val.to_string());

            return ControlFlow::Continue(());
        }

        ControlFlow::Break(Err(format!(
            "Error parsing comment from PGN: {:?}",
            raw_comment
        )))
    }

    fn comment(
        &mut self,
        movetext: &mut Self::Movetext,
        comment: pgn_reader::RawComment<'_>,
    ) -> ControlFlow<Self::Output> {
        self.partial_comment(movetext, comment)
    }

    fn outcome(
        &mut self,
        movetext: &mut Self::Movetext,
        outcome: shakmaty::Outcome,
    ) -> ControlFlow<Self::Output> {
        let pgn_result = movetext;

        match outcome {
            shakmaty::Outcome::Known(known_outcome) => {
                pgn_result.known_outcome = Some(known_outcome);
                ControlFlow::Continue(())
            }
            shakmaty::Outcome::Unknown => {
                pgn_result.known_outcome = None;
                ControlFlow::Continue(())
            }
        }
    }

    fn end_game(&mut self, movetext: Self::Movetext) -> Self::Output {
        self.pgn_result = Some(movetext);
        return Ok(());
    }
}
