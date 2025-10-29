use agape_cli::run;
use std::process::exit;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => {
            println!("Error: {err}");
            exit(1);
        }
    }
}
