use crate::lexeme;
use crate::lexeme::*;
use std::array;
use std::collections::HashMap;
use std::f32::consts::E;
use std::io;
use std::io::Result;
use std::io::Stdout;
use std::io::Write;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::vec;

pub struct Function {
    parameter: Vec<String>,
    code: Vec<String>,
}

pub struct Interpreter<T: Write> {
    variables: HashMap<String, Value>,
    arrays: HashMap<String, Vec<i32>>,
    strings: HashMap<String, String>,
    float: HashMap<String, f32>,
    functions: HashMap<String, Function>,
    structs: HashMap<String, HashMap<String, i32>>,
    output_stream: T,
}

impl Interpreter<Stdout> {
    pub fn new() -> Interpreter<io::Stdout> {
        Interpreter {
            variables: HashMap::new(),
            arrays: HashMap::new(),
            strings: HashMap::new(),
            float: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            output_stream: io::stdout(),
        }
    }
}

impl<T: Write> Interpreter<T> {
    pub fn run(&mut self, source_code: &str) -> Result<&T> {
        let mut source = source_code.split(";"); // Split by semicolon for multiple statements
        while let Some(line) = source.next() {
            let mut words = line.trim().split_whitespace();
            if let Some(word) = words.next() {
                let keyword = Lexeme::from_str(word);
                match keyword {
                    Lexeme::Var => {
                        if let Some(name_type) = words.next() {
                            if let Some(equal) = words.next() {
                                if equal != "=" {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "Expected '='",
                                    ));
                                }
                                let value_str = words.collect::<Vec<_>>().join(" "); // Join remaining tokens

                                if let Some((name, var_type)) = name_type.split_once(':') {
                                    let name = name.trim();
                                    let var_type = var_type.trim();
                                    let value_str = value_str.trim(); // Ensure we trim the value as well

                                    // Parse value based on type
                                    let value = match var_type {
                                        "int" => match value_str.parse::<i32>() {
                                            Ok(v) => Ok(Value::Int(v)),
                                            Err(_) => Err(io::Error::new(
                                                io::ErrorKind::InvalidInput,
                                                "Invalid Value",
                                            )),
                                        },
                                        "float" => match value_str.parse::<f32>() {
                                            Ok(v) => Ok(Value::Float(v)),
                                            Err(_) => Err(io::Error::new(
                                                io::ErrorKind::InvalidInput,
                                                "Invalid Value",
                                            )),
                                        },
                                        "string" => {
                                            if value_str.starts_with('"')
                                                && value_str.ends_with('"')
                                            {
                                                Ok(Value::Str(
                                                    value_str.trim_matches('"').to_string(),
                                                ))
                                            } else {
                                                Err(io::Error::new(
                                                    io::ErrorKind::InvalidInput,
                                                    "String must be enclosed in double quotes",
                                                ))
                                            }
                                        }
                                        _ => Err(io::Error::new(
                                            io::ErrorKind::InvalidInput,
                                            "Unknown type",
                                        )),
                                    };
                                    // Store the variable if parsing is successful
                                    match value {
                                        Ok(v) => {
                                            self.variables.insert(name.to_string(), v);
                                        }
                                        Err(e) => return Err(e),
                                    }
                                } else {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "Expected 'name:type' format",
                                    ));
                                }
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "Expected '=' after variable declaration",
                                ));
                            }
                        }
                    }
                    Lexeme::Print => {
                        if let Some(name) = words.next() {
                            if let Some(value) = self.variables.get(name) {
                                match value {
                                    Value::Int(v) => writeln!(self.output_stream, "{}", v)?,
                                    Value::Float(v) => writeln!(self.output_stream, "{}", v)?,
                                    Value::Str(v) => writeln!(self.output_stream, "{}", v)?,
                                }
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::NotFound,
                                    format!("variable '{}' not found", name),
                                ));
                            }
                        }
                    }
                    _ => {} // Handle other lexemes
                }
            }
        }
        Ok(self.output_stream.by_ref())
    }
}
