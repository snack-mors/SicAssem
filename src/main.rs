mod symbols;
mod mnemonics;
mod ir;
mod pass1;


use std::io;
use clap::Parser;
use crate::pass1::pass_one;

#[derive(Parser)]
#[command(version, about = "A simple file reader")]
struct Args {
    filename: String,

}

fn main() -> Result<(), io::Error> {

    // // Rust has an industry standard CLI parser.
    // // The original C style CLI is deprecated in favor of it.
    // // Vectors are just heap array lists. Don't think about it too hard.
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 || args.len() > 3 {
    //     eprintln!("Usage: {} <filename>", args[0]);
    //     process::exit(1);
    // }
    // let filename = &args[1];
    // println!("Opening file: {}", filename);
    //
    // // The ? operator is an error propagation operator.
    // // If this operation fails, return the error to the function called.
    // let file = File::open(filename)?;

    let args = Args::parse();
    println!("Opening file: {}", args.filename);
    match pass_one(&args.filename) {
        Ok((symtab, _ir)) => { // TODO: REMOVE _ from IR
            println!("Pass 1 Successful!");
            symtab.print_symbols();
        },
        Err(e) => eprintln!("Assembly Failed: {}", e),
    }







    Ok(())
}
