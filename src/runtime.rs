use std::{collections::HashMap, fmt::Display, time::Instant};

use crate::{format_string::*, functions::*, output::Output, parser::*, scope::*};

pub struct Runtime<'a> {
	pub functions: FunctionLibrary,
	pub global_scope: Scope<'a>,
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
	Abort(ExecutionError),
}

#[derive(Debug)]
pub enum ExecutionError {
	UndeclaredVariable(String),
	UndefinedFunction(String),
	InternalError,
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

fn execute_string(scope: &mut Scope<'_>, name: &FormatString) -> Next {
	let mut output = Output::new_truthy();
	for piece in name.into_iter() {
		match piece {
			FormatStringPiece::Raw(value) => output.append(Output::new(value.to_owned(), 0)),
			FormatStringPiece::Variable(var) => match scope.get_var(var) {
				Some(value) => output.append(value.clone()),
				None => return Next::Abort(ExecutionError::UndeclaredVariable(var.to_owned())),
			},
		}
	}
	Next::Append(output)
}

fn execute_value(functions: &FunctionLibrary, scope: &mut Scope<'_>, value: &Value) -> Next {
	match value {
		Value::String(name) => execute_string(scope, name),
		Value::Block(block) => execute_block(functions, scope, block),
		Value::ControlStatement(control) => execute_control_statement(functions, scope, control),
	}
}

fn execute_block(functions: &FunctionLibrary, scope: &mut Scope<'_>, block: &Block) -> Next {
	let mut inner = Scope::new(Some(scope));
	return execute_statements(functions, &mut inner, &block.executions);
}

fn execute_open_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	open: &OpenStatement,
) -> Next {
	match &open {
		OpenStatement::SetStmt(stmt) => execute_set_statement(functions, scope, stmt),
		OpenStatement::ReturnStmt(stmt) => execute_return_statement(functions, scope, stmt),
		OpenStatement::ClearStmt(stmt) => execute_clear_statement(functions, scope, stmt),
		OpenStatement::CommandStmt(stmt) => execute_command_statement(functions, scope, stmt),
	}
}

fn execute_set_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &SetStatement,
) -> Next {
	match execute_value(functions, scope, &stmt.value) {
		Next::Append(output) => {
			scope.set_var(stmt.variable.as_str(), &output);
			Next::Proceed
		}
		other => other,
	}
}

fn execute_command_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &CommandStatement,
) -> Next {
	let func_list = functions.get_list(&stmt.name);
	let func_list = match func_list {
		Some(value) => value,
		None => return Next::Abort(ExecutionError::UndefinedFunction(stmt.name.to_owned())),
	};
	let mut outputs = vec![];
	for arg in &stmt.parameters {
		let output = evaluate!(execute_value(functions, scope, &arg.value));
		outputs.push(output);
	}
	let count = outputs.len();
	let func = func_list
		.iter()
		.find(|x| x.max_args >= count && x.min_args <= count);
	let func = match func {
		Some(func) => func,
		None => return Next::Abort(ExecutionError::UndefinedFunction(stmt.name.to_owned())),
	};
	match &func.runnable {
		Runnable::Block(block) => {
			let mut func_scope = Scope::new_cousin(&scope);
			for (i, arg) in func.args.iter().enumerate() {
				if i == func.args.len() {
					let joined_values = outputs
						.iter()
						.map(|output| output.value.clone())
						.collect::<Vec<String>>()
						.join(" ");
					let last_code = outputs.last().unwrap().code;
					func_scope.set_var(
						&arg.name,
						&Output {
							value: joined_values,
							code: last_code,
						},
					);
				} else {
					func_scope.set_var(&arg.name, &outputs.remove(0))
				}
			}
			return execute_block(functions, &mut func_scope, block);
		}
		Runnable::BuiltIn(builtin) => return builtin(outputs.as_slice()).into(),
	}
}

fn execute_clear_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &ClearStatement,
) -> Next {
	todo!()
}

