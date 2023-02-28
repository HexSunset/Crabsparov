use super::{Color, Piece, Board, ChessState};

use nom::error::context;

pub type Input<'a> = &'a str;
pub type Output<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

fn parse_piece(i: Input) -> Output<Piece> {
    use nom::character::complete::one_of;

    let (i, piece) = context("Parsing one piece", one_of("pPnNbBrRqQkK"))(i)?;
    Ok((i, Piece::try_from(piece).expect("If this fails something is wrong with nom")))
}

fn parse_empty_spots(i: Input) -> Output<usize> {
    use nom::bytes::complete::is_a;

    let (i, count) = context("Number of subsequent empty squares", is_a("12345678"))(i)?;
    
    Ok((i, count.parse().unwrap()))
}

fn parse_row(i: Input) -> Output<[Option<Piece>; 8]> {
    use nom::combinator::fail;

    let mut index: usize = 0;

    let mut i = i;
    let mut pieces = [None; 8];

    while index < 8 {
	if let Ok((remainder, p)) = parse_piece(i) {
	    i = remainder;

	    pieces[index] = Some(p);
	    index += 1;
	} else if let Ok((remainder, count)) = parse_empty_spots(i) {
	    i = remainder;

	    index += count;
	} else {
	    return context("Not enough square identifiers in row", fail)(i);
	}
    }

    if index != 8 {
	return context("FEN row has to have 8 pieces, got: {index}", fail)(i);
    }

    Ok((i, pieces))
}

fn parse_color(i: Input) -> Output<Color> {
    use nom::character::complete::one_of;

    let (i, color) = context("Color is either w or b", one_of("wb"))(i)?;

    let c = match color {
	'w' => Color::White,
	'b' => Color::Black,
	
	_ => unreachable!(),
    };

    Ok((i, c))
}

fn parse_board(input: Input) -> Output<Board> {
    use nom::bytes::complete::tag;

    let mut board = Board::new();

    let mut i = input;

    for r in (0..8).rev() {
	let (remainder, row) = parse_row(i)?;
	i = remainder;
	
	if r > 0 {
	    let (remainder, _) = tag("/")(i)?;
	    i = remainder;
	}
	
	for c in 0..8 {
	    board.0[r * 8 + c] = row[c];
	}
    }

    Ok((i, board))
}

pub fn parse_fen(i: Input) -> Output<ChessState> {
    let (i, board) = parse_board(i)?;
    let (i, side) = parse_color(i)?;
    

    Ok((
	i,    
	ChessState {
	    turn: todo!(),
	    side,
	    board,
	}
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use Piece::*;

    #[test]
    fn test_parse() {
	// parse_empty_piece
	assert_eq!(parse_piece("p"), Ok(("", Piece::BPawn)));
	assert_eq!(parse_piece("pP"), Ok(("P", Piece::BPawn)));
	assert!(parse_piece("").is_err());

	// parse_empty_spots
	assert_eq!(parse_empty_spots("1pP"), Ok(("pP", 1)));
	assert_eq!(parse_empty_spots("2"), Ok(("", 2)));

	// parse_row
	let expected = [None; 8];
	assert_eq!(parse_row("8"), Ok(("", expected)));

	let expected = [Some(BRook), Some(BKnight), Some(BBishop), Some(BQueen), Some(BKing), Some(BBishop), Some(BKnight), Some(BRook)];
	assert_eq!(parse_row("rnbqkbnr"), Ok(("", expected)));

	assert!(parse_row("abcdefg").is_err());

	// parse_board
	assert!(parse_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_ok());
    }
}
