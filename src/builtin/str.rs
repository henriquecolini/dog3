use crate::{
	builtin,
	runtime::{
		functions::FunctionLibrary,
		output::{join_outputs, Output},
		ExecutionError,
	},
};

fn upper(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_uppercase().into());
	Ok(out)
}

fn lower(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_lowercase().into());
	Ok(out)
}

fn replace(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (target, from, to) = match args {
		[out, from, to] => (out, from, to),
		_ => return Err(ExecutionError::InternalError),
	};
	let mut out = target.clone();
	out.replace(out.value().replace(from.value(), to.value()).into());
	Ok(out)
}

fn search(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (target, pattern) = match args {
		[target, pattern] => (target, pattern),
		_ => return Err(ExecutionError::InternalError),
	};
	let Ok(reg) = regex::Regex::new(pattern.value()) else {
		return Ok(Output::new_falsy());
	};
	let mut out = Output::new_truthy();
	for line in target.value().lines() {
		if reg.is_match(line) {
			out.append_str(line);
			out.append_str("\n");
		}
	}
	Ok(out)
}

fn is_alpha(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(
		if args
			.iter()
			.map(|o| o.value().chars())
			.flatten()
			.all(|c| c.is_alphabetic())
		{
			Output::new_truthy()
		} else {
			Output::new_falsy()
		},
	)
}

fn is_alphanumeric(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(
		if args
			.iter()
			.map(|o| o.value().chars())
			.flatten()
			.all(|c| c.is_alphanumeric())
		{
			Output::new_truthy()
		} else {
			Output::new_falsy()
		},
	)
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, upper, "%args");
	builtin!(library, lower, "%args");
	builtin!(library, replace, "target", "from", "to");
	builtin!(library, search, "target", "pattern");
	builtin!(library, is_alpha, "%args");
	builtin!(library, is_alphanumeric, "%args");
	library
}
