use serde::{Deserialize, Serialize};

#[derive(tsify::Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub enum SuffixSymbol {
    Exclam,
    Question,
    DoubleExclam,
    ExclamQuestion,
    QuestionExclam,
    DoubleQuestion,
}

impl SuffixSymbol {
    // fn as_str(&self) -> &'static str {
    //     match self {
    //         SuffixSymbol::Exclam => "!",
    //         SuffixSymbol::Question => "?",
    //         SuffixSymbol::DoubleExclam => "!!",
    //         SuffixSymbol::ExclamQuestion => "!?",
    //         SuffixSymbol::QuestionExclam => "?!",
    //         SuffixSymbol::DoubleQuestion => "??",
    //     }
    // }

    pub fn from_str(str: &str) -> Option<SuffixSymbol> {
        match str {
            "!" | "$1" => Some(SuffixSymbol::Exclam),
            "?" | "$2" => Some(SuffixSymbol::Question),
            "!!" | "$3" => Some(SuffixSymbol::DoubleExclam),
            "??" | "$4" => Some(SuffixSymbol::DoubleQuestion),
            "!?" | "$5" => Some(SuffixSymbol::ExclamQuestion),
            "?!" | "$6" => Some(SuffixSymbol::QuestionExclam),
            _ => None,
        }
    }

    pub fn str_is_valid_suffix(str: &str) -> bool {
        match str {
            "!" | "$1" | "?" | "$2" | "!!" | "$3" | "??" | "$4" | "!?" | "$5" | "?!" | "$6" => true,
            _ => false,
        }
    }
}
