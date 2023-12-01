use crate::tokenizer::{Keyword, TokenType, Tokenizer};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct CompilationEngine {
    tokenizer: Tokenizer,
    pub output: String,
}

impl CompilationEngine {
    pub fn new(code_contents: String) -> CompilationEngine {
        CompilationEngine {
            tokenizer: Tokenizer::new(code_contents, true),
            output: String::new(),
        }
    }

    fn write_output(&mut self, depth: u32, o: String) {
        let mut space = String::new();
        for _ in 0..depth {
            space.push_str("  ");
        }
        self.output.push_str(&format!("{}{}\n", space, &o));
    }

    pub fn compile_class(&mut self) {
        let mut depth: u32 = 0;
        self.write_output(depth, format!("<class>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.compile_class_var_decs(depth);
        self.compile_subrouties(depth);

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</class>"));
    }

    fn compile_class_var_decs(&mut self, depth: u32) {
        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Keyword => match self.tokenizer.keyword() {
                    Keyword::Static | Keyword::Field => self.compile_class_var_dec(depth),
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }

            self.tokenizer.advance();
        }
    }

    fn compile_class_var_dec(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<classVarDec>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            TokenType::Keyword => {
                self.write_output(
                    depth,
                    format!(
                        "<keyword> {} </keyword>",
                        self.tokenizer.keyword().to_string()
                    ),
                );
            }
            TokenType::Identifier => {
                self.write_output(
                    depth,
                    format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                );
            }
            _ => {}
        }

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );

        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                TokenType::Symbol => match self.tokenizer.symbol() {
                    ';' => {
                        break;
                    }
                    ',' => {
                        self.write_output(
                            depth,
                            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                        );
                        self.tokenizer.advance();
                        self.write_output(
                            depth,
                            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                        )
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</classVarDec>"));
    }

    fn compile_subrouties(&mut self, depth: u32) {
        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Keyword => match self.tokenizer.keyword() {
                    Keyword::Constructor | Keyword::Function | Keyword::Method => {
                        self.compile_subroutie(depth)
                    }
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }

            self.tokenizer.advance();
        }
    }

    fn compile_subroutie(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<subroutineDec>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            TokenType::Keyword => {
                self.write_output(
                    depth,
                    format!(
                        "<keyword> {} </keyword>",
                        self.tokenizer.keyword().to_string()
                    ),
                );
            }
            TokenType::Identifier => {
                self.write_output(
                    depth,
                    format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                );
            }
            _ => {}
        }

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.compile_parameter_list(depth);

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.compile_subroutine_body(depth);

