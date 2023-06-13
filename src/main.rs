use std::{fs, path::PathBuf};

use clap::Parser;
use dog3::{
	dogs,
	parser::parser::{parse, Rule},
	runtime::{functions::RegisterError, runtime::Runtime},
};

#[derive(Parser, Debug)]
struct Args {
	inputs: Vec<PathBuf>,
}

#[derive(Debug)]
enum Error {
	IO(std::io::Error),
	Syntax(pest::error::Error<Rule>),
	Library(RegisterError),
}

fn register_libraries(runtime: &mut Runtime) -> Result<String, RegisterError> {
	runtime.functions.register_library(dogs::std::build())
}

fn main() -> Result<(), Error> {
	let args = Args::parse();
	let mut inputs = vec![];
	for path in args.inputs {
		match fs::read_to_string(path) {
			Ok(content) => inputs.push(content),
			Err(err) => {
				return Err(Error::IO(err));
			}
		}
	}
	let mut runtime = Runtime::new();
	if let Err(err) = register_libraries(&mut runtime) {
		return Err(Error::Library(err));
	}
	let program = match parse(&inputs.join("\n")) {
		Ok(program) => program,
		Err(err) => return Err(Error::Syntax(err)),
	};
	runtime.execute(program);
	Ok(())
}
