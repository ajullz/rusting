use std::io::{self, Write};

use rpn::{self, Stack};

/// Start& a read-eval-print loop, which runs until an error or `quit`.
pub fn read_eval_print_loop() -> rpn::Result<()> {
    // Create a stack to work on.
    let mut stack = Stack::new();

    loop {
        // Print a user input prompt.
        print!("> ");
        io::stdout().flush().map_err(rpn::Error::IO)?;

        // Read from stdin into a String, 
        let mut user_input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut user_input) {
            Ok(_) => return evaluate_line(&mut stack, &user_input),
            Err(e) => return rpn::Result::Err(rpn::Error::IO(e)),
        }
    }
}

fn get_value(token: &str) -> Result<rpn::Elt, rpn::Error> {
    // for me to understand what is happening here!!
    // 1. token.parse returns a Result
    // 2. that result calls its method map
    // 2.1      if result was ok(t), call the function given to map -> f(t)
    // 2.2      else return a Result as Err
    // 3. on the result from map, call its or method
    // 3.1      if the result from map was ok, then forward the ok
    // 3.2      else return the result provided to or, which is the parse.map to bool
    // 4. on the result from the or(parse.map), call again or
    token.parse::<i32>()
        .map(|v| rpn::Elt::Int(v))
        .or(token.parse::<bool>().map(|v| rpn::Elt::Bool(v)))
        .or(Err(rpn::Error::Syntax))
}

fn get_operation(token: &str) -> rpn::Result<rpn::Op> {
    match token {
        "+" => rpn::Result::Ok(rpn::Op::Add),
        "~" => rpn::Result::Ok(rpn::Op::Neg),
        "<->" => rpn::Result::Ok(rpn::Op::Swap),
        "=" => rpn::Result::Ok(rpn::Op::Eq),
        "#" => rpn::Result::Ok(rpn::Op::Rand),
        "quit" => rpn::Result::Ok(rpn::Op::Quit),
        _ => rpn::Result::Err(rpn::Error::Syntax),
    }
}

fn evaluate_line(stack: &mut Stack, buf: &String) -> rpn::Result<()> {
    // Create an iterator over the tokens.
    let tokens = buf.trim().split_whitespace();

    let mut result = Err(rpn::Error::Syntax);
    for t in tokens {
        if let Ok(op) = get_operation(t) {
            result = stack.eval(op);
        }
        else if let Ok(value) = get_value(t) {
            result = stack.push(value);
        }
        else {
            return Err(rpn::Error::Syntax);
        }
    }

    println!("Result: {:?}", stack.pop());
    result
}

#[cfg(test)]
mod tests {
    use rpn::{Stack, Error, Elt};
    use parser::evaluate_line;

    #[test]
    fn test_evaluate_line_bool() {
        let mut stack = Stack::new();
        let s = "true".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_int() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_plus() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "13".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "+".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(25));
    }

    #[test]
    fn test_evaluate_line_neg() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "~".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_evaluate_line_swap() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "<->".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
        assert_eq!(stack.pop().unwrap(), Elt::Int(15));
    }

    #[test]
    fn test_evaluate_line_eq() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "=".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_rand() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "#".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let res = stack.pop();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res >= Elt::Int(0));
        assert!(res < Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_quit() {
        let mut stack = Stack::new();
        let s = "quit".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else { assert!(false); }
    }

    #[test]
    fn test_evaluate_line_bad_parse() {
        let mut stack = Stack::new();
        let s = "~false".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Syntax) = res {
        } else { assert!(false); }
    }
}
