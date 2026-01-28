pub mod lexer;
pub mod parser;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("DN Compiler v0.1.0");
        eprintln!("Usage: dnc <file.dn> [options]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --tokens    Print tokens only (don't compile)");
        eprintln!("  --help      Show this help");
        return ExitCode::FAILURE;
    }

    let filename = args[1].clone();

    if args.iter().any(|arg| arg == "--help") {
        eprintln!("DVMC: 0.1");
        eprintln!("Usage: dnc <file.dn> [options]");
        return ExitCode::SUCCESS;
    }

    let code = match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}, couldn't read {}", e, filename);
            return ExitCode::FAILURE;
        }
    };

    let mut lex = lexer::Lexer::new(&code);

    let tokens = match lex.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "{}:{}:{}: Error: {}",
                filename, e.span.line, e.span.col, e.msg
            );
            return ExitCode::FAILURE;
        }
    };

    if args.iter().any(|arg| arg == "--print") {
        for token in tokens.iter() {
            if *token != crate::lexer::Token::Term {
                print!("{}  ", token);
            } else {
                println!("{}", token);
            }
        }
        return ExitCode::SUCCESS;
    }

    println!("Compiled file: {}", filename);
    return ExitCode::SUCCESS;
}
