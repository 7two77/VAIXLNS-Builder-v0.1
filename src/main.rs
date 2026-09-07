use vaixlns_builder::lexer::tokenize;
use vaixlns_builder::parser::Parser;
use std::fs;

fn main() {
    println!("╔════════════════════════════════════════════╗");
    println!("║   VAIXLNS Builder v0.1 - Bootstrap        ║");
    println!("╚════════════════════════════════════════════╝");
    println!();

    let input = match fs::read_to_string("VAIXLNS_ROOT.lns") {
        Ok(content) => {
            println!("✅ Loaded VAIXLNS_ROOT.lns ({} bytes)", content.len());
            content
        }
        Err(e) => {
            println!("⚠️  VAIXLNS_ROOT.lns not found: {}", e);
            return;
        }
    };
    println!();

    let tokens = tokenize(&input);
    println!("🔍 Lexer: {} tokens found", tokens.len());
    println!();

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(doc) => {
            println!("✅ Parser: {} declarations parsed", doc.decls.len());
            println!();
            println!("📦 VAIXLNS Builder is ready!");
        }
        Err(e) => {
            eprintln!("❌ Parser error: {}", e);
            std::process::exit(1);
        }
    }
}
