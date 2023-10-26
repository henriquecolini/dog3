use crate::{
	builtin,
	runtime::{
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError,
	},
};

fn get(args: &[Output]) -> Result<Output, ExecutionError> {
	print!("{}", join_outputs(args));
	Ok(Output::new_truthy())
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, get, "url");
	library
}
