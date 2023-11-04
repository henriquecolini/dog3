use crate::{
	builtin, builtin_alias,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn truthy(_: &FunctionLibrary, _: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_truthy())
}

fn falsy(_: &FunctionLibrary, _: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_falsy())
}

fn eq(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a == b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn neq(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a != b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn gt(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a > b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn lt(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a < b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn geq(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a >= b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn leq(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			let a: Result<f64, _> = a.try_into();
			let b: Result<f64, _> = b.try_into();
			match (a, b) {
				(Ok(a), Ok(b)) => Output::new("".into(), if a <= b { 0 } else { 1 }),
				_ => Output::new_falsy(),
			}
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn like(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok(Output::new(
			"".into(),
			if a.value() == b.value() { 0 } else { 1 },
		)),
		_ => Err(ExecutionError::InternalError),
	}
}

fn and(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			Output::new(
				"".into(),
				if a.is_truthy() && b.is_truthy() { 0 } else { 1 },
			)
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn or(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a, b] => Ok({
			Output::new(
				"".into(),
				if a.is_truthy() || b.is_truthy() { 0 } else { 1 },
			)
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

fn not(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[a] => Ok(Output::new("".into(), if a.is_truthy() { 1 } else { 0 })),
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
	builtin!(library, not, "a");
	library
}
