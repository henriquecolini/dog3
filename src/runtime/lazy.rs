use std::{cell::Cell, str::FromStr};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct LazyParse<T: Copy + FromStr> {
    value: Cell<LazyParseValue<T>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
enum LazyParseValue<T: Copy + FromStr> {
	NotParsed,
	Ok(T),
	Err,
}

impl<'a, T: Copy + FromStr> LazyParse<T> {
    pub fn new() -> Self {
        Self { value: Cell::new(LazyParseValue::NotParsed) }
    }
    pub fn discard(&self) {
        self.value.set(LazyParseValue::NotParsed)
    }
    pub fn try_parse(&self, raw: &str) -> Option<T> {
        match self.value.get() {
            LazyParseValue::Ok(v) => Some(v),
            LazyParseValue::Err => None,
            LazyParseValue::NotParsed => {
                let res = raw.parse().ok();
                match res {
                    Some(v) => self.value.set(LazyParseValue::Ok(v)),
                    None => self.value.set(LazyParseValue::Err),
                }
                res
            },
        }
    }
}
