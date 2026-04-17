use crate::{
	builtin,
	runtime::{ExecutionError, functions::FunctionLibrary, output::Output, scope::ScopeStack},
};

async fn request_output(resp: reqwest::Response) -> Output {
    let status = resp.status().as_u16();
    let status = if status == 200 { 0 } else { status }.into();

    match resp.text().await {
        Ok(text) => Output::new(text.into(), status),
        Err(_) => Output::new("".into(), status),
    }
}

async fn get(
    _: &FunctionLibrary,
    _: &mut ScopeStack<'_>,
    args: Vec<Output>,
) -> Result<Output, ExecutionError> {
    let (url, timeout) = match args.as_slice() {
        [url] => (url, None),
        [url, timeout] => (url, Some(TryInto::<i64>::try_into(timeout))),
        _ => return Err(ExecutionError::InternalError),
    };

    let client = reqwest::Client::new();

    let mut req = client.get(url.value());

    if let Some(Ok(t)) = timeout {
        if t < 0 {
            return Ok(Output::new_falsy());
        }
        req = req.timeout(std::time::Duration::from_secs(t as u64));
    }

    match req.send().await {
        Ok(resp) => Ok(request_output(resp).await),
        Err(_) => Ok(Output::new_falsy()),
    }
}

async fn post(
    _: &FunctionLibrary,
    _: &mut ScopeStack<'_>,
    args: Vec<Output>,
) -> Result<Output, ExecutionError> {
    let (url, body, timeout) = match args.as_slice() {
        [url] => (url, "", None),
        [url, body] => (url, body.value(), None),
        [url, body, timeout] => (url, body.value(), Some(TryInto::<i64>::try_into(timeout))),
        _ => return Err(ExecutionError::InternalError),
    };

    let client = reqwest::Client::new();

    let mut req = client.post(url.value()).body(body.to_string());

    if let Some(Ok(t)) = timeout {
        if t < 0 {
            return Ok(Output::new_falsy());
        }
        req = req.timeout(std::time::Duration::from_secs(t as u64));
    }

    match req.send().await {
        Ok(resp) => Ok(request_output(resp).await),
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
