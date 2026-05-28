use std::io::{self};

use pgn_reader::Reader;

use ordermap::OrderMap;
use shakmaty::fen::Fen;

use crate::WasmChess;

use super::*;

const SEVEN_TAG_ROSTER: [&str; 7] = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];
const SUPPLEMENTAL_TAGS: [(&str, Option<&str>); 30] = [
    ("WhiteTitle", None),
    ("BlackTitle", None),
    ("WhiteElo", None),
    ("BlackElo", None),
    ("WhiteUSCF", None),
    ("BlackUSCF", None),
    ("WhiteNA", None),
    ("BlackNA", None),
    ("WhiteType", None),
    ("BlackType", None),
    ("EventDate", None),
    ("EventSponsor", None),
    ("Section", None),
    ("Stage", None),
    ("Board", None),
    ("Opening", None),
    ("Variation", None),
    ("SubVariation", None),
    ("ECO", None),
    ("NIC", None),
    ("Time", None),
    ("UTCTime", None),
    ("UTCDate", None),
    ("TimeControl", None),
    ("SetUp", None),
    ("FEN", None),
    ("Termination", None),
    ("Annotator", None),
    ("Mode", None),
    ("PlyCount", None),
];

#[derive(Debug, Default, Clone)]
pub struct PGNResult {
    pub headers: OrderMap<String, String>,
    pub starting_fen: Fen,

    pub comments_map: OrderMap<String, String>,
    pub suffix_map: OrderMap<String, String>,
    pub nag_map: OrderMap<String, Vec<String>>,

    pub known_outcome: Option<KnownOutcome>,
}

#[wasm_bindgen]
impl WasmChess {
    #[wasm_bindgen(js_name = "loadPgn")]
    pub fn load_pgn(&mut self, pgn: &str) -> Result<(), String> {
        self.reset();
        let mut reader = Reader::new(io::Cursor::new(pgn));

        reader
            .read_game(self)
            .map_err(|err| err.to_string())?
            .unwrap_or(Ok(()))
    }

    #[wasm_bindgen(js_name = "pgn")]
    pub fn pgn(&mut self, options: Option<PGNOptions>) -> String {
        let options = options.unwrap_or_else(|| PGNOptions {
            max_width: None,
            newline: None,
        });

        let newline = options.newline.unwrap_or("\r\n".to_string());
        let newline_char = newline.as_str();
        let max_line_width = options.max_width.unwrap_or(0);

        let mut move_chunks: Vec<String> = Vec::new();
        let mut header_string = String::new();

        let initial_fen = if self.history.is_empty() {
            self.fen(None)
        } else {
            self.history[0].fen_before.to_string()
        };

        let headers = self.get_headers().0;

        let pgn_result = self.pgn_result.get_or_insert_with(PGNResult::default);

        for (key, value) in headers {
            header_string.push_str(&format!("[{key} \"{value}\"]{newline_char}"));
        }

        if !pgn_result.headers.is_empty() {
            header_string.push_str(newline_char);
        }

        if let Some(comment) = pgn_result.comments_map.get(&initial_fen) {
            move_chunks.push(format!("{{{comment}}}"));
        }

        for (index, history_entry) in self.history.iter().enumerate() {
            let move_number = history_entry.position_before.fullmoves();

            let san = San::from_move(&history_entry.position_before, history_entry.raw_move);
            let san_plus = san_to_san_plus(&san, &history_entry.position_after);
            let mut move_text = san_plus;

            let prefix = if index == 0 {
                match history_entry.turn {
                    shakmaty::Color::White => format!("{move_number}."),
                    shakmaty::Color::Black => format!("{move_number}. ..."),
                }
            } else {
                match history_entry.turn {
                    shakmaty::Color::White => format!("{move_number}."),
                    shakmaty::Color::Black => String::new(),
                }
            };

            if prefix.is_empty() {
                if let Some(last) = move_chunks.last_mut() {
                    last.push(' ');
                    last.push_str(&move_text);
                }
            } else {
                move_chunks.push(format!("{prefix} {move_text}"));
            }

            let fen_after = history_entry.fen_after.to_string();

            if history_entry.position_after.is_checkmate() {
                move_text.push_str("#");
            } else if history_entry.position_after.is_check() {
                move_text.push_str("+");
            }

            // NAGs
            if let Some(nags) = pgn_result.nag_map.get(&fen_after) {
                for nag in nags {
                    move_text.push(' ');
                    move_text.push_str(nag);
                }
            }

            // Suffix annotation
            if let Some(suffix) = pgn_result.suffix_map.get(&fen_after) {
                move_text.push(' ');
                move_text.push_str(suffix);
            }

            if let Some(comment) = pgn_result.comments_map.get(&fen_after) {
                let comment = comment.split_whitespace().collect::<Vec<_>>().join(" ");
                move_chunks.push(format!("{{{comment}}}"));
            }
        }

        if let Some(pgn_result) = self.pgn_result.as_ref() {
            let result = match pgn_result.known_outcome {
                Some(shakmaty::KnownOutcome::Decisive { winner }) => match winner {
                    shakmaty::Color::White => "1-0",
                    shakmaty::Color::Black => "0-1",
                },
                Some(shakmaty::KnownOutcome::Draw) => "1/2-1/2",
                None => pgn_result
                    .headers
                    .get("Result")
                    .map(|s| s.as_str())
                    .unwrap_or("*"),
            };

            move_chunks.push(result.to_string());
        }

        // No wrapping
        if max_line_width == 0 {
            return format!("{header_string}{}", move_chunks.join(" "))
                .trim()
                .to_string();
        }

        let mut result_string = header_string;
        let mut current_line_width = 0;

        for token in move_chunks {
            let token_len = token.chars().count();

            let required_width = if current_line_width == 0 {
                token_len
            } else {
                1 + token_len
            };

            if current_line_width > 0 && ((current_line_width + required_width) > max_line_width) {
                result_string.push_str(newline_char);
                result_string.push_str(&token);
                current_line_width = token_len;
            } else {
                if current_line_width > 0 {
                    result_string.push(' ');
                    current_line_width += 1;
                }

                result_string.push_str(&token);
                current_line_width += token_len;
            }
        }

        result_string
    }
}

impl PGNResult {
    pub(crate) fn reorder_headers(&mut self) {
        let mut ordered: OrderMap<String, Option<String>> = OrderMap::new();

        // Seven tag roster first
        for key in SEVEN_TAG_ROSTER {
            ordered.insert(key.to_string(), self.headers.get(key).cloned());
        }

        // Supplemental tags second
        for (key, _) in SUPPLEMENTAL_TAGS {
            ordered.insert(key.to_string(), self.headers.get(key).cloned());
        }

        // Remaining custom tags last
        for (key, val) in self.headers.iter() {
            if !ordered.contains_key(key) {
                ordered.insert(key.clone(), Some(val.clone()));
            }
        }

        self.headers = ordered
            .into_iter()
            .filter_map(|(k, v)| v.map(|v| (k, v)))
            .collect();
    }
}
