use miette::GraphicalReportHandler;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use sql_jr_parser::{query::SqlQuery, types::Parse};

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match SqlQuery::parse_format_error(line.as_ref()) {
                    Ok(q) => println!("{q:?}"),
                    Err(e) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &e)
                            .unwrap();
                        println!("{s}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
