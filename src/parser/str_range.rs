#[derive(Debug, Clone)]
pub struct Range {
	begin: usize,
	len: usize,
}
impl Range {
	pub fn slice<'a>(&self, value: &'a str) -> &'a str {
		&value[self.begin..self.begin + self.len]
	}
	pub fn first_after(last: Option<&Range>, value: Option<char>) -> Range {
		let begin = match last {
			Some(range) => range.begin + range.len,
			None => 0,
		};
		let mut res = Range { begin, len: 0 };
		res.grow(value);
		return res;
	}
	pub fn grow(&mut self, value: Option<char>) {
		if let Some(c) = value {
			self.len += c.len_utf8();
		}
	}
}
