use agape_cli::run;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {err}"),
    }
}
