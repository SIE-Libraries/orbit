use clap::Parser;
use spaceship::frontend::lexer::Lexer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    emit_obj: bool,

    input: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Spaceship Compiler v0.1.0");

    let source = std::fs::read_to_string(&args.input)?;
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
