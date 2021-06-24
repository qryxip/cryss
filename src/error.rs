//! エラー出力のためのモジュール

use crate::pos;

pub enum Error {
    UnexpectedCharacter(pos::Pos),
    NoCharacterAfterBackSlash(pos::Pos),
    UnterminatedComment(pos::Pos),
    UnterminatedStringLiteral(pos::Pos),
    NoLineFeedAtEOF,
    IncompleteScientificNotation(pos::Range),
    SingleAmpersand(pos::Range),
    SingleDot(pos::Range),
    ParseFloatError(pos::Range, std::num::ParseFloatError),
}

impl Error {
    pub fn print(&self, log: &Vec<String>) {
        print!("error: ");
        match self {
            Error::UnexpectedCharacter(pos) => {
                println!("unexpected character at {}", pos);
                pos.print(log);
            }
            Error::NoCharacterAfterBackSlash(pos) => {
                println!("no character after `\\` at {}", pos);
                pos.print(log);
            }
            Error::UnterminatedComment(pos) => {
                println!("unterminated comment (started at {})", pos);
                pos.print(log);
            }
            Error::UnterminatedStringLiteral(pos) => {
                println!("unterminated string literal (started at {})", pos);
                pos.print(log);
            }
            Error::NoLineFeedAtEOF => {
                println!("no line feed at end of file");
            }
            Error::IncompleteScientificNotation(range) => {
                println!("incomplete scientific notation at {}", range);
                range.print(log);
            }
            Error::SingleAmpersand(range) => {
                println!("single ampersand at {}", range);
                range.print(log);
            }
            Error::SingleDot(range) => {
                println!("single dot at {}", range);
                range.print(log);
            }
            Error::ParseFloatError(range, err) => {
                println!("failed to parse number at {} ({})", range, err);
                range.print(log);
            }
        }
    }
}
