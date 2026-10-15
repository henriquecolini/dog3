# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`dog3` is an esoteric interpreted language (Rust + Bash flavored) written in Rust. The only data type is a string that carries a hidden integer **status code** (0 = truthy, anything else = falsy). Every statement "outputs" a string that is appended to its enclosing scope; a block `{ ... }` is a value whose content is everything output inside it. Output only reaches stdout if it bubbles up to the top-level program (or via `print`/`println`, which write directly). The README is the language reference.

## Commands

```sh
cargo build                                  # build
cargo run -- examples/hello.dog              # run a script
cargo run -- a.dog b.dog                     # multiple files are concatenated with "\n" and parsed as one program
cargo clippy                                 # lint
```

There is no test suite; the `.dog` files in `examples/` (especially `examples/features/`) serve as manual regression checks. When writing dog3 code (examples, README snippets), run it — common pitfalls: unquoted `,` `'` and other special chars are syntax errors, and a variable name extends to the next whitespace/special char (`$x!` is variable `x!`).

## Architecture

Pipeline (`src/main.rs`, a `#[tokio::main]` binary): read files → register builtin libraries → `parser::parse` → register user `fn`s into the library → `Runtime::execute(...).await` → print final output value. The runtime is async so it can be embedded in an async host (a Telegram bot): builtins, the library and scopes are `Send + Sync`, and loops call `tokio::task::yield_now()` so long scripts don't starve other tasks.

- **Parser** (`src/parser/`): pest grammar in `grammar.pest`; `mod.rs` converts pest pairs into the AST types in `grammar.rs` via the `AST` trait (one `impl AST for X` per node). Adding syntax means touching all three. Note the `for` split form is `for x in $list split <sep> { ... }`. String literals become `FormatString` (`format_string.rs`), which pre-splits into raw and `$variable` regions; single-quoted strings don't expand variables.
- **Runtime** (`src/runtime/mod.rs`): an async tree-walking interpreter (recursion via `async_recursion`) where every `execute_*` function returns a `Next` control-flow enum (`Proceed`, `Append`, `Return`, `Clear`, `Abort`). The `evaluate!`/`proceed!` macros propagate non-local control flow; `scoped!` pushes/pops a scope. Bare blocks/control statements used as statements are `.supress()`ed (their output is discarded) — this is what makes `{ pln hi }` print nothing while `put { pln hi }` does.
- **Scopes** (`scope.rs`): `ScopeStack` over a global `HashMap`. `set_var` writes to the nearest scope that already has the variable (else the innermost); `declare_var` always writes to the innermost. Function calls get a fresh `call_frame` that only shares globals.
- **Output** (`output.rs`): the string+status value type.
- **Functions** (`runtime/functions/`): `FunctionLibrary` (`mod.rs`) maps name → overloads, selected by argument count (`min_args..=max_args`; a trailing `%arg` makes it variadic and joins the rest with spaces). A new overload whose arg range collides with an existing one replaces it; user scripts may not overload builtins (`RegisterError::OverloadBuiltin`). User functions keep their source text so `src <name>` can print it.

## Adding builtins

Each module in `src/builtin/` exposes `pub fn build() -> FunctionLibrary`. Builtins are `async fn(&FunctionLibrary, &mut ScopeStack<'_>, Vec<Output>) -> Result<Output, ExecutionError>` (they can read and write the caller's variables through the `ScopeStack`), registered with the `builtin!(library, func, "arg1", "%rest")` macro (or `builtin_alias!` to register under a different name). Builtins that need host state (an API client, a database handle) use `builtin_state!`/`builtin_state_alias!`, which pass a cloned `Ctx` as the first argument (see `builtin_state` in `runtime/functions/builtin.rs`; the `BuiltIn` trait is the boxed, `Send + Sync` form). Register the same Rust fn multiple times with different arities for overloads, then match on the `args` slice length inside it. A new module must be added to `src/builtin/mod.rs` and merged in `register_libraries` in `src/main.rs`. Report bad input through the returned status code (e.g. `Output::new_falsy()`), not `Err`; `ExecutionError::Custom(message)` (what `panic` raises) stops the program and prints the message without the `error:` prefix. Document new functions in the module tables in `README.md`.

`std::eval` re-parses a string and runs it in a fresh `Runtime` with a clone of the current library; `std::src` prints function source (builtins show as `// built-in` stubs).
