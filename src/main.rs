use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub mod lexer;
use lexer::lexer::Lexer;
use lexer::token::TokenType;

use clap::{Arg, ArgAction, Command};
use psutil::process::Process;

fn main() {

    let matches = Command::new("JCL - Jacob's C Lexer")
        .about("Jacob's C Lexer - A lexer for the C programming language, part of the greater JCC: Jacob's C Compiler.")
        .version("0.0.1")
        .arg_required_else_help(true)
        .args([
            Arg::new("input")
                .help("Input C file to lex.")
                .required(true)
                .index(1),
            Arg::new("no-comments")
                .long("no-comments")
                .help("Exclude comments from output")
                .action(ArgAction::SetTrue),
            Arg::new("show-positions")
                .short('p')
                .long("show-positions")
                .help("Show token positions in the output")
                .action(ArgAction::SetTrue),
            Arg::new("count-tokens")
                .short('c')
                .long("count-tokens")
                .help("Display a count of each token type")
                .action(ArgAction::SetTrue),
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Use verbose output")
                .action(ArgAction::SetTrue),
            Arg::new("diagnostics")
                .short('d')
                .long("diagnostics")
                .help("Show system diagnostics (runtime and memory usage)")
                .action(ArgAction::SetTrue),
        ]).get_matches();

    let start = Instant::now();
    let initial_memory = get_memory_usage().unwrap_or(0.0);

    match run_lexer(&matches) {
        Ok(_) => println!("Lexing completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    if matches.get_flag("diagnostics") {
        let duration = start.elapsed();
        let final_memory = get_memory_usage().unwrap_or(0.0);

        println!("\nSystem Diagnostics:");
        println!("  Runtime: {:.2} seconds", duration.as_secs_f64());
        println!("  Memory Usage: {:.2} MB", final_memory - initial_memory);
    }
}

fn get_memory_usage() -> Option<f64> {
    Process::new(std::process::id() as u32).ok()
        .and_then(|process| process.memory_info().ok())
        .map(|meminfo| meminfo.rss() as f64 / 1_048_576.0) // Convert to MB
}

fn run_lexer(matches: &clap::ArgMatches) -> Result<(), std::io::Error> {

    let input_path = matches.get_one::<String>("input").expect("Required");
    let mut file = File::open(input_path)?;
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)?;

    let mut lexer = Lexer::new(&source_code);

    // Token counting
    let mut token_count: std::collections::HashMap<TokenType, usize> = std::collections::HashMap::new();

    // Print token stream
    println!("Tokens:");
    loop {
        let token = lexer.next_token();

        if token.token_type == TokenType::Comment && matches.get_flag("no-comments"){
            continue;
        }

        // Update token count
        *token_count.entry(token.token_type.clone()).or_insert(0) += 1;


        // Print token
        if matches.get_flag("show-positions") {
            println!("{}", token);
        } else {
            println!("{{ type: {:?}, value: {:?} }}", token.token_type, token.value);
        }

        if matches.get_flag("verbose") {
            println!("  Details:");
            println!("    Row: {}", token.position.row);
            println!("    Column: {}", token.position.col);
            // Add more verbose information as needed
        }


        if token.token_type == TokenType::EOF {
            break;
        }
    }

    // Print token count if requested
    if matches.get_flag("count-tokens") {
        println!("\nToken counts:");
        for (token_type, count) in token_count.iter() {
            println!("  {:?}: {}", token_type, count);
        }
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
