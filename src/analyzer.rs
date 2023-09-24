use crate::tokenizer;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn analyze() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("Usage: analyzer INPUT_PATH OUTPUT_PATH");
        std::process::exit(1)
    }

    let in_file = &args[0];
    let out_file = &args[1];

    let mut f = File::open(in_file)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut tokenizer = tokenizer::Tokenizer::new(contents);

    let mut output: String = String::new();
    output.push_str("<tokens>\n");

    while tokenizer.has_more_tokens() {
        match tokenizer.token_type() {
            tokenizer::TokenType::Keyword => {
                output.push_str(&format!(
                    "<keyword> {} </keyword>\n",
                    tokenizer.keyword().to_string()
                ));
            }
            tokenizer::TokenType::Identifier => {
                output.push_str(&format!(
                    "<identifier> {} </identifier>\n",
                    tokenizer.identifier()
                ));
            }
            tokenizer::TokenType::Symbol => match tokenizer.symbol() {
                '<' => output.push_str(&format!("<symbol> &lt; </symbol>\n")),
                '>' => output.push_str(&format!("<symbol> &gt; </symbol>\n")),
                '"' => output.push_str(&format!("<symbol> &quot; </symbol>\n")),
                '&' => output.push_str(&format!("<symbol> &amp; </symbol>\n")),
                _ => output.push_str(&format!("<symbol> {} </symbol>\n", tokenizer.symbol())),
            },
            tokenizer::TokenType::IntConst => {
                output.push_str(&format!(
                    "<integerConstant> {} </integerConstant>\n",
                    tokenizer.int_val()
                ));
            }
            tokenizer::TokenType::StringConst => {
                output.push_str(&format!(
                    "<stringConstant> {} </stringConstant>\n",
                    tokenizer.string_val()
                ));
            }
            tokenizer::TokenType::LineComment => output.push_str(&format!(
                "<lineComment> {} </lineComment>\n",
                tokenizer.comment_val()
            )),
            tokenizer::TokenType::BlockComment => output.push_str(&format!(
                "<blockComment> {} </blockComment>\n",
                tokenizer.comment_val()
            )),
        }

        tokenizer.advance()
    }

    output.push_str("</tokens>\n");
    let mut f = File::create(out_file)?;
    f.write_all(output.as_bytes())?;

    Ok(())
}
