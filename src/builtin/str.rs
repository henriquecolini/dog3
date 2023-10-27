use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::{Output, join_outputs}, ExecutionError},
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

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, upper, "%in");
    builtin!(library, lower, "%in");
	library
}
