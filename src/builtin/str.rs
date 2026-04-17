use crate::{
	builtin,
	runtime::{
		ExecutionError, functions::FunctionLibrary, output::{Output, join_outputs}, scope::ScopeStack
	},
};

async fn upper(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_uppercase().into());
	Ok(out)
}

async fn lower(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let mut out = join_outputs(args.iter());
	out.replace(out.value().to_lowercase().into());
	Ok(out)
}

async fn replace(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let (target, from, to) = match args.as_slice() {
		[out, from, to] => (out, from, to),
		_ => return Err(ExecutionError::InternalError),
	};
	let Ok(regex) = regex::Regex::new(from.value()) else {
		return Ok(Output::new_falsy());
	};
	let replaced = regex.replace_all(target.value(), to.value()).to_string();
	Ok(Output::new_truthy_with(replaced.into()))
}

async fn search(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
	let (target, pattern) = match args.as_slice() {
		[target, pattern] => (target, pattern),
		_ => return Err(ExecutionError::InternalError),
	};
	let Ok(reg) = regex::Regex::new(pattern.value()) else {
		return Ok(Output::new_falsy());
	};
	let mut out = Output::new_truthy();
	let mut any_match = false;
	for line in target.value().lines() {
		if reg.is_match(line) {
			out.append_str(line);
			out.append_str("\n");
			any_match = true;
		}
	}
	if !any_match {
		Ok(Output::new_falsy())
	} else {
		Ok(out)
	}
}

async fn is_alpha(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
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

async fn is_alphanumeric(_: &FunctionLibrary, _: &mut ScopeStack<'_>, args: Vec<Output>) -> Result<Output, ExecutionError> {
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
