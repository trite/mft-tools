use clap::{arg, Command};

fn main() {
    let matches =
        Command::new("mft-tools")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(arg!(-e --echo <ECHO>).required(false))
        .arg(arg!(-f --file <FILE>).required(false))
        .get_matches();

    match matches.value_of("echo") {
        Some(e) => println!("Echoing: [{e}]"),
        None => ()
    };
}
