use std::collections::HashMap;

use bevy::ecs::event::EventWriter;

use crate::int::lexeme::*;
use crate::int::parser::interpreter_parser;
use crate::ui::resources::PrintEvent;

use super::native::build_native_fn_table;

pub struct Interpreter {
    // A stack of scopes; the last is the current environment.
    pub scopes: Vec<HashMap<String, Value>>,
    // Function definitions.
    pub functions: HashMap<String, FunctionDef>,
}

impl Interpreter {
    pub fn new() -> Self {
        let native_fns = build_native_fn_table();
        let mut interp = Interpreter {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
        };
        for (name, func) in native_fns.iter() {
            println!("Registering native fn: {}", name);
            interp.set_var(name.clone(), Value::Native(*func));
        }
        interp
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn set_var(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    // Look up a variable by searching from innermost to outermost scope.
    fn get_var(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    // Evaluate an expression.
    fn eval_expr(&self, expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Literal(v) => Some(v.clone()),
            Expr::Variable(name) => self.get_var(name),
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                match (left_val, right_val, op.as_str()) {
                    (Value::Int(l), Value::Int(r), "+") => Some(Value::Int(l + r)),
                    (Value::Int(l), Value::Int(r), "<") => Some(Value::Bool(l < r)),
                    // Extend with additional operators and type handling as needed.
                    _ => None,
                }
            }
        }
    }

    // Evaluate a function call.
    fn eval_function_call(
        &mut self,
        func: FunctionDef,
        arg_values: Vec<Value>,
        writer: &mut EventWriter<PrintEvent>,
    ) -> Option<Value> {
        self.push_scope();
        // Bind parameters.
        for (param, arg_val) in func.parameters.iter().zip(arg_values.into_iter()) {
            self.set_var(param.clone(), arg_val);
        }
        // Execute the function body.
        let mut ret_val = None;
        for s in &func.body {
            if let Some(v) = self.eval_statement(s, writer) {
                ret_val = Some(v);
                break;
            }
        }
        self.pop_scope();
        ret_val
    }

    // Evaluate a statement. Returns Some(value) if a return statement is encountered.
    fn eval_statement(
        &mut self,
        stmt: &Statement,
        writer: &mut EventWriter<PrintEvent>,
    ) -> Option<Value> {
        match stmt {
            Statement::VarDecl(var) => {
                self.set_var(var.name.clone(), var.value.clone());
                None
            }
            Statement::PrintExpr(expr) => {
                if let Some(val) = self.eval_expr(expr) {
                    // send event to bevy
                    writer.send(PrintEvent {
                        message: format!("{}", val),
                    });
                } else {
                    println!("error evaluating print statement");
                }
                None
            }
            Statement::ForLoop {
                var_name,
                start,
                end,
                body,
            } => {
                for i in *start..*end {
                    self.push_scope();
                    self.set_var(var_name.clone(), Value::Int(i));
                    for s in body {
                        if let Some(ret_val) = self.eval_statement(s, writer) {
                            self.pop_scope();
                            return Some(ret_val);
                        }
                    }
                    self.pop_scope();
                }
                None
            }
            Statement::While { condition, body } => {
                // Continue while condition evaluates to true.
                while let Some(Value::Bool(true)) = self.eval_expr(condition) {
                    for s in body {
                        if let Some(ret_val) = self.eval_statement(s, writer) {
                            return Some(ret_val);
                        }
                    }
                }
                None
            }
            Statement::Function {
                name,
                parameters,
                body,
            } => {
                self.functions.insert(
                    name.clone(),
                    FunctionDef {
                        parameters: parameters.clone(),
                        body: body.clone(),
                    },
                );
                println!("Defined function: {}", name);
                None
            }
            Statement::FunctionCall { name, arguments } => {
                let mut arg_values = Vec::new();
                for arg in arguments {
                    if let Some(val) = self.eval_expr(arg) {
                        arg_values.push(val);
                    } else {
                        println!("Error evaluating argument for function call.");
                        return None;
                    }
                }

                if let Some(Value::Native(func)) = self.get_var(name) {
                    println!("Calling native fn: {}", name); // debug
                    return func(arg_values, writer);
                }

                if let Some(func) = self.functions.get(name).cloned() {
                    self.eval_function_call(func, arg_values, writer)
                } else {
                    println!("Function '{}' not found!", name);
                    None
                }
            }
            Statement::Assignment { name, expr } => {
                if let Some(val) = self.eval_expr(expr) {
                    self.set_var(name.clone(), val);
                } else {
                    println!("Error evaluating assignment expression.");
                }
                None
            }
            Statement::Return(expr) => self.eval_expr(expr),
        }
    }

    fn eval_program(&mut self, stmts: &[Statement], writer: &mut EventWriter<PrintEvent>) {
        for stmt in stmts {
            if let Some(ret_val) = self.eval_statement(stmt, writer) {
                writer.send(PrintEvent {
                    message: format!("Returned : {}", ret_val),
                });
            }
        }
    }
}

// format display for printed value(remove the debug)
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Native(_) => write!(f, "[native function]"),
        }
    }
}

pub fn run(source: String, mut writer: EventWriter<PrintEvent>) {
    // test run using file with jw extension

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <source_file.jw>", args[0]);
    //     process::exit(1);
    // }
    // let filename = &args[1];
    // if !filename.ends_with(".jw") {
    //     eprintln!("Error: The file must have a .jw extension.");
    //     process::exit(1);
    // }
    // let source = fs::read_to_string(filename).unwrap_or_else(|err| {
    //     eprintln!("Error reading file {}: {}", filename, err);
    //     process::exit(1);
    // });

    // no touch 😡😡😡😡

    let stmts = match interpreter_parser::program(&source) {
        Ok(result) => result,
        Err(e) => {
            writer.send(PrintEvent {
                message: format!(" parse error : {:?}", e),
            });
            return;
        }
    };

    let mut interp = Interpreter::new();
    interp.eval_program(&stmts, &mut writer);
}
