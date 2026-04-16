use std::{borrow::Cow, fmt::Write, str::FromStr};

use serde_json::{Map, Value};

use crate::{
    builtin,
    runtime::{functions::FunctionLibrary, output::Output, scope::ScopeStack, ExecutionError},
};

fn gron(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let input = match args {
        [input] => input,
        _ => return Err(ExecutionError::InternalError),
    };
    let Ok(json_value) = serde_json::from_str(&input.value()) else {
        return Ok(Output::new_falsy());
    };
    let mut output = String::new();
    fn flatten(value: &Value, path: &str, output: &mut String) {
        match value {
            Value::Object(map) => {
                writeln!(output, "{} = {{}}", path).unwrap();
                for (key, val) in map {
                    let new_path = format!("{}.{}", path, key);
                    flatten(val, &new_path, output);
                }
            }
            Value::Array(arr) => {
                writeln!(output, "{} = []", path).unwrap();
                for (index, val) in arr.iter().enumerate() {
                    let new_path = format!("{}[{}]", path, index);
                    flatten(val, &new_path, output);
                }
            }
            _ => {
                writeln!(output, "{} = {}", path, value).unwrap();
            }
        }
    }
    flatten(&json_value, "json", &mut output);
    Ok(Output::new_truthy_with(output.into()))
}

fn jstr(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let input = match args {
        [input] => input,
        _ => return Err(ExecutionError::InternalError),
    };
    let Ok(input) = serde_json::to_string(input.value()) else {
        return Err(ExecutionError::InternalError);
    };
    Ok(Output::new_truthy_with(input.into()))
}

fn jnum(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let input = match args {
        [input] => input,
        _ => return Err(ExecutionError::InternalError),
    };
    let Ok(input) = serde_json::Number::from_str(input.value()) else {
        return Ok(Output::new_falsy_with("null".into()));
    };
    let Ok(input) = serde_json::to_string(&input) else {
        return Err(ExecutionError::InternalError);
    };
    Ok(Output::new_truthy_with(input.into()))
}

fn fallback_to_str<'a>(value: &'a str) -> Cow<'a, str> {
    if let Ok(_) = serde_json::from_str::<Value>(value) {
        value.into()
    } else {
        serde_json::to_string(value).unwrap().into()
    }
}

fn fallback_to_str_value<'a>(value: &'a str) -> Value {
    if let Ok(value) = serde_json::from_str::<Value>(value) {
        value
    } else {
        Value::String(value.into())
    }
}

fn jarr(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let mut output = String::new();
    let mut first = true;
    output += "[";
    for arg in args {
        if !first {
            output += ","
        }
        output += &fallback_to_str(arg.value());
        first = false;
    }
    output += "]";
    Ok(Output::new_truthy_with(output.into()))
}

fn jobj(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let mut output = String::new();
    let mut first = true;
    output += "{";
    for pair in args.chunks(2) {
        match pair {
            [key, value] => {
                if !first {
                    output += ","
                }
                let Ok(name) = serde_json::to_string(key.value()) else {
                    return Err(ExecutionError::InternalError);
                };
                output += &name;
                output += ":";
                output += &fallback_to_str(value.value());
                first = false;
            }
            _ => break,
        }
    }
    output += "}";
    Ok(Output::new_truthy_with(output.into()))
}

fn jstr_parse(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let input = match args {
        [input] => input,
        _ => return Err(ExecutionError::InternalError),
    };
    let Ok(input) = serde_json::from_str::<String>(input.value()) else {
        return Ok(Output::new_falsy());
    };
    Ok(Output::new_truthy_with(input.into()))
}

fn jpush(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let (obj, items) = match args {
        [obj, items @ ..] => (obj, items),
        _ => return Err(ExecutionError::InternalError),
    };
    if let Ok(mut array) = serde_json::from_str::<Vec<Value>>(obj.value()) {
        for item in items {
            array.push(fallback_to_str_value(item.value()));
        }
        let Ok(output) = serde_json::to_string(&array) else {
            return Err(ExecutionError::InternalError);
        };
        Ok(Output::new_truthy_with(output.into()))
    } else if let Ok(mut obj) = serde_json::from_str::<Map<String, Value>>(obj.value()) {
        for pair in items.chunks(2) {
			match pair {
				[key, value] => {
					obj.insert(key.value().into(), fallback_to_str_value(value.value()));
				}
				_ => break,
			}
        }
        let Ok(output) = serde_json::to_string(&obj) else {
            return Err(ExecutionError::InternalError);
        };
        Ok(Output::new_truthy_with(output.into()))
    } else {
        Ok(Output::new_falsy_with("null".into()))
    }
}

fn jlen(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let obj = match args {
        [obj] => obj,
        _ => return Err(ExecutionError::InternalError),
    };
    if let Ok(array) = serde_json::from_str::<Vec<Value>>(obj.value()) {
        Ok(Output::new_truthy_with(array.len().to_string().into()))
    } else if let Ok(obj) = serde_json::from_str::<Map<String, Value>>(obj.value()) {
        Ok(Output::new_truthy_with(obj.len().to_string().into()))
    } else {
        Ok(Output::new_falsy_with("null".into()))
    }
}

fn jget(
    _: &FunctionLibrary,
    _: &mut ScopeStack,
    args: &[Output],
) -> Result<Output, ExecutionError> {
    let (obj, key) = match args {
        [obj, key] => (obj, key),
        _ => return Err(ExecutionError::InternalError),
    };
    if let Ok(array) = serde_json::from_str::<Vec<Value>>(obj.value()) {
		let Ok(key): Result<usize, _> = key.value().parse() else {
			return Ok(Output::new_falsy_with("null".into()));
		};
		match array.get(key) {
			Some(value) => {
				Ok(Output::new_truthy_with(serde_json::to_string(value).unwrap().into()))
			},
			None => Ok(Output::new_falsy_with("null".into())),
		}
    } else if let Ok(obj) = serde_json::from_str::<Map<String, Value>>(obj.value()) {
        match obj.get(key.value()) {
			Some(value) => {
				Ok(Output::new_truthy_with(serde_json::to_string(value).unwrap().into()))
			},
			None => Ok(Output::new_falsy_with("null".into())),
		}
    } else {
        Ok(Output::new_falsy_with("null".into()))
    }
}

pub fn build() -> FunctionLibrary {
    let mut library = FunctionLibrary::new();
    builtin!(library, gron, "input");
    builtin!(library, jstr, "string");
    builtin!(library, jnum, "number");
    builtin!(library, jarr, "%items");
    builtin!(library, jobj, "%keys_values");
	builtin!(library, jstr_parse, "json_string");
	builtin!(library, jpush, "object", "%items");
	builtin!(library, jlen, "object");
	builtin!(library, jget, "object", "key");
    library
}
