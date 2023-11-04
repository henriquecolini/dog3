use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn range(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	#[rustfmt::skip]
	let (min, max, step, separator): (Result<i64, _>, Result<i64, _>, Result<i64, _>, _) = match args {
		[max]                       => (Ok(0),          max.try_into(), Ok(1),           " "),
		[min, max]                  => (min.try_into(), max.try_into(), Ok(1),           " "),
		[min, max, step]            => (min.try_into(), max.try_into(), step.try_into(), " "),
		[min, max, step, separator] => (min.try_into(), max.try_into(), step.try_into(), separator.value()),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(match (min, max, step) {
		(Ok(min), Ok(max), Ok(step)) => {
			if step < 0 {
				return Ok(Output::new_falsy());
			}
			let range = (min..max).step_by(step as usize);
			let range: Vec<String> = range.map(|x| x.to_string()).collect();
			Output::new_truthy_with(range.join(separator).into())
		}
		_ => Output::new_falsy(),
	})
}

fn first(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, n, separator) = match &args {
		[arr, n] => (arr, n.try_into(), None),
		[arr, n, separator] => (arr, n.try_into(), Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let n: i64 = match n {
		Ok(x) if x >= 0 => x,
		_ => return Ok(Output::new_falsy()),
	};
	let n: usize = n as usize;
	let arr: Vec<&str> = arr.split_iter(separator).collect();
	if n > arr.len().try_into().unwrap_or(0) {
		return Ok(Output::new_falsy());
	}
	let separator = match separator {
		Some(s) => s.value(),
		None => " ",
	};
	Ok(Output::new_truthy_with(arr[..n].join(separator).into()))
}

fn last(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, n, separator) = match &args {
		[arr, n] => (arr, n.try_into(), None),
		[arr, n, separator] => (arr, n.try_into(), Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let n: i64 = match n {
		Ok(x) if x >= 0 => x,
		_ => return Ok(Output::new_falsy()),
	};
	let n: usize = n as usize;
	let arr: Vec<&str> = arr.split_iter(separator).collect();
	if n > arr.len() {
		return Ok(Output::new_falsy());
	}
	let separator = match separator {
		Some(s) => s.value(),
		None => " ",
	};
	Ok(Output::new_truthy_with(
		arr[arr.len() - n..].join(separator).into(),
	))
}

fn append(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (left, right, separator) = match &args {
		[left, right] => (left, right, None),
		[left, right, separator] => (left, right, Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let combo = left
		.split_iter(separator)
		.chain(right.split_iter(separator));
	let separator = match separator {
		Some(s) => s.value(),
		None => " ",
	};
	Ok(Output::new_truthy_with(
		combo.collect::<Vec<&str>>().join(separator).into(),
	))
}

fn len(_: &FunctionLibrary, args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, separator) = match &args {
		[arr] => (arr, None),
		[arr, separator] => (arr, Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(Output::new_truthy_with(
		arr.split_iter(separator).count().to_string().into(),
	))
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, range, "max");
	builtin!(library, range, "min", "max");
	builtin!(library, range, "min", "max", "step");
	builtin!(library, range, "min", "max", "step", "sep");
	builtin!(library, len, "arr");
	builtin!(library, len, "arr", "sep");
	builtin!(library, first, "arr", "n");
	builtin!(library, first, "arr", "n", "sep");
	builtin!(library, last, "arr", "n");
	builtin!(library, last, "arr", "n", "sep");
	builtin!(library, append, "left", "right");
	builtin!(library, append, "left", "right", "sep");
	library
}
