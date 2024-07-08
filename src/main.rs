use std::fs::File;
use std::io::Read;
use std::path::Path;

mod lexer;
use lexer::lexer::Lexer;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_c_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    match run_lexer(file_path) {
        Ok(_) => println!("Lexing completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run_lexer(file_path: &str) -> Result<(), std::io::Error> {
    // Read file
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)?;

    // Create Lexer
    let mut lexer = Lexer::new(&source_code);

    // Print token stream
    println!("Tokens:");
    while let Some(token) = lexer.next_token() {
        println!("{}", token);
    }

    // Print errors
    let errors = lexer.get_errors();
    if !errors.is_empty() {
        println!("\nLexer errors:");
        for error in errors {
            println!("  Error at {}:{}: {}", error.position.row, error.position.col, error.message);
        }
    }

    Ok(())
}
