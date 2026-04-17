use crate::{
	builtin, builtin_alias,
	runtime::{ExecutionError, functions::FunctionLibrary, output::Output, scope::ScopeStack},
};

async fn truthy(_: &FunctionLibrary, _: &mut ScopeStack<'_>, _: Vec<Output>) -> Result<Output, ExecutionError> {
	Ok(Output::new_truthy())
}

async fn falsy(_: &FunctionLibrary, _: &mut ScopeStack<'_>, _: Vec<Output>) -> Result<Output, ExecutionError> {
	Ok(Output::new_falsy())
}

async fn eq(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn neq(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn gt(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn lt(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn geq(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn leq(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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

async fn like(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[a, b] => Ok(Output::new(
			"".into(),
			if a.value() == b.value() { 0 } else { 1 },
		)),
		_ => Err(ExecutionError::InternalError),
	}
}

async fn and(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[a, b] => Ok({
			Output::new(
				"".into(),
				if a.is_truthy() && b.is_truthy() { 0 } else { 1 },
			)
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

async fn or(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[a, b] => Ok({
			Output::new(
				"".into(),
				if a.is_truthy() || b.is_truthy() { 0 } else { 1 },
			)
		}),
		_ => Err(ExecutionError::InternalError),
	}
}

async fn not(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
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
