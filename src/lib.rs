use std::fs;
use std::error::Error;

pub struct Config {
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't provide input file"),
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't provide output file"),
        };

        Ok(Config { input_file, output_file })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let mut assembler = HackAssembler::new(&config);
    // let mut parser = Parser::new(&config)?;
    // let mut symbols = SymbolTable::new();
    //
    // // First pass
    // loop {
    //     match parser.instruction_type() {
    //         Some(Instruction::L) => {
    //             // add to the symbol table
    //             symbols.add_entry(parser.symbol().unwrap(), parser.current_instruction as i32);
    //             // remove that line, so further symbols match the lines
    //             if parser.has_more_lines() {
    //                 // do not advance here!
    //                 parser.lines.remove(parser.current_instruction);
    //                 continue;
    //             } else {
    //                 parser.lines.remove(parser.current_instruction);
    //                 break;
    //             }
    //         }
    //         _ => (),
    //     }
    //
    //     if !parser.has_more_lines() {
    //         break;
    //     }
    //
    //     parser.advance();
    // }
    //
    // // reset parser
    // parser.current_instruction = 0;
    //
    // // Second pass
    // loop {
    //     match parser.instruction_type() {
    //         Some(Instruction::A) => {
    //             match parser.symbol().unwrap().parse::<i32>() {
    //                 Ok(num) => {
    //                     let binary = format!("{:016b}", num);
    //                     assembler.add_bytecode(&binary);
    //                 },
    //                 _ => {
    //                     // ether label or variable
    //                     if symbols.contains(&parser.symbol().unwrap()) {
    //                         let address: &i32 = symbols.get_address(&parser.symbol().unwrap()).unwrap();
    //                         let binary = format!("{:016b}", address);
    //                         assembler.add_bytecode(&binary);
    //                     } else {
    //                         // this is a variable
    //                         let binary = format!("{:016b}", parser.current_variable_address);
    //                         assembler.add_bytecode(&binary);
    //
    //                         symbols.add_entry(parser.symbol().unwrap(), parser.current_variable_address);
    //                         parser.current_variable_address += 1;
    //                     }
    //                 },
    //             }
    //         },
    //         Some(Instruction::C) => {
    //             let mut binary = String::from("111");
    //             binary += &Code::comp(parser.comp());
    //             binary += &Code::dest(parser.dest());
    //             binary += &Code::jump(parser.jump());
    //             assembler.add_bytecode(&binary);
    //         }
    //         _ => (),
    //     }
    //
    //     if !parser.has_more_lines() {
    //         break;
    //     }
    //
    //     parser.advance();
    // }
    //
    // assembler.write_to_file()?;

    Ok(())
}

struct Parser {
    lines: Vec<String>,
    current_instruction: usize,
    current_command_type: Option<CommandType>,
}

impl Parser {
    fn new(config: &Config) -> Result<Parser, Box<dyn Error>> {
        let source = fs::read_to_string(&config.input_file)?;

        Ok(Parser::create(source))
    }

    fn create(contents: String) -> Parser {
        let mut parser = Parser {
            lines: Vec::new(),
            current_instruction: 0,
            current_command_type: None,
        };

        parser.lines = contents
            .lines()
            .map(|line| {
                match line.find("//") {
                    Some(index) => &line[..index],
                    None => line
                }
            })
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        parser
    }

    fn has_more_lines(&self) -> bool {
        self.lines.len() > self.current_instruction + 1
    }

    fn advance(&mut self) {
        self.current_instruction += 1;
        self.current_command_type = None;
    }

    fn command_type(&mut self) -> Option<CommandType> {
        if self.current_instruction >= self.lines.len() {
            return None;
        }

        if self.current_command_type.is_some() {
            return self.current_command_type.clone();
        }

        if let Some(command) = self.lines[self.current_instruction].split_whitespace().next() {
            let command_type = match command {
                "add" => Some(CommandType::Arithmetic),
                "sub" => Some(CommandType::Arithmetic),
                "neg" => Some(CommandType::Arithmetic),
                "eq" => Some(CommandType::Arithmetic),
                "gt" => Some(CommandType::Arithmetic),
                "lt" => Some(CommandType::Arithmetic),
                "and" => Some(CommandType::Arithmetic),
                "or" => Some(CommandType::Arithmetic),
                "not" => Some(CommandType::Arithmetic),
                "push" => Some(CommandType::Push),
                "pop" => Some(CommandType::Pop),
                "label" => Some(CommandType::Label),
                "goto" => Some(CommandType::Goto),
                "if" => Some(CommandType::If),
                "function" => Some(CommandType::Function),
                "return" => Some(CommandType::Return),
                "call" => Some(CommandType::Call),
                _ => None,
            };

            self.current_command_type = command_type.clone();
            command_type
        } else {
            None
        }
    }

