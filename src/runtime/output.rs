use std::{borrow::Cow, fmt::{Display, Debug}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Output {
	value: Cow<'static, str>,
	code: i64
}

pub enum OutputSplitIterator<'a> {
	SplitChars(CharIterator<'a>),
	Split(std::str::Split<'a, &'a str>),
	SplitWhitespace(std::str::SplitWhitespace<'a>),
}

pub struct CharIterator<'a> {
	input: &'a str,
	index: usize,
}

impl<'a> CharIterator<'a> {
	fn new(input: &'a str) -> CharIterator<'a> {
		CharIterator { input, index: 0 }
	}
}

pub fn join_outputs<'a, 'b, I: Iterator<Item = &'b Output>>(outputs: I) -> Output {
	let mut result = Output::new_truthy();
	let mut first = true;
	for out in outputs {
		if first {
			first = false;
		} else {
			result.append_str(" ");
		}
		result.append(out);
	}
	result
}

impl Output {
	pub fn new(value: Cow<'static, str>, code: i64) -> Output {
		Output { value, code }
	}
	pub fn new_truthy() -> Output {
		Self::new("".into(), 0)
	}
	pub fn new_falsy() -> Output {
		Self::new("".into(), 1)
	}
	pub fn new_truthy_with(value: Cow<'static, str>) -> Output {
		Self::new(value, 0)
	}
	pub fn new_falsy_with(value: Cow<'static, str>) -> Output {
		Self::new(value, 1)
	}
	pub fn append(&mut self, other: &Output) {
		self.value.to_mut().push_str(&other.value);
		self.code = other.code;
	}
	pub fn append_str(&mut self, other: &str) {
		self.value.to_mut().push_str(&other);
	}
	pub fn replace(&mut self, other: Cow<'static, str>) {
		self.value = other;
	}
	pub fn split_iter<'b>(&'b self, arg: Option<&'b Output>) -> OutputSplitIterator<'b> {
		match arg {
			Some(arg) => {
				if arg.value == "" {
					OutputSplitIterator::SplitChars(CharIterator::new(&self.value))
				} else {
					OutputSplitIterator::Split(self.value.split(&arg.value))
				}
			}
			None => OutputSplitIterator::SplitWhitespace(self.value.split_whitespace()),
		}
	}
	pub fn is_truthy(&self) -> bool {
		self.code == 0
	}
	pub fn value<'a>(&'a self) -> &'a str {
		&self.value
	}
	pub fn code(&self) -> i64 {
		self.code
	}
}

impl Display for Output {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}'{})", self.value, self.code)
	}
}

impl Debug for Output {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}'{})", self.value, self.code)
	}
}

impl TryFrom<&Output> for f64 {
	type Error = ();
	fn try_from(value: &Output) -> Result<Self, Self::Error> {
		value.value().parse().map_err(|_| ())
	}
}

impl TryFrom<&Output> for i64 {
	type Error = ();
	fn try_from(value: &Output) -> Result<Self, Self::Error> {
		value.value().parse().map_err(|_| ())
	}
}

impl<'a> Iterator for OutputSplitIterator<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			OutputSplitIterator::SplitChars(iter) => iter.next(),
			OutputSplitIterator::Split(iter) => iter.next(),
			OutputSplitIterator::SplitWhitespace(iter) => iter.next(),
		}
	}
}

impl<'a> Iterator for CharIterator<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.input.len() {
			None
		} else {
			let start = self.index;
			let end = self.input[self.index..]
				.char_indices()
				.nth(1)
				.map(|(i, _)| self.index + i)
				.unwrap_or_else(|| self.input.len());

			self.index = end;

			Some(&self.input[start..end])
		}
	}
}
