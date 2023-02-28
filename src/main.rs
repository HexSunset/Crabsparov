use rustyline::{DefaultEditor, Result};
use rustyline::config::Config;

fn main() -> Result<()> {

    let mut rl = DefaultEditor::new()?;

    loop {
	let readline = rl.readline("FEN: ");
	match readline {
	    Ok(line) => {
		let position = libsparov::parse::parse_fen(&line).unwrap().1;

		println!("{}", position.board);
	    },
	    Err(err) => {
		eprintln!("ERROR: {:?}", err);
		break;
	    }
	}
    }

    Ok(())
}
