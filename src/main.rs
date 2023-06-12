use std::fs;

use functions::Runnable;
use output::Output;
use parser::{parse, FormalParameter};
use runtime::{ExecutionError, Runtime};

mod format_string;
mod functions;
mod output;
mod parser;
mod runtime;
mod scope;

fn put(args: &[Output]) -> Result<Output, ExecutionError> {
	let joined_values = args
		.iter()
		.map(|output| output.value.clone())
		.collect::<Vec<String>>()
		.join(" ");
	Ok(Output::new(joined_values, 0))
}

fn arrlen(args: &[Output]) -> Result<Output, ExecutionError> {
	let first = args.first();
	match first {
		Some(first) => {
			return Ok(Output::new_truthy_with(
				first.value.split_whitespace().count().to_string(),
			));
		}
		None => {
			return Err(runtime::ExecutionError::InternalError);
		}
	}
}

fn main() {
	let input = fs::read_to_string("example.dog").expect("Failed to read file");
	let program = parse(&input).expect("Failed to parse");
	let mut runtime = Runtime::new();
	runtime
		.functions
		.register_function(
			"put",
			vec![FormalParameter {
				name: "put".to_owned(),
				vector: true,
			}],
			Runnable::BuiltIn(put),
		)
		.expect("Failed to register put");
	runtime
		.functions
		.register_function(
			"arrlen",
			vec![FormalParameter {
				name: "arrlen".to_owned(),
				vector: false,
			}],
			Runnable::BuiltIn(arrlen),
		)
		.expect("Failed to register put");
	runtime.execute(program);
}
