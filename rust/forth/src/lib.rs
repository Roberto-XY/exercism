use std::result::Result;
use std::{collections::HashMap, iter, panic};

pub type Value = i32;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    user_defined_words: HashMap<String, Vec<Word>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Word {
    Push(Value),
    Plus,
    Minus,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over,
    ReferenceNewWord(String),
    DefineNewWord(String, Vec<Word>),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Self {
        Forth {
            stack: vec![],
            user_defined_words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let word_stack = parse(input);

        dbg!(&word_stack);
        dbg!(self.interpret(word_stack))
    }

    fn interpret(&mut self, word_stack: Vec<Word>) -> Result<(), Error> {
        for word in word_stack.iter() {
            match word {
                Word::Push(num) => {
                    dbg!(&self.user_defined_words);
                    dbg!(&self.stack);
                    self.stack.push(*num);
                }
                Word::Plus => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b + a),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Minus => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b - a),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Multiply => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b * a),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Divide => match (self.stack.pop(), self.stack.pop()) {
                    (Some(0), Some(_)) => return Err(Error::DivisionByZero),
                    (Some(a), Some(b)) => self.stack.push(b / a),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Duplicate => match self.stack.last() {
                    Some(&a) => self.stack.push(a),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Drop => match self.stack.pop() {
                    Some(_) => (),
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Swap => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                },

                Word::Over => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(b);
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                },

                Word::DefineNewWord(name, words) => {
                    self.user_defined_words
                        .insert(name.clone(), words.to_owned());
                    dbg!(self.interpret(words.to_owned()))?;
                }

                Word::ReferenceNewWord(name) => {
                    dbg!(&self.user_defined_words);
                    dbg!(&self.stack);

                    let words = self.user_defined_words.get(name).unwrap().to_owned();
                    dbg!(self.interpret(words))?
                }
            }
        }

        Ok(())
    }
}

fn parse(input: &str) -> Vec<Word> {
    let mut iter = input.split_ascii_whitespace().filter(|s| !s.is_empty());

    iter::from_fn(move || {
        if let Some(s) = iter.next() {
            let s = s.to_ascii_lowercase();
            if s.chars().all(|c| c.is_ascii_digit()) {
                Some(Word::Push(s.parse::<i32>().unwrap()))
            } else {
                let word = match s.as_str() {
                    "+" => Word::Plus,
                    "-" => Word::Minus,
                    "*" => Word::Multiply,
                    "/" => Word::Divide,
                    "dup" => Word::Duplicate,
                    "drop" => Word::Drop,
                    "swap" => Word::Swap,
                    "over" => Word::Over,
                    ":" => {
                        let mut word_name = iter.next().unwrap().to_string();
                        word_name.make_ascii_lowercase();

                        let word_definition = iter
                            .by_ref()
                            .take_while(|s| *s != ";")
                            .flat_map(|s| iter::once(s).chain(iter::once("\n")))
                            .collect::<String>();

                        dbg!(&word_definition);

                        Word::DefineNewWord(word_name, dbg!(parse(word_definition.as_str())))
                    }
                    word_name => {
                        dbg!(word_name);
                        Word::ReferenceNewWord(word_name.to_owned())
                    }
                };
                Some(word)
            }
        } else {
            None
        }
    })
    .fuse()
    .collect()
}
