use crate::{
	runtime::{
		functions::{FunctionLibrary},
		output::{join_outputs, Output},
		runtime::{ExecutionError},
	}, builtin,
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

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, put, "%in");
	builtin!(library, pln, "%in");
	builtin!(library, print, "%in");
	builtin!(library, println, "%in");
	library
}
