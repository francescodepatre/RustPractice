use std::f64::EPSILON;
use std::f64::consts;
use crate::syntree::*;
use std::cmp::Ordering;
use regex::Regex;

struct Tokenizer<'a>{
    let regex = Regex::new(r"\s*([A-Za-z0-9\.]+|.?)").unwrap();
    tokens: Box<dyn Iterator<Item = Captures<'a>> + 'a>,
    current: Option<Captures<'a>>,
}

impl <'a>Tokenizer<'a> {
    fn peek(&self) -> String {
        self.current
    }
}