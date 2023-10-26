use minreq::{Request, Response};

use crate::{
	builtin,
	runtime::{functions::FunctionLibrary, output::Output, ExecutionError},
};

fn request_output(req: Request) -> Output {
	match req.send() {
		Ok(resp) => {
			let status = resp.status_code;
			let status = if status == 200 { 0 } else { status }.into();
			let bytes = resp.into_bytes();
			match String::from_utf8(bytes) {
				Ok(s) => Output::new(s.into(), status),
				Err(_) => Output::new("".into(), status),
			}
		}
		Err(_) => Output::new_falsy(),
	}
}

fn request_timeout(req: Request, timeout: Option<Result<i64, ()>>) -> Result<Request, ()> {
	match timeout {
		Some(timeout) => {
			let timeout: i64 = match timeout {
				Ok(timeout) if timeout >= 0 => timeout,
				_ => return Err(()),
			};
			let timeout = timeout as u64;
			Ok(req.with_timeout(timeout))
		}
		None => Ok(req),
	}
}

fn get(args: &[Output]) -> Result<Output, ExecutionError> {
	let (url, timeout) = match args {
		[url] => (url, None),
		[url, timeout] => (url, Some(timeout.try_into())),
		_ => return Err(ExecutionError::InternalError),
	};
	let req = minreq::get(url.value());
	let req = match request_timeout(req, timeout) {
		Ok(req) => req,
		Err(_) => return Ok(Output::new_falsy()),
	};
	Ok(request_output(req))
}

fn post(args: &[Output]) -> Result<Output, ExecutionError> {
	let (url, body, timeout) = match args {
		[url] => (url, "", None),
		[url, body] => (url, body.value(), None),
		[url, body, timeout] => (url, body.value(), Some(timeout.try_into())),
		_ => return Err(ExecutionError::InternalError),
	};
	let req = minreq::post(url.value()).with_body(body);
	let req = match request_timeout(req, timeout) {
		Ok(req) => req,
		Err(_) => return Ok(Output::new_falsy()),
	};
	Ok(request_output(req))
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
