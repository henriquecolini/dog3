use crate::{
	runtime::{functions::FunctionLibrary, output::Output, runtime::ExecutionError}, builtin,
};

fn status(args: &[Output]) -> Result<Output, ExecutionError> {
	match args {
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
	builtin!(library, status, "value", "status");
	library
}
