use rustyline::{DefaultEditor, Result};
use rustyline::config::Config;

fn main() -> Result<()> {

    let mut rl = DefaultEditor::new()?;

    loop {
	let readline = rl.readline("FEN: ");
	match readline {
	    Ok(line) => {
		let position = libsparov::parse::parse_fen(&line);

		match position {
		    Ok((_, pos)) => println!("{}", pos.board),
		    Err(e) => eprintln!("ERROR: Parsing failed. {}", e),
		}
	    },

	    Err(err) => {
		eprintln!("ERROR: {}", err);
		break;
	    }
	}
    }

    Ok(())
}
