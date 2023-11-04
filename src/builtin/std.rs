use std::borrow::Cow;

use crate::{
	builtin, parser,
	runtime::{
		self,
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError, Runtime,
	},
};

// TODO: Improve memory usage. We can always move outputs into functions, and reuse the first one.
// Maybe Builtin functions could take either an iterator over outputs, or maybe we could have an enum of
// possible builtin signatures, such as
// enum Builtin { One(Fn(Output) -> Output), Two(Fn(Output, Output) -> Output), ..., Any(Fn(Iterator<Output>) -> Output) }

fn put(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(join_outputs(args.iter()))
}

fn pln(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	Ok(out)
}

fn print(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let out = join_outputs(args.iter());
	print!("{}", out.value());
	Ok(Output::new_truthy())
}

fn println(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	print!("{}", out.value());
	Ok(Output::new_truthy())
}

fn status(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
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

fn src(fl: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[value] => Ok(Output::new_truthy_with(
			fl.get_scripts(true, true, Some(value.value())).into(),
		)),
		_ => Err(ExecutionError::InternalError),
	}
}

fn eval(fl: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
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
					match runtime.execute(&program.executions) {
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
	library
}
