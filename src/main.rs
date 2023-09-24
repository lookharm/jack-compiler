mod analyzer;
mod compilation_engine;
mod tokenizer;

fn main() -> Result<(), std::io::Error> {
    analyzer::analyze()
}
