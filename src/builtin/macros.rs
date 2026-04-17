#[macro_export]
macro_rules! builtin_params {
	[$($param:expr),* $(,)?] => {
		{
			#[allow(unused_mut)]
			let mut parameters = ::std::vec::Vec::new();
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
			parameters
		}
	};
}

#[macro_export]
macro_rules! builtin_alias {
	($library:expr, $func:expr, $alias:expr) => {
		builtin_alias!($library,$func,$alias,)
	};
	($library:expr, $func:expr, $alias:expr, $($param:expr),* $(,)?) => {
		{
			$crate::runtime::functions::FunctionLibrary::add_builtin(
				&mut $library,
				$alias,
				$crate::builtin_params![$($param),*],
				{
					fn __builtin_impl<'env, 'stack>(
						lib: &'stack $crate::runtime::functions::FunctionLibrary,
						scope: &'stack mut $crate::runtime::scope::ScopeStack<'env>,
						args: ::std::vec::Vec<$crate::runtime::output::Output>,
					) -> ::std::pin::Pin<
						::std::boxed::Box<dyn ::std::future::Future<Output = Result<$crate::runtime::output::Output, $crate::runtime::ExecutionError>> + Send + 'stack>
					> {
						let fut = $func(lib, scope, args);
						::std::boxed::Box::pin(fut)
					}

					::std::boxed::Box::new(__builtin_impl)
				}
			);
		}
	};
}

#[macro_export]
macro_rules! builtin {
	($library:expr, $name:expr, $($param:expr),*) => {
		$crate::builtin_alias!($library, $name, stringify!($name), $($param),*)
	};
}
