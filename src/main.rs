mod lexer;
mod parser;
mod typecheck;
mod codegen;

use std::fs;
use inkwell::context::Context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input.dlang>", args[0]);
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let source_code = fs::read_to_string(input_path)?;
    
    // 1. Tokenizálás
    let lexer = lexer::Lexer::new(&source_code);
    
    // 2. Parselés
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();
    
    // 3. Típusellenőrzés
    let mut typechecker = typecheck::TypeChecker::new();
    typechecker.check_program(&program)?;
    
    // 4. Kódgenerálás
    let context = Context::create();
    let mut codegen = codegen::Codegen::new(&context);
    codegen.compile(&program)?;
    
    println!("Successfully compiled to output.ll!");
    Ok(())
}