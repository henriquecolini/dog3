use std::{fmt::Display, fs, path::PathBuf, process::ExitCode};

use clap::Parser;
use dog3::{
	builtin,
	parser::{parse, Rule},
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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::Syntax(value)
    }
}

impl From<RegisterError> for Error {
    fn from(value: RegisterError) -> Self {
        Self::Library(value)
    }
}

fn register_libraries(runtime: &mut Runtime) -> Result<String, RegisterError> {
	runtime.library.merge(builtin::std::build())?;
	runtime.library.merge(builtin::iter::build())?;
	runtime.library.merge(builtin::math::build())?;
	runtime.library.merge(builtin::logic::build())?;
	runtime.library.merge(builtin::net::build())
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
	register_libraries(&mut runtime)?;
	let program = parse(&inputs.join("\n"))?;
	runtime.library.add_scripts(program.functions)?;
	match runtime.execute(&program.executions) {
		Ok(output) => print!("{}", output.value()),
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
