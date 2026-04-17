use std::borrow::Cow;

use crate::{
	builtin, parser,
	runtime::{
		ExecutionError, Runtime, functions::FunctionLibrary, output::{Output, join_outputs}, scope::ScopeStack
	},
};

// TODO: Improve memory usage. We can always move outputs into functions, and reuse the first one.
// Maybe Builtin functions could take either an iterator over outputs, or maybe we could have an enum of
// possible builtin signatures, such as
// enum Builtin { One(Fn(Output) -> Output), Two(Fn(Output, Output) -> Output), ..., Any(Fn(Iterator<Output>) -> Output) }

async fn put(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	Ok(join_outputs(args.iter()))
}

async fn pln(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	Ok(out)
}

async fn print(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let out = join_outputs(args.iter());
	print!("{}", out.value());
	Ok(Output::new_truthy())
}

async fn println(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	print!("{}", out.value());
	Ok(Output::new_truthy())
}

async fn status(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[value] => Ok(Output::new(value.code().to_string().into(), value.code())),
		[value, status] => {
			let status: Result<i64, _> = status.try_into();
			Ok(Output::new(
				Cow::Owned(value.value().into()),
				match status {
					Ok(x) => x,
					Err(_) => 1,
				},
			))
		}
		_ => Err(ExecutionError::InternalError),
	}
}

async fn src(fl: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[value] => Ok(Output::new_truthy_with(
			fl.get_scripts(true, true, Some(value.value())).into(),
		)),
		_ => Err(ExecutionError::InternalError),
	}
}

async fn eval<'env, 'stack>(fl: &FunctionLibrary, stack: &'stack mut ScopeStack<'env>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[value] => {
			let program = parser::parse(&value.value());
			match program {
				Ok(program) => {
					let mut runtime = Runtime::new();
					match runtime.library.merge(fl.clone()) {
						Ok(_) => (),
						Err(err) => return Ok(Output::new_falsy_with(err.to_string().into())),
					};
					match runtime.library.add_scripts(program.functions) {
						Ok(_) => (),
						Err(err) => return Ok(Output::new_falsy_with(err.to_string().into())),
					};
					match runtime.execute_scoped(stack, &program.executions).await {
						Ok(output) => Ok(output),
						Err(err) => Ok(Output::new_falsy_with(err.to_string().into())),
					}
				}
				Err(err) => Ok(Output::new_falsy_with(err.to_string().into())),
			}
		}
		_ => Err(ExecutionError::InternalError),
	}
}

async fn panic(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	match args.as_slice() {
		[value] => {
			Err(ExecutionError::Custom(value.value().into()))
		}
		_ => Err(ExecutionError::InternalError),
	}
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, put, "%args");
	builtin!(library, pln, "%args");
	builtin!(library, print, "%args");
	builtin!(library, println, "%args");
	builtin!(library, status, "value");
	builtin!(library, status, "value", "status");
	builtin!(library, src, "function");
	builtin!(library, eval, "code");
	builtin!(library, panic,);
	builtin!(library, panic, "message");
	library
}
