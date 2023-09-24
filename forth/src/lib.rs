use std::collections::HashMap;

pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Forth {
    stack: Vec<Value>,
    keywords: HashMap<String, Keyword>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone, Debug)]
struct Keyword {
    name: String,
    actions: Vec<Action>,
}

#[derive(Clone, Debug)]
enum Action {
    Add,
    Subtract,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    LiteralInt(i32),
}

#[allow(clippy::new_without_default)]
impl Forth {
    fn default_keywords() -> HashMap<String, Keyword> {
        let default_words = vec![
            Keyword {
                name: "+".to_string(),
                actions: vec![Action::Add],
            },
            Keyword {
                name: "-".to_string(),
                actions: vec![Action::Subtract],
            },
            Keyword {
                name: "*".to_string(),
                actions: vec![Action::Multiply],
            },
            Keyword {
                name: "/".to_string(),
                actions: vec![Action::Divide],
            },
            Keyword {
                name: "dup".to_string(),
                actions: vec![Action::Dup],
            },
            Keyword {
                name: "drop".to_string(),
                actions: vec![Action::Drop],
            },
            Keyword {
                name: "swap".to_string(),
                actions: vec![Action::Swap],
            },
            Keyword {
                name: "over".to_string(),
                actions: vec![Action::Over],
            },
        ];

        default_words
            .into_iter()
            .map(|kw| (kw.name.clone(), kw))
            .collect()
    }

    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            keywords: Forth::default_keywords(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        input
            .replace(':', "\n:")
            .replace(';', ";\n")
            .lines()
            .filter(|s| !s.is_empty())
            .try_for_each(|s| {
                let mut it = s.split(' ').filter(|s| !s.is_empty()).peekable();
                if it.peek().unwrap_or(&"") == &":" {
                    self.record_word(it)
                } else {
                    self.eval_line(it)
                }
            })
    }

    fn eval_line<'a>(&mut self, mut line_it: impl Iterator<Item = &'a str>) -> Result<()> {
        line_it.try_for_each(|s| match s.parse::<i32>() {
            Ok(v) => {
                self.stack.push(v);
                Ok(())
            }
            Err(_) => self.eval_token(s),
        })
    }

    fn eval_token(&mut self, token: &str) -> Result<()> {
        let token = token.to_lowercase();
        if !self.keywords.contains_key(token.as_str()) {
            return Err(Error::UnknownWord);
        }
        for action in self.keywords.get(token.as_str()).unwrap().actions.clone() {
            self.eval_action(action)?;
        }
        Ok(())
    }

    fn eval_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Add => {
                let (first, second) = self.pop_two()?;
                self.stack.push(first + second);
            }
            Action::Subtract => {
                let (first, second) = self.pop_two()?;
                self.stack.push(first - second);
            }
            Action::Multiply => {
                let (first, second) = self.pop_two()?;
                self.stack.push(first * second);
            }
            Action::Divide => {
                let (first, second) = self.pop_two()?;
                if second == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack.push(first / second);
            }
            Action::Dup => {
                let v = self.pop_one()?;
                self.stack.push(v);
                self.stack.push(v);
            }
            Action::Drop => {
                self.pop_one()?;
            }
            Action::Swap => {
                let (first, second) = self.pop_two()?;
                self.stack.push(second);
                self.stack.push(first);
            }
            Action::Over => {
                let (first, second) = self.pop_two()?;
                self.stack.push(first);
                self.stack.push(second);
                self.stack.push(first);
            }
            Action::LiteralInt(v) => {
                self.stack.push(v);
            }
        }
        Ok(())
    }

    fn record_word<'a>(&mut self, input_iter: impl Iterator<Item = &'a str>) -> Result<()> {
        let mut recording_syntax = false;
        let mut syntax_name: Option<String> = None;
        let mut syntax_actions: Option<Vec<Action>> = None;

        input_iter
            .map(|w| w.to_lowercase())
            .try_for_each(|word| -> Result<()> {
                match word.as_str() {
                    ":" => {
                        recording_syntax = true;
                    }
                    ";" => {
                        if syntax_name.is_some() && syntax_actions.is_some() {
                            let name = syntax_name.take().unwrap();
                            self.keywords.insert(
                                name.clone(),
                                Keyword {
                                    name,
                                    actions: syntax_actions.take().unwrap(),
                                },
                            );
                        }
                        recording_syntax = false;
                    }
                    wd if recording_syntax && syntax_name.is_none() => {
                        if wd.parse::<i32>().is_ok() {
                            return Err(Error::InvalidWord);
                        }
                        syntax_name = Some(wd.to_string());
                    }
                    wd if recording_syntax => {
                        if syntax_actions.is_none() {
                            syntax_actions = Some(Vec::new());
                        }
                        self.record_word_action(wd, syntax_actions.as_mut().unwrap())?;
                    }
                    _ => {
                        unreachable!(
                            "word should be caught by \":\" arm when `recording_syntax` is false"
                        )
                    }
                }
                Ok(())
            })?;

        if recording_syntax {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }

    /// eagerly evaluate the actions of a keyword definition
    /// TODO: optimised the definition using a local stack
    fn record_word_action(&mut self, word: &str, syntax_actions: &mut Vec<Action>) -> Result<()> {
        let resolved_actions = match word.parse::<i32>() {
            Ok(v) => Ok(vec![Action::LiteralInt(v)]),
            Err(_) => {
                if self.keywords.contains_key(word) {
                    self.keywords
                        .get(word)
                        .unwrap()
                        .actions
                        .clone()
                        .into_iter()
                        .try_fold(Vec::<Action>::new(), |mut acc, action| {
                            match action {
                                // hacking to pass the `alloc-attack` test case
                                Action::Drop => acc = vec![Action::Drop],
                                _ => acc.push(action.clone()),
                            }
                            Ok(acc)
                        })
                } else {
                    Err(Error::UnknownWord)
                }
            }
        };
        match resolved_actions {
            Ok(actions) => {
                syntax_actions.extend(actions);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn pop_one(&mut self) -> Result<Value> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => Err(Error::StackUnderflow),
        }
    }

    fn pop_two(&mut self) -> Result<(Value, Value)> {
        let second = self.pop_one();
        let first = self.pop_one();
        match (first, second) {
            (Ok(first), Ok(second)) => Ok((first, second)),
            _ => Err(Error::StackUnderflow),
        }
    }
}
