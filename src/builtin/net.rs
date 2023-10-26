use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn get(args: &[Output]) -> Result<Output, ExecutionError> {
	let (url, timeout) = match args {
		[url] => (url, None),
		[url, timeout] => (url, Some(&timeout.value)),
		_ => return Err(ExecutionError::InternalError),
	};
	let req = minreq::get(&url.value);
	let req = match timeout {
		Some(timeout) => {
			let timeout: u64 = match timeout.parse() {
				Ok(timeout) => timeout,
				_ => return Ok(Output::new_falsy()),
			};
			req.with_timeout(timeout)
		}
		None => req,
	};
	match req.send() {
		Ok(resp) => match resp.as_str() {
			Ok(data) => Ok(Output {
				value: data.to_owned(),
				code: if resp.status_code == 200 {
					0
				} else {
					resp.status_code
				},
			}),
			Err(_) => Ok(Output {
				value: "".to_owned(),
				code: resp.status_code,
			}),
		},
		Err(_) => Ok(Output::new_falsy()),
	}
}

fn post(args: &[Output]) -> Result<Output, ExecutionError> {
	let (url, body, timeout) = match args {
		[url] => (url, "", None),
		[url, body] => (url, body.value.as_ref(), None),
		[url, body, timeout] => (url, body.value.as_ref(), Some(&timeout.value)),
		_ => return Err(ExecutionError::InternalError),
	};
	let req = minreq::post(&url.value).with_body(body);
	let req = match timeout {
		Some(timeout) => {
			let timeout: u64 = match timeout.parse() {
				Ok(timeout) => timeout,
				_ => return Ok(Output::new_falsy()),
			};
			req.with_timeout(timeout)
		}
		None => req,
	};
	match req.send() {
		Ok(resp) => match resp.as_str() {
			Ok(data) => Ok(Output {
				value: data.to_owned(),
				code: if resp.status_code == 200 {
					0
				} else {
					resp.status_code
				},
			}),
			Err(_) => Ok(Output {
				value: "".to_owned(),
				code: resp.status_code,
			}),
		},
		Err(_) => Ok(Output::new_falsy()),
	}
}

pub fn build() -> FunctionLibrary {
	let mut library = FunctionLibrary::new();
	builtin!(library, get, "url");
	builtin!(library, get, "url", "timeout");
	builtin!(library, post, "url");
	builtin!(library, post, "url", "body");
	builtin!(library, post, "url", "body", "timeout");
	library
}
