use crate::{runtime::{output::Output, runtime::{ExecutionError, self}, functions::{FunctionLibrary, Runnable}}, params};


fn put(args: &[Output]) -> Result<Output, ExecutionError> {
	let joined_values = args
		.iter()
		.map(|output| output.value.clone())
		.collect::<Vec<String>>()
		.join(" ");
	Ok(Output::new(joined_values, 0))
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

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	library
		.register_function("put", params!("%in"), Runnable::BuiltIn(put))
		.expect("Failed to register put");
	library
		.register_function("arrlen", params!("arr"), Runnable::BuiltIn(arrlen))
		.expect("Failed to register arr");
	library
}