        depth -= 1;
        self.write_output(depth, format!("</subroutineDec>"));
    }

    fn compile_parameter_list(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<parameterList>"));
        depth += 1;

        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Symbol => match self.tokenizer.symbol() {
                    ')' => {
                        break;
                    }
                    ',' => {
                        self.write_output(
                            depth,
                            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                        );

                        self.tokenizer.advance();
                        match self.tokenizer.token_type() {
                            TokenType::Keyword => {
                                self.write_output(
                                    depth,
                                    format!(
                                        "<keyword> {} </keyword>",
                                        self.tokenizer.keyword().to_string()
                                    ),
                                );
                            }
                            TokenType::Identifier => {
                                self.write_output(
                                    depth,
                                    format!(
                                        "<identifier> {} </identifier>",
                                        self.tokenizer.identifier()
                                    ),
                                );
                            }
                            _ => {}
                        }

                        self.tokenizer.advance();
                        self.write_output(
                            depth,
                            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                        );
                    }
                    _ => {}
                },
                TokenType::Keyword | TokenType::Identifier => {
                    match self.tokenizer.token_type() {
                        TokenType::Keyword => {
                            self.write_output(
                                depth,
                                format!(
                                    "<keyword> {} </keyword>",
                                    self.tokenizer.keyword().to_string()
                                ),
                            );
                        }
                        TokenType::Identifier => {
                            self.write_output(
                                depth,
                                format!(
                                    "<identifier> {} </identifier>",
                                    self.tokenizer.identifier()
                                ),
                            );
                        }
                        _ => {}
                    }

                    self.tokenizer.advance();
                    self.write_output(
                        depth,
                        format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                    );
                }
                _ => {}
            }

            self.tokenizer.advance();
        }

        depth -= 1;
        self.write_output(depth, format!("</parameterList>"));
    }

    fn compile_subroutine_body(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<subroutineBody>"));
        depth += 1;

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.compile_var_decs(depth);
        self.compile_statements(depth);

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</subroutineBody>"));
    }

    fn compile_var_decs(&mut self, depth: u32) {
        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Keyword => match self.tokenizer.keyword() {
                    Keyword::Var => self.compile_var_dec(depth),
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }

            self.tokenizer.advance();
        }
    }

    fn compile_var_dec(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<varDec>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            TokenType::Keyword => {
                self.write_output(
                    depth,
                    format!(
                        "<keyword> {} </keyword>",
                        self.tokenizer.keyword().to_string()
                    ),
                );
            }
            TokenType::Identifier => {
                self.write_output(
                    depth,
                    format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                );
            }
            _ => {}
        }

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );

        self.tokenizer.advance();
        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Symbol => match self.tokenizer.symbol() {
                    ';' => break,
                    ',' => {
                        self.write_output(
                            depth,
                            format!("<sybmol> {} </sybmol>", self.tokenizer.symbol()),
                        );

                        self.tokenizer.advance();
                        self.write_output(
                            depth,
                            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                        );
                    }
                    _ => break,
                },

                _ => break,
            }
            self.tokenizer.advance();
        }

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</varDec>"));
    }

    fn compile_statements(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<statements>"));
        depth += 1;

        loop {
            if !self.tokenizer.has_more_tokens() {
                break;
            }

            match self.tokenizer.token_type() {
                TokenType::Keyword => match self.tokenizer.keyword() {
                    Keyword::Let => self.compile_let(depth),
                    Keyword::If => self.compile_if(depth),
                    Keyword::While => self.compile_while(depth),
                    Keyword::Do => self.compile_do(depth),
                    Keyword::Return => self.compile_return(depth),
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }

            self.tokenizer.advance();
        }

        depth -= 1;
        self.write_output(depth, format!("</statements>"));
    }

    fn compile_let(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<letStatement>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );
        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );
        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );
        self.tokenizer.advance();

        self.write_output(depth, format!("<expression>"));
        self.tokenizer.advance();
        self.write_output(depth, format!("</expression>"));

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );
        self.write_output(depth, format!("</letStatement>"))
    }

    fn compile_if(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<ifStatement>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.write_output(depth, format!("<expression>"));
        self.tokenizer.advance();
        self.write_output(depth, format!("</expression>"));

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</ifStatement>"))
    }

    fn compile_while(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<whileStatement>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.write_output(depth, format!("<expression>"));
        self.tokenizer.advance();
        self.write_output(depth, format!("</expression>"));

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</whileStatement>"))
    }

    fn compile_do(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<doStatement>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
        );

        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            TokenType::Symbol => match self.tokenizer.symbol() {
                '.' => {
                    self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    );

                    self.tokenizer.advance();
                    self.write_output(
                        depth,
                        format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                    );
                }
                _ => {}
            },
            _ => {}
        }

        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        self.compile_expression_list(depth);

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</doStatement>"));
    }

    fn compile_return(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<returnStatement>"));
        depth += 1;

        self.write_output(
            depth,
            format!(
                "<keyword> {} </keyword>",
                self.tokenizer.keyword().to_string()
            ),
        );

        self.tokenizer.advance();
        match self.tokenizer.token_type() {
            TokenType::Symbol => match self.tokenizer.symbol() {
                ';' => {
                    self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    );
                    self.write_output(depth, format!("</returnStatement>"));
                    return;
                }
                _ => self.compile_expression(depth),
            },
            _ => self.compile_expression(depth),
        }

        self.tokenizer.advance();
        self.write_output(
            depth,
            format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
        );

        depth -= 1;
        self.write_output(depth, format!("</returnStatement>"));
    }

    fn compile_expression(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<expression>"));
        depth += 1;

        self.compile_term(depth);

        depth -= 1;
        self.write_output(depth, format!("</expression>"));
    }

    fn compile_term(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<term>"));
        depth += 1;

        match self.tokenizer.token_type() {
            TokenType::IntConst => {
                self.write_output(
                    depth,
                    format!(
                        "<integerConstant> {} </integerConstant>",
                        self.tokenizer.int_val()
                    ),
                );
            }
            TokenType::StringConst => {
                self.write_output(
                    depth,
                    format!(
                        "<stringConstant> {} </stringConstant>",
                        self.tokenizer.string_val()
                    ),
                );
            }
            TokenType::Keyword => match self.tokenizer.keyword() {
                Keyword::True | Keyword::False | Keyword::Null | Keyword::This => {
                    self.write_output(
                        depth,
                        format!(
                            "<keyword> {} </keyword>",
                            self.tokenizer.keyword().to_string()
                        ),
                    );
                }
                _ => {}
            },
            TokenType::Identifier => {
                self.write_output(
                    depth,
                    format!("<identifier> {} </identifier>", self.tokenizer.identifier()),
                );

                if self.tokenizer.has_more_tokens() {
                    self.tokenizer.advance();
                    match self.tokenizer.token_type() {
                        TokenType::Symbol => match self.tokenizer.symbol() {
                            '[' => {
                                self.write_output(
                                    depth,
                                    format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                                );

                                if self.tokenizer.has_more_tokens() {
                                    self.tokenizer.advance();
                                    self.compile_expression(depth);
                                }

                                self.write_output(
                                    depth,
                                    format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                                );
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            TokenType::Symbol => match self.tokenizer.symbol() {
                '(' => {
                    self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    );

                    self.tokenizer.advance();
                    self.compile_expression(depth);

                    self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    );
                }
                '-' | '~' => {
                    self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    );

                    self.tokenizer.advance();
                    self.compile_term(depth)
                }
                _ => {}
            },
            _ => {}
        }

        depth -= 1;
        self.write_output(depth, format!("</term>"));
    }

    fn compile_expression_list(&mut self, mut depth: u32) {
        self.write_output(depth, format!("<expressionList>"));
        depth += 1;

        if self.tokenizer.has_more_tokens() {
            self.tokenizer.advance();
            self.compile_expression(depth);
        }

        while self.tokenizer.has_more_tokens() {
            self.tokenizer.advance();
            match self.tokenizer.token_type() {
                TokenType::Symbol => match self.tokenizer.symbol() {
                    ',' => self.write_output(
                        depth,
                        format!("<symbol> {} </symbol>", self.tokenizer.symbol()),
                    ),
                    _ => {
                        break;
                    }
                },
                _ => {
                    break;
                }
            }

            if self.tokenizer.has_more_tokens() {
                self.tokenizer.advance();
                self.compile_expression(depth);
            }
        }

        depth -= 1;
        self.write_output(depth, format!("</expressionList>"));
    }
}

#[test]
fn test_compilation_engine() -> io::Result<()> {
    let mut f = File::open("/Users/mo/nand2tetris/projects/10/Square/Main2.jack")?;
    let mut code_contents = String::new();
    f.read_to_string(&mut code_contents)?;

    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_class();

    Ok(())
}

#[test]
fn test_compile_class() {
    let code_contents = "class Main {}".to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_class();
    let expected = r###"<class>
  <keyword> class </keyword>
  <identifier> Main </identifier>
  <symbol> { </symbol>
  <symbol> } </symbol>
</class>
"###;
    assert_eq!(expected, compilation_engine.output);
}

#[test]
fn test_compile_class_var_decs() {
    let code_contents = r###"
    static int staticInt;
    static char staticChar;
    static boolean staticBoolean;
    static ClassName staticClassName;

    field int fieldInt;
    field char fieldChar;
    field boolean filedBoolean;
    field ClassName filedClassName;
    "###
    .to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_class_var_decs(0);

    let expected = r###"<classVarDec>
  <keyword> static </keyword>
  <keyword> int </keyword>
  <identifier> staticInt </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> static </keyword>
  <keyword> char </keyword>
  <identifier> staticChar </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> static </keyword>
  <keyword> boolean </keyword>
  <identifier> staticBoolean </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> static </keyword>
  <identifier> ClassName </identifier>
  <identifier> staticClassName </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> field </keyword>
  <keyword> int </keyword>
  <identifier> fieldInt </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> field </keyword>
  <keyword> char </keyword>
  <identifier> fieldChar </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> field </keyword>
  <keyword> boolean </keyword>
  <identifier> filedBoolean </identifier>
  <symbol> ; </symbol>
</classVarDec>
<classVarDec>
  <keyword> field </keyword>
  <identifier> ClassName </identifier>
  <identifier> filedClassName </identifier>
  <symbol> ; </symbol>
</classVarDec>
"###;

    assert_eq!(expected, compilation_engine.output);
}

#[test]
fn test_compile_subroutines() {
    let code_contents = r###"
    constructor int cInt() {}
    constructor char cChar() {}
    constructor boolean cBoolean() {}
    constructor ClassName cClassName() {}

    function int fInt() {}
    function char fChar() {}
    function boolean fBoolean() {}
    function ClassName fClassName() {}

    method int mInt() {}
    method char mChar() {}
    method boolean mBoolean() {}
    method ClassName mClassName() {}
    "###
    .to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_subrouties(0);
    let expected = r###"<subroutineDec>
  <keyword> constructor </keyword>
  <keyword> int </keyword>
  <identifier> cInt </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> constructor </keyword>
  <keyword> char </keyword>
  <identifier> cChar </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> constructor </keyword>
  <keyword> boolean </keyword>
  <identifier> cBoolean </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> constructor </keyword>
  <identifier> ClassName </identifier>
  <identifier> cClassName </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> function </keyword>
  <keyword> int </keyword>
  <identifier> fInt </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> function </keyword>
  <keyword> char </keyword>
  <identifier> fChar </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> function </keyword>
  <keyword> boolean </keyword>
  <identifier> fBoolean </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> function </keyword>
  <identifier> ClassName </identifier>
  <identifier> fClassName </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> method </keyword>
  <keyword> int </keyword>
  <identifier> mInt </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> method </keyword>
  <keyword> char </keyword>
  <identifier> mChar </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> method </keyword>
  <keyword> boolean </keyword>
  <identifier> mBoolean </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
<subroutineDec>
  <keyword> method </keyword>
  <identifier> ClassName </identifier>
  <identifier> mClassName </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec
"###;
    assert_eq!(
        expected, compilation_engine.output,
        "expected {}\noutput {}\n",
        expected, compilation_engine.output
    );
}

#[test]
fn test_compile_parameter_list() {
    let code_contents = "int a, char b, boolean c, ClassName d ".to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_parameter_list(0);
    let expected = r###"<parameterList>
  <keyword> int </keyword>
  <identifier> a </identifier>
  <symbol> , </symbol>
  <keyword> char </keyword>
  <identifier> b </identifier>
  <symbol> , </symbol>
  <keyword> boolean </keyword>
  <identifier> c </identifier>
  <symbol> , </symbol>
  <identifier> ClassName </identifier>
  <identifier> d </identifier>
</parameterList>
"###;
    assert_eq!(expected, compilation_engine.output);
}

#[test]
fn test_compile_subroutine_body() {
    let code_contents = "{}".to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_subroutine_body(0);
    println!("{}", compilation_engine.output);
}

#[test]
fn test_compile_var_decs() {
    let code_contents = r###"
    var int i1, i2, i3;
    var char c1, c2, c3;
    var boolean b1, b2, b3;
    var ClassName cn1, cn2, cn3;
    "###
    .to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_var_decs(0);
    println!("{}", compilation_engine.output);
}

#[test]
fn test_compile_var_dec() {
    let code_contents = r"var int i, j, k;".to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_var_dec(0);
    println!("{}", compilation_engine.output);
}

#[test]
fn test_compile_statements() {
    let code_contents = r"###
    let i = true;
    if (true) {}
    while(true) {}
    do run();
    return true;
    ###"
    .to_string();
    let mut compilation_engine = CompilationEngine::new(code_contents);
    compilation_engine.compile_statements(0);
    println!("{}", compilation_engine.output);
}

#[test]
fn test_compile_let() {
    {
        let code_contents = "let y = Ay;".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_let(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_if() {
    {
        let code_contents = "if(true) {}".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_if(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_while() {
    {
        let code_contents = "while(true) {}".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_while(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_do() {
    {
        let code_contents = "do run();".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_do(0);
        println!("{}", compilation_engine.output);
    }
    {
        let code_contents = "do game.run();".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_do(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_return() {
    {
        let code_contents = "return;".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_return(0);
        println!("{}", compilation_engine.output);
    }
    {
        let code_contents = "return true;".to_string();
        let mut compilation_engine = CompilationEngine::new(code_contents);
        compilation_engine.compile_return(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_term() {
    let code_contents_vec = vec![
        "42 ", "\"Yo\"", "true ", "false ", "null ", "this ", "x ", "x[1] ", "(1) ", "-1 ",
        "~true ",
    ];
    for code_contents in code_contents_vec {
        let mut compilation_engine = CompilationEngine::new(code_contents.to_string());
        compilation_engine.compile_term(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_compile_expression_list() {
    let code_contents_vec = vec!["1, \"X\", true, ClassName "];
    for code_contents in code_contents_vec {
        let mut compilation_engine = CompilationEngine::new(code_contents.to_string());
        compilation_engine.compile_expression_list(0);
        println!("{}", compilation_engine.output);
    }
}

#[test]
fn test_write_space() {
    let mut compilation_engine = CompilationEngine::new("".to_string());
    compilation_engine.write_output(0, "<keyword>".to_string());
    compilation_engine.write_output(1, "<keyword>".to_string());
    println!("{}", compilation_engine.output)
}
