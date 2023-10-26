use std::{fmt::Display, fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use dog3::{
	builtin,
	parser::parser::{parse, Rule},
	runtime::{functions::RegisterError, Runtime},
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

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::IO(err) => {
				write!(f, "error: Failed to read input files.\n{}", err)
			}
			Error::Syntax(err) => {
				write!(f, "error: Incorrect syntax.\n{}", err)
			}
			Error::Library(err) => {
				write!(f, "error: Failed to load standard libraries.\n{}", err)
			}
		}
	}
}

fn register_libraries(runtime: &mut Runtime) -> Result<String, RegisterError> {
	runtime.functions.register_library(builtin::std::build())?;
	runtime
		.functions
		.register_library(builtin::status::build())?;
	runtime.functions.register_library(builtin::iter::build())?;
	runtime.functions.register_library(builtin::math::build())?;
	runtime.functions.register_library(builtin::logic::build())
}

fn run() -> Result<(), Error> {
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
	match runtime.execute(program) {
		Ok(output) => print!("{}", output.value),
		Err(err) => eprintln!("{}", err),
	}
	Ok(())
}

fn main() -> ExitCode {
	match run() {
		Ok(_) => ExitCode::SUCCESS,
		Err(err) => {
			println!("{}", err);
			ExitCode::FAILURE
		}
	}
}
