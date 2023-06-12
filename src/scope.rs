use std::collections::HashMap;

use crate::output::Output;

pub struct Scope<'a> {
	root: Option<&'a Scope<'a>>,
	parent: Option<&'a Scope<'a>>,
	vars: HashMap<String, Output>,
}

impl<'a> Scope<'a> {
	pub fn new(parent: Option<&'a Scope>) -> Scope<'a> {
		Scope {
			parent,
			root: if let Some(parent) = parent {
				parent.find_root()
			} else {
				None
			},
			vars: HashMap::new(),
		}
	}
	pub fn new_cousin(parent: &'a Scope) -> Scope<'a> {
		Scope::new(parent.root)
	}
	fn find_root(&'a self) -> Option<&'a Scope<'a>> {
		match self.parent {
			Some(parent) => parent.find_root(),
			None => Some(self),
		}
	}
	pub fn get_var(&self, var: &str) -> Option<&Output> {
		let local = self.vars.get(var);
		match local {
			Some(_) => local,
			None => match self.parent {
				Some(parent) => parent.get_var(var),
				None => None,
			},
		}
	}
	pub fn set_var(&mut self, var: &str, value: &Output) {
		self.vars.insert(var.to_owned(), value.to_owned());
	}
}
