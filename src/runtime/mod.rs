pub mod functions;
pub mod output;
pub mod scope;

use std::{collections::HashMap, fmt::Display};

use crate::parser::{format_string::*, grammar::*};

use functions::*;
use output::*;
use scope::ScopeStack;
use scope::*;

pub struct Runtime {
	functions: FunctionLibrary,
	global_scope: Scope,
}

#[derive(Debug)]
pub enum RuntimeError {
	RegisterError(RegisterError),
	ExecutionError(ExecutionError),
}

pub enum Next {
	Proceed,
	Append(Output),
	Return(Output),
	Clear(Output),
	Abort(ExecutionError),
}

#[derive(Debug)]
pub enum ExecutionError {
	UndeclaredVariable(String),
	UndefinedFunction(String),
	UndefinedOverload(String, usize),
	InternalError,
}

macro_rules! scoped {
	($stack:expr, $block:block) => {{
		$stack.push();
		let res = (|| $block)();
		$stack.pop();
		res
	}};
}

impl Next {
	fn supress(self) -> Next {
		if let Next::Append(_) = self {
			Next::Proceed
		} else {
			self
		}
	}
}

impl From<Result<Output, ExecutionError>> for Next {
	fn from(value: Result<Output, ExecutionError>) -> Self {
		match value {
			Ok(output) => Next::Append(output),
			Err(err) => Next::Abort(err),
		}
	}
}

impl Display for ExecutionError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ExecutionError::UndeclaredVariable(var) => {
				write!(f, "error: Use of undeclared variable `{}`", var)
			}
			ExecutionError::UndefinedFunction(func) => {
				write!(f, "error: Use of undefined function `{}`", func)
			}
			ExecutionError::UndefinedOverload(func, arg_c) => {
				write!(
					f,
					"error: no overload for function `{}` takes `{}` arguments",
					func, arg_c
				)
			}
			ExecutionError::InternalError => write!(f, "error: Internal runtime error"),
		}
	}
}

macro_rules! evaluate {
	($x:expr) => {
		match $x {
			Next::Append(output) => output,
			Next::Proceed => unreachable!(),
			other => return other,
		}
	};
}

macro_rules! proceed {
	($x:expr) => {
		match $x {
			Next::Proceed => Next::Proceed,
			other => return other,
		}
	};
}

fn execute_string(stack: &mut ScopeStack, name: &FormatString) -> Next {
	let mut output = Output::new_truthy();
	for piece in name.into_iter() {
		match piece {
			FormatStringPiece::Raw(value) => output.append(Output::new(value.to_owned(), 0)),
			FormatStringPiece::Variable(var) => match stack.get_var(var) {
				Some(value) => output.append(value.clone()),
				None => return Next::Abort(ExecutionError::UndeclaredVariable(var.to_owned())),
			},
		}
	}
	Next::Append(output)
}

fn execute_value(functions: &FunctionLibrary, stack: &mut ScopeStack, value: &Value) -> Next {
	match value {
		Value::String(name) => execute_string(stack, name),
		Value::Block(block) => execute_block(functions, stack, block),
		Value::ControlStatement(control) => execute_control_statement(functions, stack, control),
	}
}

fn execute_block(functions: &FunctionLibrary, stack: &mut ScopeStack, block: &Block) -> Next {
	scoped!(stack, {
		execute_statements(functions, stack, &block.executions)
	})
}

fn execute_open_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	open: &OpenStatement,
) -> Next {
	match &open {
		OpenStatement::SetStmt(stmt) => execute_set_statement(functions, stack, stmt),
		OpenStatement::ReturnStmt(stmt) => execute_return_statement(functions, stack, stmt),
		OpenStatement::ClearStmt(stmt) => execute_clear_statement(functions, stack, stmt),
		OpenStatement::CommandStmt(stmt) => execute_command_statement(functions, stack, stmt),
	}
}

fn execute_set_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &SetStatement,
) -> Next {
	match execute_value(functions, stack, &stmt.value) {
		Next::Append(output) => {
			stack.set_var(stmt.variable.as_str(), output);
			Next::Proceed
		}
		other => other,
	}
}

fn execute_command_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &CommandStatement,
) -> Next {
	let func_list = functions.get_list(&stmt.name);
	let func_list = match func_list {
		Some(value) => value,
		None => return Next::Abort(ExecutionError::UndefinedFunction(stmt.name.to_owned())),
	};
	let count = stmt.parameters.len();
	let func = func_list
		.iter()
		.find(|x| x.max_args >= count && x.min_args <= count);
	let func = match func {
		Some(func) => func,
		None => {
			return Next::Abort(ExecutionError::UndefinedOverload(
				stmt.name.to_owned(),
				count,
			))
		}
	};
	let mut outputs = vec![];
	for arg in &stmt.parameters {
		let output = evaluate!(execute_value(functions, stack, &arg.value));
		outputs.push(output);
	}
	match &func.runnable {
		Runnable::Block(block) => {
			let mut func_stack = ScopeStack::new_sibling(stack);
			for arg in func.args.iter() {
				if arg.vector {
					let joined_values = outputs
						.iter()
						.map(|output| output.value.clone())
						.collect::<Vec<String>>()
						.join(" ");
					let last_code = outputs.last().map(|o| o.code).unwrap_or(0);
					func_stack.set_var(
						&arg.name,
						Output {
							value: joined_values,
							code: last_code,
						},
					);
					break;
				} else {
					func_stack.set_var(&arg.name, outputs.remove(0))
				}
			}
			let res = execute_block(functions, &mut func_stack, block);
			let res = match res {
				Next::Return(out) => Next::Append(out),
				other => other,
			};
			return res;
		}
		Runnable::BuiltIn(builtin) => return builtin(outputs.as_slice()).into(),
	}
}

