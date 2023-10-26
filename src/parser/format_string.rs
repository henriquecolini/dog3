use super::str_range::Range;

#[derive(Debug, Clone)]
pub struct FormatString {
	value: String,
	regions: Vec<Region>,
}

#[derive(Debug, Clone)]
pub enum Region {
	Raw(Range),
	Variable(Range),
}

pub struct FormatStringIter<'a> {
	region: usize,
	target: &'a FormatString,
}

#[derive(Debug, Clone)]
pub enum FormatStringPiece<'a> {
	Raw(&'a str),
	Variable(&'a str),
}

fn is_special(c: char) -> bool {
	match c {
		';' | ',' | '{' | '}' | '(' | ')' | '"' | '\'' | '=' | '$' | '%' | '[' | ']' | '`'
		| '&' => true,
		_ => false,
	}
}

fn is_identifier_boundary(c: Option<char>) -> bool {
	match c {
		Some(c) => c != '%' && (is_special(c) || c.is_whitespace()),
		None => true,
	}
}

fn is_raw_boundary(c: Option<char>, expand_variables: bool) -> bool {
	match c {
		Some(c) => expand_variables && is_identifier_start(Some(c)),
		None => true,
	}
}

fn is_identifier_start(c: Option<char>) -> bool {
	c == Some('$')
}

fn escape(c: char) -> char {
	match c {
		'n' => '\n',
		'r' => '\r',
		't' => '\t',
		c => c,
	}
}

fn remove_ends<'a>(raw: &'a str) -> &'a str {
	match raw.chars().next() {
		Some(c) => match c {
			'\'' | '"' => {
				if raw.len() < 2 {
					""
				} else {
					&raw[1..raw.len() - 1]
				}
			}
			_ => raw,
		},
		None => raw,
	}
}

impl Region {
	fn range(&self) -> &Range {
		match self {
			Region::Raw(r) => r,
			Region::Variable(r) => r,
		}
	}
}

impl FormatString {
	pub fn raw(value: &str) -> FormatString {
		FormatString {
			value: value.to_owned(),
			regions: vec![],
		}
	}
	pub fn parse(raw: &str, expand_variables: bool) -> FormatString {
		let raw = remove_ends(raw);
		let mut target = FormatString::raw("");
		if raw.len() == 0 {
			return target;
		}
		let mut current_region = None;
		let mut iter = raw.chars();

		loop {
			let c = match iter.next() {
				Some('\\') => match iter.next() {
					Some(c) => Some(escape(c)),
					c => c,
				},
				c => c,
			};
			let is_identifier_start = expand_variables && is_identifier_start(c);
			let is_identifier_boundary = is_identifier_boundary(c);
			let is_raw_boundary = is_raw_boundary(c, expand_variables);
			let c = if is_identifier_start { iter.next() } else { c };
			if let Some(region) = &mut current_region {
				match region {
					Region::Raw(ref mut range) => {
						if is_raw_boundary {
							target.regions.push(current_region.unwrap());
							current_region = None;
						} else {
							range.grow(c);
						}
					}
					Region::Variable(ref mut range) => {
						if is_identifier_boundary {
							target.regions.push(current_region.unwrap());
							current_region = None
						} else {
							range.grow(c);
						}
					}
				}
			}
			if current_region.is_none() {
				let range = Range::first_after(target.regions.last().map(|re| re.range()), c);
				current_region = Some(if is_raw_boundary {
					Region::Variable(range)
				} else {
					Region::Raw(range)
				});
			}
			match c {
				Some(c) => target.value.push(c),
				None => break,
			}
		}
		target
	}
}

impl From<FormatString> for String {
	fn from(value: FormatString) -> Self {
		value.value.to_owned()
	}
}

impl<'a> IntoIterator for &'a FormatString {
	type Item = FormatStringPiece<'a>;
	type IntoIter = FormatStringIter<'a>;
	fn into_iter(self) -> Self::IntoIter {
		FormatStringIter {
			target: self,
			region: 0usize,
		}
	}
}

impl<'a> Iterator for FormatStringIter<'a> {
	type Item = FormatStringPiece<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.target.regions.get(self.region) {
			Some(region) => {
				self.region += 1;
				Some(match region {
					Region::Raw(range) => FormatStringPiece::Raw(range.slice(&self.target.value)),
					Region::Variable(range) => {
						FormatStringPiece::Variable(range.slice(&self.target.value))
					}
				})
			}
			None => None,
		}
	}
}
