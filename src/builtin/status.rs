use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

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
	builtin!(library, status, "value");
	builtin!(library, status, "value", "status");
	library
}
