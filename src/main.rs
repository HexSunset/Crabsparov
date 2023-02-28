use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {

    let mut rl = DefaultEditor::new()?;

    loop {
	let readline = rl.readline("FEN: ");
	match readline {
	    Ok(line) => {
		let board = libsparov::parse::parse_board(&line).unwrap().1;

		println!("{}", board);
	    },
	    Err(err) => {
		eprintln!("ERROR: {:?}", err);
		break;
	    }
	}
    }

    Ok(())
}
