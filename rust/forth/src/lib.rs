use std::result::Result;
use std::{collections::HashMap, iter};

pub type Value = i32;

// Sequence of words that make up a new word
type WordDefinition = Vec<Word>;
// If word definitions get overwritten they are appended
type WordHistory = Vec<WordDefinition>;

#[derive(Debug, PartialEq)]
pub struct WordAddress {
    name: String,
    // index into the WordHistory
    history_index: usize,
}

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

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

fn interpret(
    stack: &mut Vec<Value>,
    word_env: &HashMap<String, Vec<Vec<Word>>>,
    word: &Word,
) -> Result<(), Error> {
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

        Word::Call(WordAddress {
            name,
            history_index: index,
        }) => match word_env.get(name) {
            None => return Err(Error::UnknownWord),
            Some(word_history) => word_history[*index]
                .as_slice()
                .iter()
                .try_for_each(|word| interpret(stack, word_env, word))?,
        },
    }

    Ok(())
}
fn tokenize(input: &str) -> impl Iterator<Item = Result<Token, Error>> + '_ {
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
                        Err(_) => Err(Error::InvalidWord),
                    }
                } else {
                    Ok(Token::WordName(s))
                }
            }
        })
}

fn parse<'a>(
    // TODO: could make this truly streaming by propagating immutable, minimal necessary history
    // with each parsed word
    word_env: &'a mut HashMap<String, WordHistory>,
    input: &'a str,
) -> impl Iterator<Item = Result<Word, Error>> + 'a {
    let mut iter = tokenize(input);

    // outer option -> iterator has next
    // inner option -> we returned a word or error. Definitions only mutate the word_env
    // result -> word creation was successful
    iter::from_fn(move || match iter.next() {
        Some(Ok(Token::StartWordDef)) => match parse_word_definition(&mut iter, word_env) {
            Ok(()) => Some(None),
            Err(error) => Some(Some(Err(error))),
        },
        Some(Ok(Token::EndWordDef)) => Some(Some(Err(Error::InvalidWord))),
        Some(Ok(Token::WordName(name))) => Some(Some(parse_word_name(name, word_env))),
        Some(Ok(Token::Number(num))) => Some(Some(Ok(Word::Push(num)))),
        Some(Err(error)) => Some(Some(Err(error))),
        None => None,
    })
    // throw all definition outputs (None) away
    // .filter_map(|x| x)
    .flatten()
    .fuse()
}

fn parse_word_name(
    name: String,
    word_env: &mut HashMap<String, WordHistory>,
) -> Result<Word, Error> {
    match get_latest_word_address(name, word_env) {
        None => Err(Error::UnknownWord),
        Some(word_address) => Ok(Word::Call(word_address)),
    }
}

fn parse_word_definition<T>(
    iter: &mut T,
    word_env: &mut HashMap<String, WordHistory>,
) -> Result<(), Error>
where
    T: Iterator<Item = Result<Token, Error>>,
{
    let name = match iter.next() {
        Some(Ok(Token::WordName(name))) => name,
        None | Some(_) => return Err(Error::InvalidWord),
    };

    let mut is_in_definition = true;
    let mut words = vec![];
    while is_in_definition {
        match iter.next() {
            Some(Ok(Token::StartWordDef)) => return Err(Error::InvalidWord),

            Some(Ok(Token::EndWordDef)) => {
                is_in_definition = false;
                if words.is_empty() {
                    return Err(Error::InvalidWord);
                }
            }

            Some(Err(error)) => return Err(error),

            None => return Err(Error::InvalidWord),

            Some(Ok(Token::Number(num))) => words.push(Word::Push(num)),

            Some(Ok(Token::WordName(name))) => match get_latest_word_address(name, word_env) {
                None => return Err(Error::UnknownWord),
                Some(word_address) => words.push(Word::Call(word_address)),
            },
        }
    }

    let word_history = word_env.entry(name).or_insert(vec![]);
    word_history.push(words);

    Ok(())
}

fn get_latest_word_address(
    name: String,
    word_env: &mut HashMap<String, WordHistory>,
) -> Option<WordAddress> {
    word_env
        .get(&name)
        .map(|word_history| word_history.len())
        .map(|history_len| WordAddress {
            name,
            history_index: history_len - 1,
        })
}
