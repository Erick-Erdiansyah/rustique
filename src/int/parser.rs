use crate::int::lexeme::*;

peg::parser! {
  pub grammar interpreter_parser() for str {
      // Skip whitespace.
      rule _() = ([' ' | '\t' | '\n' | '\r'] / comment())* { () }

      // Single-line comment: matches '//' then any characters until a newline. and '/*comment*/' for multi-line comment
      rule comment() -> () = ("//" (!"\n" [_])* (("\n") / ![_])) { () } / ("/*" (!"*/" [_])* "*/") { () }

            // Parse an identifier.
      rule identifier() -> &'input str
          = s:$(['a'..='z'|'A'..='Z'] ['a'..='z'|'A'..='Z'|'0'..='9'|'_']*) { s }
          / expected!("identifier")

      // Parse an integer.
      rule int_value() -> i64
          = n:$(['0'..='9']+) { n.parse().unwrap() }

      // parse float
      rule float_value()-> f64
          = n:$((['0'..='9'] + "." ['0'..='9']+)) {n.parse().unwrap()}

      // parse string simple
      rule string_value()-> &'input str
          = "\"" s:$((!"\"" [_])*) "\"" { s }

      // Parse a boolean
      rule bool_value() -> bool
          = "true" { true } / "false" { false }

      rule value() -> Value
      = v:(
            n:float_value() { Value::Float(n) }
          / n:int_value() { Value::Int(n) }
          / s:string_value() { Value::Str(s.to_string()) }
          / b:bool_value()  { Value::Bool(b) }
        ) { v }

      // Parse a boolean literal.
      rule bool_literal() -> Expr
          = "true" { Expr::Literal(Value::Bool(true)) }
          / "false" { Expr::Literal(Value::Bool(false)) }

      // Parse a literal expression.
      rule literal_expr() -> Expr
          = n:int_value() { Expr::Literal(Value::Int(n)) }

      // Parse a variable expression.
      rule variable_expr() -> Expr
          = id:identifier() { Expr::Variable(id.to_string()) }

      // Parse a factor: bool literal, literal, variable, or parenthesized expression.
      rule factor() -> Expr
          = bool_literal()
          / literal_expr()
          / variable_expr()
          / "(" _ e:expr() _ ")" { e }

      // Parse addition expressions.
      rule add_expr() -> Expr
          = left:factor() _ "+" _ right:add_expr() { Expr::BinaryOp(Box::new(left), "+".to_string(), Box::new(right)) }
          / factor()

      // Parse relational expressions with '<'.
      rule rel_expr() -> Expr
          = left:add_expr() _ "<" _ right:rel_expr() { Expr::BinaryOp(Box::new(left), "<".to_string(), Box::new(right)) }
          / add_expr()

      // An expression is a relational expression.
      rule expr() -> Expr = rel_expr()

      // Parse a variable declaration: "var <id>:<type> = <int>;"
      rule var_decl() -> Statement
          = "var" _ id:identifier() _ ":" _ typ:identifier() _ "=" _ val:value() _ ";" {
              Statement::VarDecl({
                  Variable {
                      name: id.to_string(),
                      _type_annotation: typ.to_string(),
                      value: val,
                  }
              })
          }

      // Parse an assignment: "<id> = <expr>;"
      rule assignment() -> Statement
          = id:identifier() _ "=" _ e:expr() _ ";" {
              Statement::Assignment { name: id.to_string(), expr: e }
          }

      // Parse a print statement: "print(<id>);"
      rule print_stmt() -> Statement
          = "print" _ "(" _ id:identifier() _ ")" _ ";" {
              Statement::Print(id.to_string())
          }

      // Parse a for loop: "for <id> in <int>..<int> { <statements> }"
      rule for_loop() -> Statement
          = "for" _ id:identifier() _ "in" _ start:int_value() _ ".." _ end:int_value() _ "{" _
            stmts:(statement() ** _) _ "}" {
              Statement::ForLoop { var_name: id.to_string(), start, end, body: stmts }
          }

      // Parse a while loop: "while (<expr>) { <statements> }"
      rule while_loop() -> Statement
          = "while" _ "(" _ cond:expr() _ ")" _ "{" _
            stmts:(statement() ** _) _ "}" {
              Statement::While { condition: cond, body: stmts }
          }

      // Parse a return statement: "return <expr>;"
      rule return_stmt() -> Statement
          = "return" _ e:expr() _ ";" { Statement::Return(e) }

      // Parse a function declaration:
      // "fn <id>(<params>) { <statements> }"
      // Parameters are a comma-separated list of identifiers.
      rule function_decl() -> Statement
          = "fn" _ name:identifier() _ "(" _ params:(identifier() ** ("," _))? _ ")" _ "{" _
            stmts:(statement() ** _) _ "}" {
                Statement::Function {
                    name: name.to_string(),
                    parameters: params.map(|v| v.into_iter().map(|s| s.to_string()).collect())
                                       .unwrap_or_else(Vec::new),
                    body: stmts
                }
            }

      // Parse a function call: "<id>(<args>);"
      // Arguments are a comma-separated list of expressions.
      rule function_call() -> Statement
          = name:identifier() _ "(" _ args:(expr() ** ("," _))? _ ")" _ ";" {
              Statement::FunctionCall { name: name.to_string(), arguments: args.unwrap_or(vec![]) }
          }

      // A statement can be one of several alternatives.
      rule statement() -> Statement
          = function_decl()
          / return_stmt()
          / var_decl()
          / assignment()
          / print_stmt()
          / for_loop()
          / while_loop()
          / function_call()

      // A program is a series of statements.
      pub rule program() -> Vec<Statement>
          = _ stmts:(statement() ** _) _ { stmts }
  }
}
