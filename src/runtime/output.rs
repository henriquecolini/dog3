#[derive(Debug, Clone)]
pub struct Output {
	pub value: String,
	pub code: i32,
}

pub enum OutputSplitIterator<'a> {
	Split(std::str::Split<'a, &'a str>),
    SplitWhitespace(std::str::SplitWhitespace<'a>),
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
			Some(arg) => OutputSplitIterator::Split(self.value.split(&arg.value)),
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
			OutputSplitIterator::Split(iter) => iter.next(),
			OutputSplitIterator::SplitWhitespace(iter) => iter.next(),
		}
	}
}
