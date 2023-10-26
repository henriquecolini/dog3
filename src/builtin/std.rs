use crate::{
	builtin,
	runtime::{
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError,
	},
};

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

fn status(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
		[value] => Ok(Output::new(value.code.to_string(), value.code)),
		[value, status] => {
			let status = status.value.parse();
			Ok(Output::new(
				value.value.to_owned(),
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
