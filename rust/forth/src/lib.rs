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
    ReferenceDefined(String),
    DefineWord(String, Vec<Word>),
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

        // let acc: &mut Vec<i32> = self.stack.as_mut();

        Ok(())
    }

    fn interpret(&mut self, word_stack: Vec<Word>) -> Result<(), Error> {
        word_stack.iter().try_fold(&mut self.stack, |acc, word| {
            match word {
                Word::Push(num) => Ok(acc.push(*num)),

                Word::Plus => match (acc.pop(), acc.pop()) {
                    (Some(a), Some(b)) => Ok(acc.push(b + a)),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Minus => match (acc.pop(), acc.pop()) {
                    (Some(a), Some(b)) => Ok(acc.push(b - a)),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Multiply => match (acc.pop(), acc.pop()) {
                    (Some(a), Some(b)) => Ok(acc.push(b * a)),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Divide => match (acc.pop(), acc.pop()) {
                    (Some(0), Some(_)) => Err(Error::DivisionByZero),
                    (Some(a), Some(b)) => Ok(acc.push(b / a)),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Duplicate => match acc.last() {
                    Some(&a) => Ok(acc.push(a)),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Drop => match acc.pop() {
                    Some(_) => Ok(()),
                    _ => Err(Error::StackUnderflow),
                },

                Word::Swap => match (acc.pop(), acc.pop()) {
                    (Some(a), Some(b)) => {
                        acc.push(a);
                        acc.push(b);
                        Ok(())
                    }
                    _ => Err(Error::StackUnderflow),
                },

                Word::Over => match (acc.pop(), acc.pop()) {
                    (Some(a), Some(b)) => {
                        acc.push(b);
                        acc.push(a);
                        acc.push(b);
                        Ok(())
                    }
                    _ => Err(Error::StackUnderflow),
                },

                Word::DefineWord(name, words) => {
                    self.user_defined_words.insert(name.clone(), words.to_vec());
                    Ok(())
                }

                Word::ReferenceDefined(name) => {
                    let user_defined_words = &self.user_defined_words;
                    let words = user_defined_words.get(name).unwrap();
                    Self::interpret(self, words.to_vec())
                }
            }
            .map(|_| acc)
        })?;

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

                        Word::DefineWord(word_name, dbg!(parse(word_definition.as_str())))
                    }
                    word_name => {
                        dbg!(word_name);
                        Word::ReferenceDefined(word_name.to_owned())
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

fn parse2(input: &str) -> Vec<Word> {
    input
        .split_ascii_whitespace()
        .map(|s| {
            let s = s.to_ascii_lowercase();
            if s.chars().all(|c| c.is_ascii_digit()) {
                Word::Push(s.parse::<i32>().unwrap())
            } else {
                match s.as_str() {
                    "+" => Word::Plus,
                    "-" => Word::Minus,
                    "*" => Word::Multiply,
                    "/" => Word::Divide,
                    "dup" => Word::Duplicate,
                    "drop" => Word::Drop,
                    "swap" => Word::Swap,
                    "over" => Word::Over,
                    //  str => {
                    //     match str.chars().collect::<Vec<_>>().as_slice() {
                    //         [':']
                    //      }
                    //  }
                    _ => panic!("xD"),
                }
            }
        })
        .collect::<Vec<_>>()
}
