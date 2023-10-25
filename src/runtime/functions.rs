use std::{collections::HashMap, fmt::Display};

use crate::parser::parser::{Block, FormalParameter};

use super::{output::Output, runtime::ExecutionError};

type BuiltIn = fn(&[Output]) -> Result<Output, ExecutionError>;

pub enum Runnable {
	Block(Block),
	BuiltIn(BuiltIn),
}

pub struct AnonymousFunction {
	pub args: Vec<FormalParameter>,
	pub min_args: usize,
	pub max_args: usize,
	pub runnable: Runnable,
}

pub struct FunctionLibrary {
	functions: HashMap<String, Vec<AnonymousFunction>>,
}

#[derive(Debug)]
pub enum RegisterError {
	AlreadyExists,
}

impl Display for RegisterError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "A function with equivalent signature already exists")
	}
}

impl AnonymousFunction {
	fn new(args: Vec<FormalParameter>, runnable: Runnable) -> AnonymousFunction {
		let mut has_vector = false;
		for arg in args.iter() {
			if arg.vector {
				has_vector = true
			}
		}
		let (min_args, max_args) = if has_vector {
			(args.len() - 1, usize::MAX)
		} else {
			(args.len(), args.len())
		};
		AnonymousFunction {
			min_args,
			max_args,
			args,
			runnable,
		}
	}
	fn collides(&self, other: &AnonymousFunction) -> bool {
		self.min_args <= other.max_args && self.max_args >= other.min_args
	}
}

impl FunctionLibrary {
	pub fn new() -> FunctionLibrary {
		FunctionLibrary {
			functions: HashMap::new(),
		}
	}
	pub fn register_function(
		&mut self,
		name: &str,
		args: Vec<FormalParameter>,
		runnable: Runnable,
	) -> Result<String, RegisterError> {
		let current = self.functions.get_mut(name);
		let anon = AnonymousFunction::new(args, runnable);

		match current {
			Some(funcs) => {
				funcs.retain(|a| !a.collides(&anon));
				let (min, max) = (anon.min_args, anon.max_args);
				funcs.push(anon);
				Ok(format!(
					"Registered overload for `{}` ({}-{} args)",
					name, min, max
				))
			}
			None => {
				let (min, max) = (anon.min_args, anon.max_args);
				self.functions.insert(name.to_owned(), vec![anon]);
				Ok(format!(
					"Registered function `{}` ({}-{} args)",
					name, min, max
				))
			}
		}
	}
	pub fn register_library(&mut self, other: FunctionLibrary) -> Result<String, RegisterError> {
		let mut count = 0;
		for (name, anons) in other.functions.into_iter() {
			for func in anons {
				count += 1;
				self.register_function(&name, func.args, func.runnable)?;
			}
		}
		Ok(format!("Registered {} functions", count))
	}

	pub fn get_list(&self, name: &str) -> Option<&Vec<AnonymousFunction>> {
		self.functions.get(name)
	}
}
