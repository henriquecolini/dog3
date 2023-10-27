use crate::{
	builtin,
	runtime::{
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError,
	},
};

fn upper(args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_uppercase().into());
	Ok(out)
}

fn lower(args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_lowercase().into());
	Ok(out)
}

fn replace(args: &[Output]) -> Result<Output, ExecutionError> {
	let (target, from, to) = match args {
		[out, from, to] => (out, from, to),
		_ => return Err(ExecutionError::InternalError),
	};
	let mut out = target.clone();
	out.replace(out.value().replace(from.value(), to.value()).into());
	Ok(out)
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, upper, "%args");
	builtin!(library, lower, "%args");
	builtin!(library, replace, "target", "from", "to");
	library
}
