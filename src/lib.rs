pub mod parse;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Piece {
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
}

impl TryFrom<char> for Piece {
    type Error = &'static str;
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
	match value {
	    'p' => Ok(Piece::BPawn),
	    'P' => Ok(Piece::WPawn),
	    'n' => Ok(Piece::BKnight),
	    'N' => Ok(Piece::WKnight),
	    'b' => Ok(Piece::BBishop),
	    'B' => Ok(Piece::WBishop),
	    'r' => Ok(Piece::BRook),
	    'R' => Ok(Piece::WRook),
	    'q' => Ok(Piece::BQueen),
	    'Q' => Ok(Piece::WQueen),
	    'k' => Ok(Piece::BKing),
	    'K' => Ok(Piece::WKing),

	    _ => Err("Invalid character: {value}"),
	}
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
	match self {
	    Piece::WPawn => 'P',
	    Piece::WKnight => 'N',
	    Piece::WBishop => 'B',
	    Piece::WRook => 'R',
	    Piece::WQueen => 'Q',
	    Piece::WKing => 'K',
	    Piece::BPawn => 'p',
	    Piece::BKnight => 'n',
	    Piece::BBishop => 'b',
	    Piece::BRook => 'r',
	    Piece::BQueen => 'q',
	    Piece::BKing => 'k',
	}
    }
}

#[derive(Debug)]
pub struct Board([Option<Piece>; 64]);

impl Board {
    pub fn new() -> Self {
        Board([None; 64])
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	let mut output = String::new();

	for i in (0..8).rev() {
	    for j in 0..8 {
		let p = self.0[i * 8 + j];
		match p {
		    Some(piece) => output.push(piece.into()),
		    None => output.push('•'),
		}
	    }
	    output.push('\n');
	}
	write!(f, "{}", output)
    }
}

//TODO: Add enum for state of game, ex: checkmate, check, stalemate, invalid position etc.
pub struct ChessState {
    pub turn: u32,
    pub side: Color,
    pub board: Board,
}