    fn arg1(&mut self) -> Option<String> {
        if self.current_instruction >= self.lines.len() {
            return None;
        }

        let command_type = self.command_type();

        let mut iter = self.lines[self.current_instruction].split_whitespace();

        match command_type {
            Some(CommandType::Arithmetic) => Some(iter.next().unwrap().to_string()),
            Some(CommandType::Push) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Pop) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Label) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Goto) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::If) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Function) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Call) => {
                iter.next();
                Some(iter.next().unwrap().to_string())
            },
            Some(CommandType::Return) => None,
            _ => None,
        }
    }

    fn arg2(&mut self) -> Option<u16> {
        if self.current_instruction >= self.lines.len() {
            return None;
        }

        let command_type = self.command_type();

        let mut iter = self.lines[self.current_instruction].split_whitespace();

        match command_type {
            Some(CommandType::Push) => {
                iter.next();
                iter.next();
                Some(iter.next().unwrap().parse::<u16>().unwrap())
            },
            Some(CommandType::Pop) => {
                iter.next();
                iter.next();
                Some(iter.next().unwrap().parse::<u16>().unwrap())
            },
            Some(CommandType::Function) => {
                iter.next();
                iter.next();
                Some(iter.next().unwrap().parse::<u16>().unwrap())
            },
            Some(CommandType::Call) => {
                iter.next();
                iter.next();
                Some(iter.next().unwrap().parse::<u16>().unwrap())
            },
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call
}

struct CodeWriter {
    output_file: String,
    assembly_code: String,
}

impl CodeWriter {
    fn new(config: &Config) -> CodeWriter {
        CodeWriter {
            output_file: config.output_file.clone(),
            assembly_code: String::new(),
        }
    }

    fn write_to_file(&self) -> Result<(), Box<dyn Error>> {
        fs::write(self.output_file.clone(), self.assembly_code.clone())?;

        Ok(())
    }

    fn add_assembly_code(&mut self, code: &str) -> Result<(), String> {
        self.assembly_code += code;

        Ok(())
    }
}

struct Code;

impl Code {
    fn generate_arithmetic(command: &str) -> String {
        let mut assembly = String::new();
        match str {
            "add" => {
                assembly += "@SP\n";
                assembly += "M=M-1\n";
                assembly += "D=M\n";
                assembly += "M=M-1\n";
                assembly += "D=D+M\n";
                assembly += "@R0\n";
                assembly += "M=D\n";
            },
            "sub" => (),
            "neg" => (),
            "eq" => (),
            "gt" => (),
            "lt" => (),
            "and" => (),
            "or" => (),
            "not" => (),
        }

        assembly
    }

    fn generate_push_pop(command: CommandType, segment: &str, index: u16) -> String {
        let mut assembly = String::new();
        match segment {
            "constant" => {
                match CommandType {
                    CommandType::Push => {
                        assembly += &format!("@{}\n", index);
                        assembly += "D=A\n";
                        assembly += "@SP\n";
                        assembly += "M=D\n";
                        assembly += "M=D\n";
                    },
                    _ => (),
                }
            },
            _ => (),
        }

        assembly
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type() {
        let contents= "\
push constant 7
push constant 8
add
";
        let mut parser = Parser::create(contents.to_string());
        assert_eq!(parser.command_type().unwrap(), CommandType::Push);
        assert_eq!(parser.arg1().unwrap(), "constant".to_string());
        assert_eq!(parser.arg2().unwrap(), 7);
        parser.advance();
        assert_eq!(parser.command_type().unwrap(), CommandType::Push);
        assert_eq!(parser.arg1().unwrap(), "constant".to_string());
        assert_eq!(parser.arg2().unwrap(), 8);
        parser.advance();
        assert_eq!(parser.command_type().unwrap(), CommandType::Arithmetic);
        assert_eq!(parser.arg1().unwrap(), "add".to_string());
        assert_eq!(parser.arg2(), None);
        assert!(!parser.has_more_lines());
        parser.advance();
        assert!(parser.command_type().is_none());
        assert_eq!(parser.arg1(), None);
        assert_eq!(parser.arg2(), None);
    }

}