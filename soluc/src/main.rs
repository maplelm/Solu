pub mod bytecode;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod symantic;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Solu Compiler v0.1.0");
        eprintln!("Usage: dnc <file.sl> [options]");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  --tokens    Print tokens only (don't compile)");
        eprintln!("  --help      Show this help");
        return ExitCode::FAILURE;
    }

    let filename = args[1].clone();

    if args.iter().any(|arg| arg == "--help") {
        eprintln!("SOLC: 0.1");
        eprintln!("Usage: solc <file.sl> [options]");
        return ExitCode::SUCCESS;
    }

    let code = match std::fs::read_to_string(&filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}, couldn't read {}", e, filename);
            return ExitCode::FAILURE;
        }
    };

    let mut lex = lexer::Lexer::new(code);

    let tokens = match lex.parse() {
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
            if token.token != crate::lexer::Token::Term {
                print!("{}  ", token.token);
            } else {
                println!("{}", token.token);
            }
        }
        return ExitCode::SUCCESS;
    }

    let mut parser = parser::Parser::new(tokens);

    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parsing Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    println!("{}\n\n", ast);

    println!("Compiled file: {}", filename);
    return ExitCode::SUCCESS;
}
