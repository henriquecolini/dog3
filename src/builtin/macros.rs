#[macro_export]
macro_rules! builtin_alias {
	($library:expr, $func:expr, $alias:expr) => {
		builtin_alias!($library,$func,$alias,)
	};
	($library:expr, $func:expr, $alias:expr, $($param:expr),*) => {
		{
			#[allow(unused_mut)]
				let mut parameters = Vec::new();
				$(
					let vector = $param.starts_with('%');
					let name = $param.trim_start_matches('%').to_string();
					let formal_param = if vector {
						$crate::parser::grammar::FormalParameter::new_vector(&name)
					} else {
						$crate::parser::grammar::FormalParameter::new(&name)
					};
					parameters.push(formal_param);
				)*
			$library
			.register_function($alias, parameters, $crate::runtime::functions::Runnable::BuiltIn($func))
			.unwrap();
		}
	};
}

#[macro_export]
macro_rules! builtin {
	($library:expr, $name:expr, $($param:expr),*) => {
		$crate::builtin_alias!($library, $name, stringify!($name), $($param),*)
	};
}
