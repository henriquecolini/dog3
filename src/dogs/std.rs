use crate::{runtime::{output::Output, runtime::{ExecutionError, self}, functions::{FunctionLibrary, Runnable}}, params};

fn put(args: &[Output]) -> Result<Output, ExecutionError> {
	let joined_values = args
		.iter()
		.map(|output| output.value.clone())
		.collect::<Vec<String>>()
		.join(" ");
	Ok(Output::new(joined_values, 0))
}

fn pln(args: &[Output]) -> Result<Output, ExecutionError> {
	let joined_values = args
		.iter()
		.map(|output| output.value.clone())
		.collect::<Vec<String>>()
		.join(" ");
	Ok(Output::new(joined_values + "\n", 0))
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
	library.register_function("arrlen", params!("arr"), Runnable::BuiltIn(arrlen)).unwrap();
	library.register_function("true", params!(), Runnable::BuiltIn(truthy)).unwrap();
	library.register_function("false", params!(), Runnable::BuiltIn(falsy)).unwrap();
	library
}
