use rand::Rng;

use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn numbers(args: &[Output]) -> Option<(f64, Vec<f64>)> {
	match args {
		[first, rest @ ..] => {
			let first = match first.try_into() {
				Ok(x) => x,
				Err(_) => return None,
			};
			let mut options = vec![];
			for arg in rest {
				let parsed = arg.try_into();
				match parsed {
					Ok(number) => options.push(number),
					Err(_) => return None,
				}
			}
			Some((first, options))
		}
		_ => None,
	}
}

fn add(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				val += x;
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn sub(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				val -= x;
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn mul(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				val *= x;
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn div(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				val /= x;
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn max(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				if x > val {
					val = x
				}
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn min(args: &[Output]) -> Result<Output, ExecutionError> {
	Ok(match numbers(args) {
		Some((first, numbers)) => {
			let mut val = first;
			for x in numbers {
				if x < val {
					val = x
				}
			}
			Output::new_truthy_with(val.to_string().into())
		}
		None => Output::new_falsy(),
	})
}

fn floor(args: &[Output]) -> Result<Output, ExecutionError> {
	let number: Result<f64, _> = match args {
		[number] => number.try_into(),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(match number {
		Ok(number) => Output::new_truthy_with(number.floor().to_string().into()),
		Err(_) => Output::new_falsy(),
	})
}

fn ceil(args: &[Output]) -> Result<Output, ExecutionError> {
	let number: Result<f64, _> = match args {
		[number] => number.try_into(),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(match number {
		Ok(number) => Output::new_truthy_with(number.ceil().to_string().into()),
		Err(_) => Output::new_falsy(),
	})
}

fn random(args: &[Output]) -> Result<Output, ExecutionError> {
	let (min, max) = match args {
		[max] => (Ok(0), max.try_into()),
		[min, max] => (min.try_into(), max.try_into()),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(match (min, max) {
		(Ok(min), Ok(max)) => Output::new_truthy_with(
			if min >= max {
				min
			} else {
				rand::thread_rng().gen_range(min..max)
			}
			.to_string()
			.into(),
		),
		_ => Output::new_falsy(),
	})
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, add, "first", "%others");
	builtin!(library, sub, "first", "%others");
	builtin!(library, mul, "first", "%others");
	builtin!(library, div, "first", "%others");
	builtin!(library, max, "first", "%others");
	builtin!(library, min, "first", "%others");
	builtin!(library, floor, "x");
	builtin!(library, ceil, "x");
	builtin!(library, random, "max");
	builtin!(library, random, "min", "max");
	library
}
