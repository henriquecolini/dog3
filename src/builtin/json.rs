use std::fmt::Write;

use serde_json::Value;

use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn gron(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let input = match args {
		[input] => input,
		_ => return Err(ExecutionError::InternalError),
	};
	let Ok(json_value) = serde_json::from_str(&input.value()) else {
		return Ok(Output::new_falsy());
	};
	let mut output = String::new();
	fn flatten(value: &Value, path: &str, output: &mut String) {
		match value {
			Value::Object(map) => {
				writeln!(output, "{} = {{}}", path).unwrap();
				for (key, val) in map {
					let new_path = format!("{}.{}", path, key);
					flatten(val, &new_path, output);
				}
			}
			Value::Array(arr) => {
				writeln!(output, "{} = []", path).unwrap();
				for (index, val) in arr.iter().enumerate() {
					let new_path = format!("{}[{}]", path, index);
					flatten(val, &new_path, output);
				}
			}
			_ => {
				writeln!(output, "{} = {}", path, value).unwrap();
			}
		}
	}
	flatten(&json_value, "json", &mut output);
	Ok(Output::new_truthy_with(output.into()))
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, gron, "input");
	library
}
