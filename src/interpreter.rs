use crate::lexeme::{Comparison, Lexeme};
use std::collections::HashMap;
use std::io;
use std::io::Result;
use std::io::Stdout;
use std::io::Write;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub struct Function {
    parameters: Vec<String>,
    code: Vec<String>,
}

pub struct Interpreter<T: Write> {
    variables: HashMap<String, i32>,
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
        let mut source = source_code.split(';'); // Split by semicolon
        while let Some(line) = source.next() {
            let mut words = line.trim().split_whitespace();
            if let Some(word) = words.next() {
                let keyword = Lexeme::from_str(word);
                match keyword {
                    Lexeme::Function => {
                        let name = words.next().unwrap();
                        let params = words.next().unwrap(); // Extract parameters from ()
                        let params = params
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                        let mut function_body = String::new();
                        while let Some(line) = source.next() {
                            if line.trim() == "}" {
                                break;
                            } // Stop at closing }
                            function_body.push_str(line);
                            function_body.push(';');
                        }
                        self.functions.insert(
                            name.to_owned(),
                            Function {
                                parameters: params,
                                code: function_body.split(';').map(String::from).collect(),
                            },
                        );
                    }
                    Lexeme::Call => {
                        let name = words.next().unwrap();
                        let params = words.next().unwrap();
                        let params: Vec<i32> = params
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .map(|s| s.trim().parse().unwrap())
                            .collect();
                        self._call_function(name, &params)?;
                    }
                    Lexeme::Var => {
                        if let Some(name) = words.next() {
                            if let Some(equal) = words.next() {
                                if equal != "=" {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "Expected '=' before string value",
                                    ));
                                }
                                if let Some(value_str) = words.next() {
                                    if let Ok(value) = value_str.parse() {
                                        self.variables.insert(name.to_owned(), value);
                                    } else {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidInput,
                                            "Invalid number format",
                                        ));
                                    }
                                }
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "Missing variable name or value",
                                ));
                            }
                        }
                    }
                    Lexeme::Array => {
                        let name = source.next().unwrap();
                        let size = source.next().unwrap().parse().unwrap();
                        let mut array = Vec::with_capacity(size);
                        for _ in 0..size {
                            let value = source.next().unwrap().parse().unwrap();
                            array.push(value);
                        }
                        self.arrays.insert(name.to_owned(), array);
                    }
                    Lexeme::String => {
                        if let Some(name) = words.next() {
                            if let Some(equal) = words.next() {
                                if equal != "=" {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "Expected '=' before string value",
                                    ));
                                }
                                let rest_of_line: String = words.collect::<Vec<_>>().join(" ");
                                if let Some(start_index) = rest_of_line.find('"') {
                                    if let Some(end_index) = rest_of_line.rfind('"') {
                                        if start_index != end_index {
                                            let value = &rest_of_line[start_index + 1..end_index];
                                            self.strings.insert(name.to_owned(), value.to_string());
                                        } else {
                                            return Err(io::Error::new(
                                                io::ErrorKind::InvalidInput,
                                                "String missing closing quote",
                                            ));
                                        }
                                    } else {
                                        return Err(io::Error::new(
                                            io::ErrorKind::InvalidInput,
                                            "String missing closing quote",
                                        ));
                                    }
                                } else {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidInput,
                                        "String missing opening quote",
                                    ));
                                }
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "Expected '=' after variable name",
                                ));
                            }
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::UnexpectedEof,
                                "Expected variable name after string declaration",
                            ));
                        }
                    }
                    Lexeme::Float => {
                        let name = source.next().unwrap();
                        let value = source.next().unwrap().parse().unwrap();
                        self.float.insert(name.to_owned(), value);
                    }
                    Lexeme::Struct => {
                        let name = source.next().unwrap();
                        let mut struct_fields = HashMap::new();
                        while let Some(field) = source.next() {
                            if field == "endstruct" {
                                break;
                            }
                            let value = source.next().unwrap().parse().unwrap();
                            struct_fields.insert(field.to_owned(), value);
                        }
                        self.structs.insert(name.to_owned(), struct_fields);
                    }
                    Lexeme::Switch => {
                        let name = source.next().unwrap();
                        let value = self.variables[name];
                        let mut found = false;
                        while let Some(word) = source.next() {
                            if word == "endswitch" {
                                break;
                            }
                            if found {
                                continue;
                            }
                            if word == "case" {
                                let case_value = source.next().unwrap().parse().unwrap();
                                if value == case_value {
                                    found = true;
                                    while let Some(word) = source.next() {
                                        if word == "break" {
                                            break;
                                        }
                                        match word {
                                            "print" => {
                                                let name = source.next().unwrap();
                                                match self.variables.get(name) {
                                                    Some(value) => println!("{}", value),
                                                    None => match self.arrays.get(name) {
                                                        Some(array) => {
                                                            for (index, &value) in
                                                                array.iter().enumerate()
                                                            {
                                                                println!(
                                                                    "{}[{}] = {}",
                                                                    name, index, value
                                                                );
                                                            }
                                                        }
                                                        None => match self.float.get(name) {
                                                            Some(value) => println!("{}", value),
                                                            None => {
                                                                println!("{}", self.strings[name])
                                                            }
                                                        },
                                                    },
                                                }
                                            }
                                            _ => continue,
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Lexeme::Print => {
                        if let Some(name) = words.next() {
                            match self.variables.get(name) {
                                Some(value) => writeln!(self.output_stream, "{}", value)?,
                                None => match self.arrays.get(name) {
                                    Some(array) => {
                                        for (index, &value) in array.iter().enumerate() {
                                            writeln!(
                                                self.output_stream,
                                                "{}[{}] = {}",
                                                name, index, value
                                            )?;
                                        }
                                    }
                                    None => match self.float.get(name) {
                                        Some(value) => writeln!(self.output_stream, "{}", value)?,
                                        None => match self.structs.get(name) {
                                            Some(_struct) => {
                                                for (key, value) in _struct.iter() {
                                                    writeln!(
                                                        self.output_stream,
                                                        "{}.{} = {}",
                                                        name, key, value
                                                    )?;
                                                }
                                            }
                                            None => {
                                                if let Some(output) = self.strings.get(name) {
                                                    writeln!(self.output_stream, "{}", output)?;
                                                } else {
                                                    return Err(io::Error::new(
                                                        io::ErrorKind::NotFound,
                                                        format!("Variable '{}' not found", name),
                                                    ));
                                                }
                                            }
                                        },
                                    },
                                },
                            }
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::UnexpectedEof,
                                "Expected variable name after print",
                            ));
                        }
                    }
                    Lexeme::If => {
                        let name = source.next().unwrap();
                        let comp = Comparison::from_str(source.next().unwrap());
                        let value = source.next().unwrap().parse().unwrap();
                        let mut executed = false;
                        let condition = match comp {
                            Comparison::Equal => self.variables[name] == value,
                            Comparison::NotEqual => self.variables[name] != value,
                            Comparison::LessThan => self.variables[name] < value,
                            Comparison::LessThanOrEqual => self.variables[name] <= value,
                            Comparison::GreaterThan => self.variables[name] > value,
                            Comparison::GreaterThanOrEqual => self.variables[name] >= value,
                        };
                        if condition {
                            while let Some(word) = source.next() {
                                if word == "else" || word == "end" {
                                    break;
                                }
                                match word {
                                    "var" => {
                                        let name = source.next().unwrap();
                                        let value = source.next().unwrap().parse().unwrap();
                                        self.variables.insert(name.to_owned(), value);
                                    }
                                    "end" => {
                                        break;
                                    }
                                    "print" => {
                                        let name = source.next().unwrap();
                                        writeln!(self.output_stream, "{}", self.variables[name])?;
                                    }
                                    "add" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() +=
                                            self.variables[name2];
                                    }
                                    "sub" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() -=
                                            self.variables[name2];
                                    }
                                    "mul" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() *=
                                            self.variables[name2];
                                    }
                                    "div" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() /=
                                            self.variables[name2];
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            while let Some(word) = source.next() {
                                if word == "end" {
                                    break;
                                }
                                if word == "else" {
                                    executed = true;
                                    while let Some(word) = source.next() {
                                        if word == "end" {
                                            break;
                                        }
                                        match word {
                                            "var" => {
                                                let name = source.next().unwrap();
                                                let value = source.next().unwrap().parse().unwrap();
                                                self.variables.insert(name.to_owned(), value);
                                            }
                                            "print" => {
                                                let name = source.next().unwrap();
                                                writeln!(
                                                    self.output_stream,
                                                    "{}",
                                                    self.variables[name]
                                                )?;
                                            }
                                            "add" => {
                                                let name1 = source.next().unwrap();
                                                let name2 = source.next().unwrap();
                                                *self.variables.get_mut(name1).unwrap() +=
                                                    self.variables[name2];
                                            }
                                            "sub" => {
                                                let name1 = source.next().unwrap();
                                                let name2 = source.next().unwrap();
                                                *self.variables.get_mut(name1).unwrap() -=
                                                    self.variables[name2];
                                            }
                                            "mul" => {
                                                let name1 = source.next().unwrap();
                                                let name2 = source.next().unwrap();
                                                *self.variables.get_mut(name1).unwrap() *=
                                                    self.variables[name2];
                                            }
                                            "div" => {
                                                let name1 = source.next().unwrap();
                                                let name2 = source.next().unwrap();
                                                *self.variables.get_mut(name1).unwrap() /=
                                                    self.variables[name2];
                                            }
                                            "end" => {
                                                break;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                if executed {
                                    break;
                                }
                            }
                        }
                    }
                    Lexeme::Loop => {
                        let name = source.next().unwrap();
                        let comp = Comparison::from_str(source.next().unwrap());
                        let value = source.next().unwrap().parse().unwrap();
                        while match comp {
                            Comparison::Equal => self.variables[name] == value,
                            Comparison::NotEqual => self.variables[name] != value,
                            Comparison::LessThan => self.variables[name] < value,
                            Comparison::LessThanOrEqual => self.variables[name] <= value,
                            Comparison::GreaterThan => self.variables[name] > value,
                            Comparison::GreaterThanOrEqual => self.variables[name] >= value,
                        } {
                            let mut inner_source = source.clone();
                            while let Some(word) = inner_source.next() {
                                if word == "end" {
                                    break;
                                }
                                match word {
                                    "var" => {
                                        let name = inner_source.next().unwrap();
                                        let value = inner_source.next().unwrap().parse().unwrap();
                                        self.variables.insert(name.to_owned(), value);
                                    }
                                    "print" => {
                                        let name = inner_source.next().unwrap();
                                        writeln!(self.output_stream, "{}", self.variables[name])?;
                                    }
                                    "add" => {
                                        let name1 = inner_source.next().unwrap();
                                        let name2 = inner_source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() +=
                                            self.variables[name2];
                                    }
                                    "sub" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() -=
                                            self.variables[name2];
                                    }
                                    "mul" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() *=
                                            self.variables[name2];
                                    }
                                    "div" => {
                                        let name1 = source.next().unwrap();
                                        let name2 = source.next().unwrap();
                                        *self.variables.get_mut(name1).unwrap() /=
                                            self.variables[name2];
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Lexeme::Add => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.variables[name1].add(self.variables[name2]);
                        self.variables.insert(name1.to_owned(), result);
                    }
                    Lexeme::Sub => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.variables[name1].sub(self.variables[name2]);
                        self.variables.insert(name1.to_owned(), result);
                    }
                    Lexeme::Mul => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.variables[name1].mul(self.variables[name2]);
                        self.variables.insert(name1.to_owned(), result);
                    }
                    Lexeme::Div => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.variables[name1].div(self.variables[name2]);
                        self.variables.insert(name1.to_owned(), result);
                    }
                    Lexeme::AddF => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.float[name1].add(self.float[name2]);
                        self.float.insert(name1.to_owned(), result);
                    }
                    Lexeme::SubF => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.float[name1].sub(self.float[name2]);
                        self.float.insert(name1.to_owned(), result);
                    }
                    Lexeme::MulF => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.float[name1].mul(self.float[name2]);
                        self.float.insert(name1.to_owned(), result);
                    }
                    Lexeme::DivF => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.float[name1].div(self.float[name2]);
                        self.float.insert(name1.to_owned(), result);
                    }
                    Lexeme::Sqrt => {
                        let name = source.next().unwrap();
                        self.variables
                            .insert(name.to_owned(), (self.variables[name] as f32).sqrt() as i32);
                    }
                    Lexeme::ABS => {
                        let name = source.next().unwrap();
                        self.variables
                            .insert(name.to_owned(), self.variables[name].abs());
                    }
                    Lexeme::POW => {
                        let name1 = source.next().unwrap();
                        let name2 = source.next().unwrap();
                        let result = self.variables[name1].pow(self.variables[name2] as u32);
                        self.variables.insert(name1.to_owned(), result);
                    }
                    Lexeme::End => {}
                    _ => panic!("Unknown command: {}", word),
                }
            }
        }
        Ok(self.output_stream.by_ref())
    }
    pub fn _call_function(&mut self, name: &str, parameters: &[i32]) -> Result<()> {
        let function = self.functions.get(name).unwrap();
        let mut interpreter = Interpreter::new();
        for (param_name, param_value) in function.parameters.iter().zip(parameters) {
            interpreter
                .variables
                .insert(param_name.to_owned(), *param_value);
        }
        interpreter.run(&function.code.join(" "))?;

        Ok(())
    }
}
