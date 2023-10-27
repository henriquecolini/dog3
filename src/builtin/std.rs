use std::borrow::Cow;

use crate::{
	builtin,
	runtime::{
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError,
	},
};

// TODO: Improve memory usage. We can always move outputs into functions, and reuse the first one.
// Maybe Builtin functions could take either an iterator over outputs, or maybe we could have an enum of
// possible builtin signatures, such as
// enum Builtin { One(Fn(Output) -> Output), Two(Fn(Output, Output) -> Output), ..., Any(Fn(Iterator<Output>) -> Output) }

fn put(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(join_outputs(args.iter()))
}

fn pln(args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	Ok(out)
}

fn print(args: &[Output]) -> Result<Output, ExecutionError> {
	let out = join_outputs(args.iter());
	print!("{}", out.value());
	Ok(out)
}

fn println(args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.append_str("\n");
	print!("{}", out.value());
	Ok(out)
}

fn status(args: &[Output]) -> Result<Output, ExecutionError> {
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

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, put, "%in");
	builtin!(library, pln, "%in");
	builtin!(library, print, "%in");
	builtin!(library, println, "%in");
	builtin!(library, status, "value");
	builtin!(library, status, "value", "status");
	library
}
