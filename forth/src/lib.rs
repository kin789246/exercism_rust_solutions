use std::collections::HashMap;
use std::rc::Rc;
pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
type Func = Rc<dyn Fn(&mut Forth) -> Result>;

pub struct Forth {
    m_stack: Vec<Value>,
    m_words: HashMap<String, Func>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone)]
enum Token {
    Number(Value),
    Command(Func),
}

impl Forth {
    pub fn new() -> Forth {
        let mut words: HashMap<String, Func> = HashMap::new();
        words.insert("DUP".to_string(), Rc::new(Self::dup));
        words.insert("DROP".to_string(), Rc::new(Self::drop));
        words.insert("SWAP".to_string(), Rc::new(Self::swap));
        words.insert("OVER".to_string(), Rc::new(Self::over));
        words.insert("+".to_string(), Rc::new(Self::add));
        words.insert("-".to_string(), Rc::new(Self::subtract));
        words.insert("*".to_string(), Rc::new(Self::multiply));
        words.insert("/".to_string(), Rc::new(Self::divide));

        Forth { m_stack: Vec::new(), m_words: words }

    }

    pub fn stack(&self) -> &[Value] {
        &self.m_stack[..]
    }

    pub fn eval(&mut self, input: &str) -> Result {
        match self.compile(input) {
            Ok(tokens) => self.execute(&tokens),
            Err(e) => Err(e),
        }
    }

    fn execute(&mut self, tokens: &Vec<Token>) -> Result {
        for token in tokens.iter() {
            match token {
                Token::Number(n) => self.m_stack.push(*n),
                Token::Command(f) => f(self)?,
            }
        }
        Ok(())
    }

    fn dup(&mut self) -> Result {
        match self.m_stack.last() {
            Some(a) => Ok(self.m_stack.push(*a)),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn drop(&mut self) -> Result {
        match self.m_stack.pop() {
            Some(_a) => Ok(()),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn swap(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => {
                self.m_stack.push(a);
                self.m_stack.push(b);
                Ok(())
            },
            _ => Err(Error::StackUnderflow),
        }
    }
    
    fn over(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => {
                self.m_stack.push(b);
                self.m_stack.push(a);
                self.m_stack.push(b);
                Ok(())
            },
            _ => Err(Error::StackUnderflow),
        }
    }

    fn add(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => Ok(self.m_stack.push(a + b)),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn subtract(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => Ok(self.m_stack.push(b - a)),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn multiply(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => Ok(self.m_stack.push(b * a)),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn divide(&mut self) -> Result {
        match (self.m_stack.pop(), self.m_stack.pop()) {
            (Some(a), Some(b)) => {
                if a == 0 {
                    return Err(Error::DivisionByZero);
                }
                Ok(self.m_stack.push(b / a))
            },
            _ => Err(Error::StackUnderflow),
        }
    }

    fn compile(&mut self, input: &str) -> std::result::Result<Vec<Token>, Error> {
        let mut compiled: Vec<Token> = Vec::new();
        let input = input.to_uppercase();
        let mut tokens = input.split_whitespace().into_iter();
        'outer: while let Some(token) = tokens.next() {
            match token {
                ":" => {
                    let mut d_str: Vec<String> = Vec::new();
                    while let Some(def_token) = tokens.next() {
                        if def_token == ";" {
                            if d_str.len() >= 2 {
                                if let Ok(_) = d_str[0].parse::<Value>() {
                                    return Err(Error::InvalidWord);
                                }
                                let cmd = d_str[1..d_str.len()].join(" ");
                                if let Ok(tokens) = self.compile(&cmd) {
                                    let f: Func = Rc::new(move |s| s.execute(&tokens));
                                    self.m_words.insert(d_str[0].to_string(), f);
                                }
                                continue 'outer;
                            }
                            else {
                                return Err(Error::InvalidWord);
                            }
                        }
                        d_str.push(def_token.to_string());
                    }
                    return Err(Error::InvalidWord);
                },
                _ => {
                    if let Ok(s) = token.parse::<Value>() {
                        compiled.push(Token::Number(s));
                        continue;
                    }
                    if let Some(w) = self.m_words.get(token) {
                        compiled.push(Token::Command(w.clone()));
                    }
                    else {
                        return Err(Error::UnknownWord);
                    }
                },
            }
        }

        Ok(compiled)
    }
}
