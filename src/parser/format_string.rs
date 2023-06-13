#[derive(Debug, Clone)]
pub struct Range {
	begin: usize,
	end: usize,
}
impl Range {
	fn slice<'a>(&'a self, value: &'a str) -> &'a str {
		&value[self.begin..self.end + 1]
	}
	fn grow(&mut self) {
		self.end += 1;
	}
}

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

fn is_identifier_boundary(c: char) -> bool {
	c != '%' && (is_special(c) || c.is_whitespace())
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
		Some(c) => {
			match c {
				'\'' | '"' => {
					if raw.len() < 2 {
						""
					}
					else {
						&raw[1..raw.len()-1]
					}
				},
				_ => raw
			}
		},
		None => raw,
	}
}

impl Region {}

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

		while let Some(c) = iter.next() {
			let c = if c == '\\' {
				match iter.next() {
					Some(c) => escape(c),
					None => break,
				}
			} else {
				c
			};
			let is_identifier_boundary = is_identifier_boundary(c);
			let is_raw_boundary = expand_variables && c == '$';
			let c = if is_raw_boundary {
				match iter.next() {
					Some(c) => c,
					None => break,
				}
			} else {
				c
			}; 
			if let Some(region) = &mut current_region {
				match region {
					Region::Raw(ref mut range) => {
						if is_raw_boundary {
							target.regions.push(current_region.unwrap());
							current_region = None;
						} else {
							range.grow();
						}
					}
					Region::Variable(ref mut range) => {
						if is_identifier_boundary {
							target.regions.push(current_region.unwrap());
							current_region = None
						} else {
							range.grow();
						}
					}
				}
			}
			if current_region.is_none() {
				let len = target.value.len();
				let range = Range {
					begin: len,
					end: len,
				};
				current_region = Some(if is_raw_boundary {
					Region::Variable(range)
				} else {
					Region::Raw(range)
				});
			}
			target.value.push(c);
		}
		if let Some(region) = current_region {
			target.regions.push(region);
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
