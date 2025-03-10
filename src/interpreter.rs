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
                                let value_str = words.collect::<Vec<_>>().join(" ");

                                if let Some((name, var_type)) = name_type.split_once(':') {
                                    let name = name.trim();
                                    let var_type = var_type.trim();
                                    let value_str = value_str.trim();

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
                                        "bool" => match value_str {
                                            "true" => Ok(Value::Bool(true)),
                                            "false" => Ok(Value::Bool(false)),
                                            _ => Err(io::Error::new(
                                                io::ErrorKind::InvalidInput,
                                                "Invalid boolean value",
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
                                        "array" => {
                                            if value_str.starts_with('[')
                                                && value_str.ends_with(']')
                                            {
                                                let elements: Result<Vec<Value>> = value_str
                                                    .trim_matches(|c| c == '[' || c == ']')
                                                    .split(',')
                                                    .map(|s| {
                                                        let s = s.trim();
                                                        s.parse::<i32>()
                                                            .map(Value::Int)
                                                            .or_else(|_| {
                                                                s.parse::<f32>().map(Value::Float)
                                                            })
                                                            .or_else(|_| match s {
                                                                "true" => Ok(Value::Bool(true)),
                                                                "false" => Ok(Value::Bool(false)),
                                                                _ if s.starts_with('"')
                                                                    && s.ends_with('"') =>
                                                                {
                                                                    Ok(Value::Str(
                                                                        s.trim_matches('"')
                                                                            .to_string(),
                                                                    ))
                                                                }
                                                                _ => Err(io::Error::new(
                                                                    io::ErrorKind::InvalidInput,
                                                                    "Invalid array element",
                                                                )),
                                                            })
                                                    })
                                                    .collect();

                                                match elements {
                                                    Ok(arr) => Ok(Value::Array(arr)),
                                                    Err(_) => Err(io::Error::new(
                                                        io::ErrorKind::InvalidInput,
                                                        "Invalid array value",
                                                    )),
                                                }
                                            } else {
                                                Err(io::Error::new(
                                                    io::ErrorKind::InvalidInput,
                                                    "Array must be enclosed in []",
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
                                    Value::Bool(v) => writeln!(self.output_stream, "{}", v)?,
                                    Value::Array(arr) => {
                                        let elements: Vec<String> = arr
                                            .iter()
                                            .map(|v| match v {
                                                Value::Int(n) => n.to_string(),
                                                Value::Str(n) => n.to_string(),
                                                Value::Float(n) => n.to_string(),
                                                Value::Bool(n) => n.to_string(),
                                                _ => "unsupported".to_string(),
                                            })
                                            .collect();
                                        writeln!(self.output_stream, "[{}]", elements.join(","))?;
                                    }
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