fn execute_return_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &ReturnStatement,
) -> Next {
	Next::Return(match &stmt.value {
		Some(value) => match execute_value(functions, scope, &value) {
			Next::Append(output) => output,
			Next::Proceed => unreachable!(),
			other => return other,
		},
		None => Output::new_truthy(),
	})
}

fn execute_for_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &ForStatement,
) -> Next {
	let mut inner = Scope::new(Some(scope));
	let mut output = Output::new_truthy();
	let list = evaluate!(execute_value(functions, &mut inner, &stmt.list));
	let split = match &stmt.split {
		None => None,
		Some(split) => Some(evaluate!(execute_value(functions, &mut inner, split))),
	};
	for value in list.split_iter(&split) {
		inner.set_var(&stmt.variable, &Output::new(value.to_owned(), 0));
		let mut inner = Scope::new(Some(&mut inner));
		output.append(evaluate!(execute_value(
			functions,
			&mut inner,
			&stmt.output
		)));
	}
	Next::Append(output)
}

fn execute_if_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &IfStatement,
) -> Next {
	let mut inner = Scope::new(Some(scope));
	let condition = evaluate!(execute_value(functions, &mut inner, &stmt.condition));
	if condition.is_truthy() {
		execute_value(functions, scope, &stmt.output)
	} else {
		Next::Append(Output::new_falsy())
	}
}

fn execute_if_else_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	stmt: &IfElseStatement,
) -> Next {
	let mut inner = Scope::new(Some(scope));
	let condition = evaluate!(execute_value(functions, &mut inner, &stmt.condition));
	if condition.is_truthy() {
		execute_value(functions, scope, &stmt.output_true)
	} else {
		execute_value(functions, scope, &stmt.output_false)
	}
}

fn execute_control_statement(
	functions: &FunctionLibrary,
	scope: &mut Scope<'_>,
	control: &ControlStatement,
) -> Next {
	match control {
		ControlStatement::ForStatement(stmt) => execute_for_statement(functions, scope, stmt),
		ControlStatement::IfStatement(stmt) => execute_if_statement(functions, scope, stmt),
		ControlStatement::IfElseStatement(stmt) => {
			execute_if_else_statement(functions, scope, stmt)
		}
	}
}

fn execute_statements<'a>(
	functions: &FunctionLibrary,
	scope: &'a mut Scope<'a>,
	execs: &[Execution],
) -> Next {
	let mut output = Output::new_truthy();
	for exec in execs.iter() {
		let next = match exec {
			Execution::Block(block) => execute_block(functions, scope, block).supress(),
			Execution::ControlStatement(control) => {
				execute_control_statement(functions, scope, control).supress()
			}
			Execution::OpenStatement(open) => execute_open_statement(functions, scope, open),
		};
		match next {
			Next::Proceed => continue,
			Next::Append(out) => output.append(out),
			other => return other,
		}
	}
	Next::Append(output)
}

impl<'a> Runtime<'a> {
	pub fn new() -> Runtime<'a> {
		Runtime {
			functions: FunctionLibrary::new(),
			global_scope: Scope::new(None),
		}
	}
	pub fn execute(&'a mut self, program: Program) {
		for func in program.functions {
			let res = self.functions.register_function(
				&func.name,
				func.args,
				Runnable::Block(func.block),
			);
			match res {
				Ok(msg) => println!("[runtime] {}", msg),
				Err(err) => eprintln!("[runtime] {}", err),
			}
		}
		let glob = &mut self.global_scope;

		let start = Instant::now();
		let res = execute_statements(&self.functions, glob, &program.executions);
		let duration = start.elapsed();

		println!("[runtime] Execution finished in {:?}", duration);

		match res {
			Next::Append(output) => println!("{}", output.value),
			Next::Return(output) => println!("{}", output.value),
			Next::Abort(err) => eprintln!("[runtime] {}", err),
			_ => {}
		}
	}
}
