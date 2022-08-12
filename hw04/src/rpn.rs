use std::result;
use std::io;
use rand;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax,
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

/// the stack data struct
pub struct Stack {
    data: Vec<Elt>,
}

/// Result alias for std::result::Result<T, Error>
pub type Result<T> = result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack {
            data: Vec::new(),
        }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        self.data.push(val);
        Ok(())
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        self.data.pop().map_or(Err(Error::Underflow), |v| Ok(v))
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                let a = self.pop();
                let b = self.pop();
        
                a.and_then(|a| b.and_then(|b| self.add(a, b)))
                 .and_then(|r| self.push(r))
            },
            Op::Neg => {
                // how to do this by replacing the last value of the vector?
                self.pop().and_then(|e| self.neg(e)).and_then(|r| self.push(r))
            },
            Op::Eq => {
                let a = self.pop();
                let b = self.pop();

                a.and_then(|a| b.and_then(|b| self.eq(a, b)))
                 .and_then(|r| self.push(r))
            },
            Op::Swap => {
                let a = self.pop();
                let b = self.pop();

                a.and_then(|a| b.and_then(|b| {
                    self.push(a).and_then(|_|self.push(b))
                }))
            },
            Op::Rand => {
                let a = self.pop();

                a.and_then(|a| self.rand(a))
                 .and_then(|r| self.push(r))
            },
            Op::Quit => Err(Error::Quit),
        }
    }

    fn add(&self, a: Elt, b: Elt) -> Result<Elt> {
        if let Elt::Int(x) = a {
            if let Elt::Int(y) = b {
                return Ok(Elt::Int(x + y));
            }
        }

        Err(Error::Type)
    }

    fn neg(&self, e: Elt) -> Result<Elt> {
        match e {
            Elt::Int(i) => Ok(Elt::Int(-i)),
            Elt::Bool(b) => Ok(Elt::Bool(!b)),
        }
    }

    fn eq(&self, a: Elt, b: Elt) -> Result<Elt> {
        match a {
            Elt::Int(ai) => {
                match b {
                    Elt::Int(bi) => Ok(Elt::Bool(ai == bi)),
                    _ => Err(Error::Type),
                }
            },
            Elt::Bool(ab) => {
                match b {
                    Elt::Bool(bb) => Ok(Elt::Bool(ab == bb)),
                    _ => Err(Error::Type),
                }
            }
        }
    }

    fn rand(&self, a: Elt) -> Result<Elt> {
        match a {
            Elt::Int(i) => Ok(Elt::Int((rand::random::<f32>() * i as f32) as i32)),
            _ => Err(Error::Type),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Eq);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}
