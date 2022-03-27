use std::marker::PhantomData;
use std::result::Result;
use std::{collections::HashMap, iter, panic};

use itertools::Itertools;

pub type Value = i32;

#[derive(Debug, Clone)]
pub struct Forth {
    stack: Vec<Value>,
    user_defined_words: HashMap<String, Vec<Vec<Word>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Word {
    Push(Value),
    // Plus,
    // Minus,
    // Multiply,
    // Divide,
    // Duplicate,
    // Drop,
    // Swap,
    // Over,
    ReferenceNewWord(String, usize),
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
        let word_stack = dbg!(self.parse(input))?;

        dbg!(&word_stack);
        dbg!(self.interpret(&word_stack))
    }

    fn parse(&self, input: &str) -> Result<Vec<Word>, Error> {
        let mut iter = input
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .peekable();

        iter::from_fn(move || {
            if let Some(s) = iter.next() {
                let s = s.to_ascii_lowercase();
                if s.chars().all(|c| c.is_ascii_digit()) {
                    Some(Ok(Word::Push(s.parse::<i32>().unwrap())))
                } else {
                    let idx = self.user_defined_words.get(&s).map_or(0, |x| x.len() - 1);

                    match s.as_str() {
                        "+" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "-" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "*" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "/" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "dup" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "drop" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "swap" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        "over" => Some(Ok(Word::ReferenceNewWord(s, idx))),
                        ":" => {
                            let word_name = match iter.next() {
                                None => return Some(Err(Error::InvalidWord)),
                                Some(word_name)
                                    if word_name.chars().all(|c| c.is_ascii_digit()) =>
                                {
                                    return Some(Err(Error::InvalidWord))
                                }
                                Some(word_name) => word_name.to_ascii_lowercase(),
                            };

                            let word_definition = iter
                                .by_ref()
                                .peeking_take_while(|s| *s != ";")
                                .flat_map(|s| iter::once(s).chain(iter::once("\n")))
                                .collect::<String>()
                                .trim()
                                .to_string();

                            if Some(";") != dbg!(iter.next()) {
                                dbg!(input);
                                return Some(Err(Error::InvalidWord));
                            }

                            if word_definition.is_empty() {
                                return Some(Err(Error::InvalidWord));
                            }

                            dbg!(&word_definition);

                            match self.parse(word_definition.as_str()) {
                                Ok(words) => Some(Ok(Word::DefineNewWord(word_name, dbg!(words)))),
                                Err(err) => Some(Err(err)),
                            }
                        }
                        word_name => {
                            dbg!(word_name);
                            Some(Ok(Word::ReferenceNewWord(word_name.to_owned(), idx)))
                        }
                    }
                }
            } else {
                None
            }
        })
        .fuse()
        .collect()
    }

    fn interpret(&mut self, word_stack: &Vec<Word>) -> Result<(), Error> {
        for word in word_stack.iter() {
            match word {
                Word::Push(num) => {
                    dbg!(&self.user_defined_words);
                    dbg!(&self.stack);
                    self.stack.push(*num);
                }

                Word::DefineNewWord(name, words) => {
                    let definitions = self
                        .user_defined_words
                        .entry(name.clone())
                        .or_insert(vec![]);

                    definitions.push(words.to_vec());
                    // dbg!(self.interpret(words.to_owned()))?;
                }

                Word::ReferenceNewWord(name, idx) => {
                    dbg!(&self.user_defined_words);
                    dbg!(&self.stack);

                    match self.user_defined_words.get(name) {
                        Some(words) => dbg!(self.interpret(&words[*idx].to_vec()))?,
                        None => self.interpret_built_in(&word)?,
                    }
                }

                _ => panic!(),
            }
        }

        Ok(())
    }

    fn interpret_built_in(&mut self, word: &Word) -> Result<(), Error> {
        dbg!(&word);
        match word {
            Word::ReferenceNewWord(name, _) if name.as_str() == "+" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b + a),
                    _ => return Err(Error::StackUnderflow),
                }
            }

            Word::ReferenceNewWord(name, _) if name.as_str() == "-" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b - a),
                    _ => return Err(Error::StackUnderflow),
                }
            }

            Word::ReferenceNewWord(name, _) if name.as_str() == "*" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b * a),
                    _ => return Err(Error::StackUnderflow),
                }
            }

            Word::ReferenceNewWord(name, _) if name.as_str() == "/" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(0), Some(_)) => return Err(Error::DivisionByZero),
                    (Some(a), Some(b)) => self.stack.push(b / a),
                    _ => return Err(Error::StackUnderflow),
                }
            }

            Word::ReferenceNewWord(name, _) if name.as_str() == "dup" => match self.stack.last() {
                Some(&a) => self.stack.push(a),
                _ => return Err(Error::StackUnderflow),
            },

            Word::ReferenceNewWord(name, _) if name.as_str() == "drop" => match self.stack.pop() {
                Some(_) => (),
                _ => return Err(Error::StackUnderflow),
            },

            Word::ReferenceNewWord(name, _) if name.as_str() == "swap" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                }
            }

            Word::ReferenceNewWord(name, _) if name.as_str() == "over" => {
                match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => {
                        self.stack.push(b);
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                    _ => return Err(Error::StackUnderflow),
                }
            }
            _ => return Err(Error::UnknownWord),
        }

        Ok(())
    }
}
