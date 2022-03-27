use std::result::Result;
use std::{collections::HashMap, iter};

pub type Value = i32;

type WordDefinition = Vec<Word>;
type WordHistory = Vec<WordDefinition>;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    word_env: HashMap<String, WordHistory>,
}

#[derive(Debug, PartialEq)]
enum Token {
    StartWordDef,
    EndWordDef,
    WordName(String),
    Number(i32),
}

#[derive(Debug, PartialEq)]
pub struct Definition {
    name: String,
    words: Vec<Word>,
}

#[derive(Debug, PartialEq)]
pub struct WordAddress {
    name: String,
    index: usize,
}

#[derive(Debug, PartialEq)]
pub enum Word {
    Plus,
    Minus,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over,
    Push(Value),
    Call(WordAddress),
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
            word_env: HashMap::from([
                ("+".to_string(), vec![vec![Word::Plus]]),
                ("-".to_string(), vec![vec![Word::Minus]]),
                ("*".to_string(), vec![vec![Word::Multiply]]),
                ("/".to_string(), vec![vec![Word::Divide]]),
                ("dup".to_string(), vec![vec![Word::Duplicate]]),
                ("drop".to_string(), vec![vec![Word::Drop]]),
                ("swap".to_string(), vec![vec![Word::Swap]]),
                ("over".to_string(), vec![vec![Word::Over]]),
            ]),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        let word_stack = parse(&mut self.word_env, input).collect::<Result<Vec<Word>, Error>>()?;

        word_stack
            .iter()
            .try_for_each(|word| interpret(&mut self.stack, &self.word_env, word))
    }
}

fn interpret(
    stack: &mut Vec<Value>,
    word_env: &HashMap<String, Vec<Vec<Word>>>,
    word: &Word,
) -> Result<(), Error> {
    dbg!(&word);
    dbg!(&word_env);
    match word {
        Word::Push(num) => stack.push(*num),
        Word::Plus => match (stack.pop(), stack.pop()) {
            (Some(a), Some(b)) => stack.push(b + a),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Minus => match (stack.pop(), stack.pop()) {
            (Some(a), Some(b)) => stack.push(b - a),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Multiply => match (stack.pop(), stack.pop()) {
            (Some(a), Some(b)) => stack.push(b * a),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Divide => match (stack.pop(), stack.pop()) {
            (Some(0), Some(_)) => return Err(Error::DivisionByZero),
            (Some(a), Some(b)) => stack.push(b / a),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Duplicate => match stack.last() {
            Some(&a) => stack.push(a),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Drop => match stack.pop() {
            Some(_) => (),
            _ => return Err(Error::StackUnderflow),
        },

        Word::Swap => match (stack.pop(), stack.pop()) {
            (Some(a), Some(b)) => {
                stack.push(a);
                stack.push(b);
            }
            _ => return Err(Error::StackUnderflow),
        },

        Word::Over => match (stack.pop(), stack.pop()) {
            (Some(a), Some(b)) => {
                stack.push(b);
                stack.push(a);
                stack.push(b);
            }
            _ => return Err(Error::StackUnderflow),
        },

        Word::Call(WordAddress { name, index }) => match word_env.get(name) {
            None => return Err(Error::UnknownWord),
            Some(word_history) => word_history[*index]
                .as_slice()
                .iter()
                .try_for_each(move |word| interpret(stack, word_env, word))?,
        },
    }

    Ok(())
}

fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Result<Token, Error>> + 'a {
    input
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_ascii_lowercase())
        .map(|s| match s.as_str() {
            ":" => Ok(Token::StartWordDef),
            ";" => Ok(Token::EndWordDef),
            _ => {
                if s.chars().all(|c| c.is_ascii_digit()) {
                    match s.parse::<i32>() {
                        Ok(num) => Ok(Token::Number(num)),
                        Err(err) => Err(Error::InvalidWord),
                    }
                } else {
                    Ok(Token::WordName(s))
                }
            }
        })
}

fn parse<'a>(
    word_env: &'a mut HashMap<String, Vec<Vec<Word>>>,
    input: &'a str,
) -> impl Iterator<Item = Result<Word, Error>> + 'a {
    let mut iter = tokenize(input);

    iter::from_fn(move || match iter.next() {
        Some(Ok(Token::StartWordDef)) => {
            let name = match iter.next() {
                Some(Ok(Token::WordName(name))) => name,
                None | Some(_) => return Some(Some(Err(Error::InvalidWord))),
            };
            let index = word_env.get(&name).map_or(0, |x| x.len());
            // let word_address = WordAddress { name, index };

            let mut is_in_definition = true;
            let mut words = vec![];
            while is_in_definition {
                match iter.next() {
                    Some(Ok(Token::StartWordDef)) => {
                        return Some(Some(Err(Error::InvalidWord)));
                    }
                    Some(Ok(Token::EndWordDef)) => {
                        dbg!(&words);
                        is_in_definition = false;
                        if words.is_empty() {
                            return Some(Some(Err(Error::InvalidWord)));
                        }
                    }
                    Some(Ok(Token::Number(num))) => {
                        words.push(Word::Push(num));
                    }
                    Some(Ok(Token::WordName(name))) => {
                        let index_opt = word_env.get(&name).map(|x| x.len()).map(|x| x - 1);

                        match index_opt {
                            None => return Some(Some(Err(Error::UnknownWord))),
                            Some(index) => words.push(Word::Call(WordAddress { name, index })),
                        }
                    }

                    Some(Err(error)) => return Some(Some(Err(error))),
                    None => return Some(Some(Err(Error::InvalidWord))),
                }
            }

            let word_history = word_env.entry(name).or_insert(vec![]);
            word_history.push(words);

            Some(None)
        }
        Some(Ok(Token::EndWordDef)) => return Some(Some(Err(Error::InvalidWord))),
        Some(Ok(Token::WordName(name))) => {
            let index_opt = word_env.get(&name).map(|x| x.len()).map(|x| x - 1);

            match index_opt {
                None => return Some(Some(Err(Error::UnknownWord))),
                Some(index) => Some(Some(Ok(Word::Call(WordAddress { name, index })))),
            }
            // let index = word_env.get(&name).map_or(0, |x| x.len());
            // let word_address = WordAddress { name, index };
            // Some(Ok(Word::Call(word_address)))
        }
        Some(Ok(Token::Number(num))) => Some(Some(Ok(Word::Push(num)))),
        Some(Err(error)) => return Some(Some(Err(error))),
        None => None,
    })
    .filter_map(|x| x)
    .fuse()
}
