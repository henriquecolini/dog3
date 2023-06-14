use crate::{
	builtin, builtin_alias,
	runtime::{functions::FunctionLibrary, output::Output, runtime::ExecutionError},
};

fn truthy(_: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_truthy())
}

fn falsy(_: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_falsy())
}

fn eq(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a == b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn neq(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a != b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn gt(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a > b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn lt(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a < b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn geq(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a >= b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn leq(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a = a.value.parse::<f64>();
			let b = b.value.parse::<f64>();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".to_owned(), if a <= b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn like(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			Output::new("".to_owned(), if a.value == b.value { 0 } else { 1 })
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn and(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			Output::new("".to_owned(), if a.is_truthy() && b.is_truthy() { 0 } else { 1 })
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn or(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			Output::new("".to_owned(), if a.is_truthy() || b.is_truthy() { 0 } else { 1 })
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn not(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a] => Ok({
			Output::new("".to_owned(), if a.is_truthy() { 0 } else { 1 })
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin_alias!(library, truthy, "true");
	builtin_alias!(library, falsy, "false");
	builtin!(library, eq, "a", "b");
	builtin!(library, neq, "a", "b");
	builtin!(library, lt, "a", "b");
	builtin!(library, gt, "a", "b");
	builtin!(library, leq, "a", "b");
	builtin!(library, geq, "a", "b");
	builtin!(library, like, "a", "b");
	builtin!(library, and, "a", "b");
	builtin!(library, or, "a", "b");
	builtin!(library, not, "a", "b");
	library
}