fn execute_clear_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &ClearStatement,
) -> Next {
	Next::Clear(match &stmt.value {
		Some(value) => match execute_value(functions, stack, &value) {
			Next::Append(output) => output,
			Next::Proceed => unreachable!(),
			other => return other,
		},
		None => Output::new_truthy(),
	})
}

fn execute_return_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &ReturnStatement,
) -> Next {
	Next::Return(match &stmt.value {
		Some(value) => match execute_value(functions, stack, &value) {
			Next::Append(output) => output,
			Next::Proceed => unreachable!(),
			other => return other,
		},
		None => Output::new_truthy(),
	})
}

fn execute_for_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &ForStatement,
) -> Next {
	scoped!(stack, {
		let mut output = Output::new_truthy();
		let list = evaluate!(execute_value(functions, stack, &stmt.list));
		let split = match &stmt.split {
			None => None,
			Some(split) => Some(evaluate!(execute_value(functions, stack, split))),
		};
		for value in list.split_iter(split.as_ref()) {
			stack.set_var(&stmt.variable, Output::new(value.to_owned(), 0));
			proceed!(scoped!(stack, {
				output.append(evaluate!(execute_value(functions, stack, &stmt.output)));
				Next::Proceed
			}));
		}
		Next::Append(output)
	})
}

fn execute_if_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &IfStatement,
) -> Next {
	let condition = evaluate!(execute_value(functions, stack, &stmt.condition));
	if condition.is_truthy() {
		execute_value(functions, stack, &stmt.output)
	} else {
		Next::Append(Output::new_falsy())
	}
}

fn execute_if_else_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &IfElseStatement,
) -> Next {
	let condition = evaluate!(execute_value(functions, stack, &stmt.condition));
	if condition.is_truthy() {
		execute_value(functions, stack, &stmt.output_true)
	} else {
		execute_value(functions, stack, &stmt.output_false)
	}
}

fn execute_while_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	stmt: &WhileStatement,
) -> Next {
	let mut output = Output::new_truthy();
	let mut condition = evaluate!(execute_value(functions, stack, &stmt.condition));
	while condition.is_truthy() {
		proceed!(scoped!(stack, {
			output.append(evaluate!(execute_value(functions, stack, &stmt.output)));
			Next::Proceed
		}));
		condition = evaluate!(execute_value(functions, stack, &stmt.condition));
	}
	Next::Append(output)
}

fn execute_control_statement(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	control: &ControlStatement,
) -> Next {
	match control {
		ControlStatement::ForStatement(stmt) => execute_for_statement(functions, stack, stmt),
		ControlStatement::IfStatement(stmt) => execute_if_statement(functions, stack, stmt),
		ControlStatement::IfElseStatement(stmt) => {
			execute_if_else_statement(functions, stack, stmt)
		}
		ControlStatement::WhileStatement(stmt) => execute_while_statement(functions, stack, stmt),
	}
}

fn execute_statements(
	functions: &FunctionLibrary,
	stack: &mut ScopeStack,
	execs: &[Execution],
) -> Next {
	let mut output = Output::new_truthy();
	for exec in execs.iter() {
		let next = match exec {
			Execution::Block(block) => execute_block(functions, stack, block).supress(),
			Execution::ControlStatement(control) => {
				execute_control_statement(functions, stack, control).supress()
			}
			Execution::OpenStatement(open) => execute_open_statement(functions, stack, open),
		};
		match next {
			Next::Proceed => continue,
			Next::Append(out) => output.append(out),
			Next::Clear(out) => output = out,
			other => return other,
		}
	}
	Next::Append(output)
}

impl Runtime {
	pub fn new() -> Runtime {
		Runtime {
			functions: FunctionLibrary::new(),
			global_scope: HashMap::new(),
		}
	}
	pub fn register_library(&mut self, other: FunctionLibrary) -> Result<String, RegisterError> {
		self.functions.register_library(other)
	}
	pub fn register_script_library(&mut self, functions: Vec<Function>) -> Vec<String> {
		let mut registered: Vec<String> = vec![];
		for func in functions {
			let res = self.functions.register_function(
				&func.name,
				func.args,
				Runnable::Block(func.block),
			);
			if let Err(err) = res {
				eprintln!("{}", err);
			} else {
				registered.push(func.def);
			}
		}
		registered
	}
	pub fn execute(&mut self, execs: &[Execution]) -> Result<Output, ExecutionError> {
		let mut glob = ScopeStack::new(&mut self.global_scope);
		let res = execute_statements(&self.functions, &mut glob, &execs);
		match res {
			Next::Append(output) => Ok(output),
			Next::Return(output) => Ok(output),
			Next::Abort(err) => Err(err),
			_ => Err(ExecutionError::InternalError),
		}
	}
}
