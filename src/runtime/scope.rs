use std::collections::{HashMap, VecDeque};

use super::output::Output;

pub type Scope = HashMap<String, Output>;

pub struct ScopeStack<'a> {
	global: &'a mut Scope,
	stack: VecDeque<Scope>
}

impl<'a> ScopeStack<'a> {
	pub fn new (global: &'a mut Scope) -> ScopeStack<'a> {
		ScopeStack { global: global, stack: VecDeque::new() }
	}
	pub fn new_sibling (sibling: &'a mut ScopeStack) -> ScopeStack<'a> {
		ScopeStack { global: sibling.global, stack: VecDeque::new() }
	}
	pub fn push (&mut self) {
		self.stack.push_front(HashMap::new())
	}
	pub fn pop (&mut self) {
		self.stack.pop_front().expect("Empty stack pop");
	}
	pub fn get_var (&self, var: &str) -> Option<&Output> {
		for scope in &self.stack {
			if let Some(out) = scope.get(var) {
				return Some(out);
			}
		}
		self.global.get(var)
	}
	pub fn set_var (&mut self, var: &str, value: Output) {
		'find: {
			for scope in &mut self.stack {
				if let Some(_) = scope.get(var) {
					break 'find scope;
				}
			}
			if let Some(_) = self.global.get(var) {
				break 'find self.global;
			}
			if let Some(scope) = self.stack.front_mut() {
				break 'find scope;
			}
			break 'find &mut self.global;
		}.insert(var.to_owned(), value);
	}
}
