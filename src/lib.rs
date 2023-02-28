mod parse;

use nom::error::ParseError;

#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
}

use derive_try_from_primitive::TryFromPrimitive;
#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
enum Piece {
    WPawn = 'P' as u8,
    WKnight = 'N' as u8,
    WBishop = 'B' as u8,
    WRook = 'R' as u8,
    WQueen = 'Q' as u8,
    WKing = 'K' as u8,

    BPawn = 'p' as u8,
    BKnight = 'n' as u8,
    BBishop = 'b' as u8,
    BRook = 'r' as u8,
    BQueen = 'q' as u8,
    BKing = 'k' as u8,
}

struct Board([Option<Piece>; 64]);

impl Board {
    pub fn new() -> Self {
        Board([None; 64])
    }

    pub fn parse_row(i: parse::Input) -> Result<Vec<Option<Piece>>, nom::error::Error<parse::Input>> {
        use nom::bytes::complete::is_a;
	use nom::error::context;

        let (i, pieces) = is_a("pPnNbBrRqQkK12345678")(i).unwrap();

	if i.len() > 0 {
	    panic!("Error in FEN, row contains invalid characters: {i}");
	}
	
	if pieces.len() > 8 {
	    panic!("Error in FEN, piece row too long: {pieces}");
	}

        let mut row = Vec::new();
	let mut index: usize = 0;

        for c in pieces.chars() {
	    if index > 7 {
		panic!("Error in FEN, row has too many elements");
	    }

	    if let Some(num) = c.to_digit(10) {
		if !((1..=8).contains(&num)) {
		    panic!("Error in FEN, row can only have [0..8] empty spaces");
		}

		for _ in 0..num {
		    row.push(None);
		}
		
		index += num as usize;
		continue;
	    }
	    
	    row.push(Some(Piece::try_from(c as u8).unwrap()));
	    index += 1;
	    continue;
        }

        Ok(row)
    }

    fn parse(i: parse::Input) -> parse::Output<Self> {
        use nom::bytes::complete::{is_a, is_not, take_till};
        use nom::character::{complete::char, is_space};
        use nom::multi::many_till;

        let mut board = Self::new();
        let mut index: usize = 1;

	let pieces: &str;
	
        (i, pieces) = take_till(|c| c == ' ')(i).expect("No piece information provided");

	let mut index: usize = 0;
	
        for line in pieces.split('/') {
	    let row = Self::parse_row(line).expect("Row parsing failed");

	    for p in row.iter().rev() {
		if p.is_some() {
		    board.0[index] = *p;
		}

		index += 1;
	    }
	}

        todo!()
    }
}

//TODO: Add enum for state of game, ex: checkmate, check, stalemate, invalid position etc.
struct ChessState {
    turn: u32,
    side: Color,
    board: Board,
}

impl ChessState {
    fn parse(input: parse::Input) -> parse::Output<Self> {
        todo!()
    }
}
