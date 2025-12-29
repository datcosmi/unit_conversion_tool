mod cli;

fn main() {
    cli::run();
    cli::print_goodbye()
    // if let Err(err) = run() {
    //     eprintln!("{}", err);
    // }
}

