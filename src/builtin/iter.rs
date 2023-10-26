use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn range(args: &[Output]) -> Result<Output, ExecutionError> {
	let (number, separator) = match args {
		[number] => (number.value.parse::<u32>(), " "),
		[number, separator] => (number.value.parse::<u32>(), separator.value.as_str()),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(match number {
		Ok(number) => {
			let range = 1..=number;
			let range: Vec<String> = range.map(|x| x.to_string()).collect();
			Output::new_truthy_with(range.join(separator))
		}
		Err(_) => Output::new_falsy(),
	})
}

fn first(args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, n, separator) = match &args {
		[arr, n] => (arr, n.value.parse(), None),
		[arr, n, separator] => (arr, n.value.parse(), Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let n: usize = match n {
		Ok(x) => x,
		Err(_) => return Ok(Output::new_falsy()),
	};
	let arr: Vec<&str> = arr.split_iter(separator).collect();
	if n > arr.len() {
		return Ok(Output::new_falsy());
	}
	let separator = match separator {
		Some(s) => s.value.as_str(),
		None => " ",
	};
	Ok(Output::new_truthy_with(arr[..n].join(separator)))
}

fn last(args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, n, separator) = match &args {
		[arr, n] => (arr, n.value.parse(), None),
		[arr, n, separator] => (arr, n.value.parse(), Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let n: usize = match n {
		Ok(x) => x,
		Err(_) => return Ok(Output::new_falsy()),
	};
	let arr: Vec<&str> = arr.split_iter(separator).collect();
	if n > arr.len() {
		return Ok(Output::new_falsy());
	}
	let separator = match separator {
		Some(s) => s.value.as_str(),
		None => " ",
	};
	Ok(Output::new_truthy_with(
		arr[arr.len() - n..].join(separator),
	))
}

fn append(args: &[Output]) -> Result<Output, ExecutionError> {
	let (left, right, separator) = match &args {
		[left, right] => (left, right, None),
		[left, right, separator] => (left, right, Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	let combo = left
		.split_iter(separator)
		.chain(right.split_iter(separator));
	let separator = match separator {
		Some(s) => s.value.as_str(),
		None => " ",
	};
	Ok(Output::new_truthy_with(
		combo.collect::<Vec<&str>>().join(separator),
	))
}

fn len(args: &[Output]) -> Result<Output, ExecutionError> {
	let (arr, separator) = match &args {
		[arr] => (arr, None),
		[arr, separator] => (arr, Some(separator)),
		_ => return Err(ExecutionError::InternalError),
	};
	Ok(Output::new_truthy_with(
		arr.split_iter(separator).count().to_string(),
	))
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, range, "n");
	builtin!(library, range, "n", "sep");
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
