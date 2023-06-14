use crate::{runtime::{output::{Output, join_outputs}, runtime::{ExecutionError, self}, functions::{FunctionLibrary, Runnable}}, params};

fn put(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new(join_outputs(args), 0))
}

fn pln(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new(join_outputs(args) + "\n", 0))
}

fn print(args: &[Output]) -> Result<Output, ExecutionError> {
	print!("{}", join_outputs(args));
	Ok(Output::new_truthy())
}

fn println(args: &[Output]) -> Result<Output, ExecutionError> {
	println!("{}", join_outputs(args));
	Ok(Output::new_truthy())
}

fn arrlen(args: &[Output]) -> Result<Output, ExecutionError> {
	let first = args.first();
	match first {
		Some(first) => {
			return Ok(Output::new_truthy_with(
				first.value.split_whitespace().count().to_string(),
			));
		}
		None => {
			return Err(runtime::ExecutionError::InternalError);
		}
	}
}

fn truthy(_: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_truthy())
}

fn falsy(_: &[Output]) -> Result<Output, ExecutionError> {
	Ok(Output::new_falsy())
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	library.register_function("put", params!("%in"), Runnable::BuiltIn(put)).unwrap();
	library.register_function("pln", params!("%in"), Runnable::BuiltIn(pln)).unwrap();
	library.register_function("print", params!("%in"), Runnable::BuiltIn(print)).unwrap();
	library.register_function("println", params!("%in"), Runnable::BuiltIn(println)).unwrap();
	library.register_function("arrlen", params!("arr"), Runnable::BuiltIn(arrlen)).unwrap();
	library.register_function("true", params!(), Runnable::BuiltIn(truthy)).unwrap();
	library.register_function("false", params!(), Runnable::BuiltIn(falsy)).unwrap();
	library
}
