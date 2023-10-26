use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
	pub value: String,
	pub code: i32,
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

pub fn join_outputs(outputs: &[Output]) -> String {
	outputs
		.iter()
		.map(|output| output.value.clone())
		.collect::<Vec<String>>()
		.join(" ")
}

impl Output {
	pub fn new(value: String, code: i32) -> Output {
		Output { value, code }
	}
	pub fn new_truthy() -> Output {
		Output {
			value: "".to_owned(),
			code: 0,
		}
	}
	pub fn new_falsy() -> Output {
		Output {
			value: "".to_owned(),
			code: 1,
		}
	}
	pub fn new_truthy_with(value: String) -> Output {
		Output { value, code: 0 }
	}
	pub fn new_falsy_with(value: String) -> Output {
		Output { value, code: 1 }
	}
	pub fn append(&mut self, other: Output) {
		self.value += &other.value;
		self.code = other.code;
	}
	pub fn split_iter<'a>(&'a self, arg: Option<&'a Output>) -> OutputSplitIterator<'a> {
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
