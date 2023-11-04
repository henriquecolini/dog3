use std::collections::{HashMap, VecDeque};

use super::output::Output;

pub type Scope = HashMap<String, Output>;

#[derive(Debug)]
pub struct ScopeStack<'a> {
	global: &'a mut Scope,
	stack: VecDeque<Scope>,
}

impl<'a> ScopeStack<'a> {
	pub fn new(global: &'a mut Scope) -> ScopeStack<'a> {
		ScopeStack {
			global: global,
			stack: VecDeque::new(),
		}
	}
	pub fn call_frame(sibling: &'a mut ScopeStack) -> ScopeStack<'a> {
		ScopeStack {
			global: sibling.global,
			stack: VecDeque::from([HashMap::new()]),
		}
	}
	pub fn push(&mut self) {
		self.stack.push_front(HashMap::new());
	}
	pub fn pop(&mut self) {
		self.stack.pop_front().expect("Empty stack pop");
	}
	pub fn get_var(&self, var: &str) -> Option<&Output> {
		for scope in &self.stack {
			if let Some(out) = scope.get(var) {
				return Some(out);
			}
		}
		self.global.get(var)
	}
	// The difference between declare_var and set_var is that declare_var will
	// always set the variable in the current scope, while set_var will set the
	// variable in the first scope it finds it in.
	pub fn declare_var(&mut self, var: &str, value: Output) {
		let scope = 'find: {
			if let Some(scope) = self.stack.front_mut() {
				break 'find scope;
			}
			break 'find &mut self.global;
		};
		scope.insert(var.to_owned(), value);
	}
	pub fn set_var(&mut self, var: &str, value: Output) {
		let scope = 'find: {
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
		};
		scope.insert(var.to_owned(), value);
	}
}
