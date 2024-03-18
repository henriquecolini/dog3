use std::{collections::HashMap, fmt::Display};

use crate::parser::grammar::{Block, FormalParameter, Function};

use super::{output::Output, ExecutionError};

type BuiltIn = fn(&FunctionLibrary, &[Output]) -> Result<Output, ExecutionError>;

#[derive(Clone)]
pub enum Runnable {
	Block(Block),
	BuiltIn(BuiltIn),
}

#[derive(Clone)]
pub struct AnonymousFunction {
	pub args: Vec<FormalParameter>,
	pub min_args: usize,
	pub max_args: usize,
	pub runnable: Runnable,
	pub script: Option<String>,
}

#[derive(Clone)]
pub struct FunctionLibrary {
	functions: HashMap<String, Vec<AnonymousFunction>>,
}

#[derive(Debug)]
pub enum RegisterError {
	OverloadBuiltin(String),
}

impl AnonymousFunction {
	fn signature(&self, name: &str) -> String {
		let mut args = String::new();
		let mut first = true;
		for arg in self.args.iter() {
			if first {
				first = false;
			} else {
				args.push_str(", ");
			}
			if arg.vector {
				args.push_str("%");
			}
			args.push_str(&arg.name);
		}
		format!("fn {}({})", name, args)
	}
}

impl Display for RegisterError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			RegisterError::OverloadBuiltin(s) => {
				write!(f, "Can not overload built-in function `{}`", s)
			}
		}
	}
}

impl AnonymousFunction {
	fn new(
		args: Vec<FormalParameter>,
		runnable: Runnable,
		script: Option<String>,
	) -> AnonymousFunction {
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
			script,
		}
	}
	fn collides(&self, other: &AnonymousFunction) -> bool {
		self.min_args <= other.max_args && self.max_args >= other.min_args
	}
	fn is_builtin(&self) -> bool {
		match self.runnable {
			Runnable::Block(_) => false,
			Runnable::BuiltIn(_) => true,
		}
	}
}

impl FunctionLibrary {
	pub fn new() -> FunctionLibrary {
		FunctionLibrary {
			functions: HashMap::new(),
		}
	}
	pub fn add_builtin(&mut self, name: &str, args: Vec<FormalParameter>, runnable: BuiltIn) {
		let _ = self.add(name, args, Runnable::BuiltIn(runnable), None);
	}
	pub fn add_script(&mut self, runnable: Function) -> Result<String, RegisterError> {
		self.add(
			&runnable.name,
			runnable.args,
			Runnable::Block(runnable.block),
			Some(runnable.script),
		)
	}

	pub fn add_scripts(&mut self, functions: Vec<Function>) -> Result<(), RegisterError> {
		for func in functions {
			self.add_script(func)?;
		}
		Ok(())
	}
	fn add(
		&mut self,
		name: &str,
		args: Vec<FormalParameter>,
		runnable: Runnable,
		script: Option<String>,
	) -> Result<String, RegisterError> {
		let current = self.functions.get_mut(name);
		let anon = AnonymousFunction::new(args, runnable, script);

		match current {
			Some(funcs) => {
				if !anon.is_builtin() && funcs.iter().any(|f| f.is_builtin()) {
					return Err(RegisterError::OverloadBuiltin(name.to_owned()));
				}
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
	pub fn merge(&mut self, other: FunctionLibrary) -> Result<String, RegisterError> {
		let mut count = 0;
		for (name, anons) in other.functions.into_iter() {
			for func in anons {
				count += 1;
				self.add(&name, func.args, func.runnable, func.script)?;
			}
		}
		Ok(format!("Registered {} functions", count))
	}

	pub fn get_list(&self, name: &str) -> Option<&Vec<AnonymousFunction>> {
		self.functions.get(name)
	}

	pub fn get_scripts(
		&self,
		include_builtin: bool,
		include_script: bool,
		name: Option<&str>,
	) -> String {
		let filtered_functions = self
			.functions
			.iter()
			.filter(|func| {
				if let Some(name) = &name {
					func.0 == *name
				} else {
					true
				}
			})
			.flat_map(|(name, funcs)| funcs.iter().map(move |f| (name, f)))
			.filter_map(|(name, func)| {
				if let Some(script) = &func.script {
					if include_script {
						Some(script.to_string())
					} else {
						None
					}
				} else {
					if include_builtin {
						Some(format!("{} {{\n    // built-in\n}}", func.signature(name)))
					} else {
						None
					}
				}
			});

		let result = itertools::join(filtered_functions, "\n");
		result
	}
}
