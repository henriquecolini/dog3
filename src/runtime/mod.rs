pub mod functions;
pub mod output;
pub mod scope;

use std::{collections::HashMap, fmt::Display};

use crate::parser::{format_string::*, grammar::*};

use async_recursion::async_recursion;
use functions::*;
use output::*;
use scope::ScopeStack;
use scope::*;

pub struct Runtime {
    pub library: FunctionLibrary,
    pub globals: Scope,
}

#[derive(Debug)]
pub enum RuntimeError {
    RegisterError(RegisterError),
    ExecutionError(ExecutionError),
}

pub enum Next {
    Proceed,
    Append(Output),
    Return(Output),
    Clear(Output),
    Abort(ExecutionError),
}

#[derive(Debug)]
pub enum ExecutionError {
    UndeclaredVariable(String),
    UndefinedFunction(String),
    UndefinedOverload(String, usize),
    InternalError,
    Custom(String),
}

macro_rules! scoped {
    ($stack:expr, $block:block) => {{
        $stack.push();
        let res = (async || $block)().await;
        $stack.pop();
        res
    }};
}

impl Next {
    fn supress(self) -> Next {
        if let Next::Append(_) = self {
            Next::Proceed
        } else {
            self
        }
    }
}

impl From<Result<Output, ExecutionError>> for Next {
    fn from(value: Result<Output, ExecutionError>) -> Self {
        match value {
            Ok(output) => Next::Append(output),
            Err(err) => Next::Abort(err),
        }
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::UndeclaredVariable(var) => {
                write!(f, "error: Use of undeclared variable `{}`", var)
            }
            ExecutionError::UndefinedFunction(func) => {
                write!(f, "error: Use of undefined function `{}`", func)
            }
            ExecutionError::UndefinedOverload(func, arg_c) => {
                write!(
                    f,
                    "error: no overload for function `{}` takes `{}` arguments",
                    func, arg_c
                )
            }
            ExecutionError::InternalError => write!(f, "error: Internal runtime error"),
            ExecutionError::Custom(err) => write!(f, "{err}"),
        }
    }
}

macro_rules! evaluate {
    ($x:expr) => {
        match $x {
            Next::Append(output) => output,
            Next::Proceed => unreachable!(),
            other => return other,
        }
    };
}

macro_rules! proceed {
    ($x:expr) => {
        match $x {
            Next::Proceed => Next::Proceed,
            other => return other,
        }
    };
}

async fn execute_string(stack: &mut ScopeStack<'_>, name: &FormatString) -> Next {
    let mut output = Output::new_truthy();
    for piece in name.into_iter() {
        match piece {
            FormatStringPiece::Raw(value) => output.append_str(value),
            FormatStringPiece::Variable(var) => match stack.get_var(var) {
                Some(value) => output.append(value),
                None => return Next::Abort(ExecutionError::UndeclaredVariable(var.into())),
            },
        }
    }
    Next::Append(output)
}

#[async_recursion]
async fn execute_value<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    value: &Value,
) -> Next {
    match value {
        Value::String(name) => execute_string(stack, name).await,
        Value::Block(block) => execute_block(functions, stack, block).await,
        Value::ControlStatement(control) => {
            execute_control_statement(functions, stack, control).await
        }
    }
}

#[async_recursion]
async fn execute_block<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    block: &Block,
) -> Next {
    scoped!(stack, {
        execute_statements(functions, stack, &block.executions).await
    })
}

async fn execute_open_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    open: &OpenStatement,
) -> Next {
    match &open {
        OpenStatement::SetStmt(stmt) => execute_set_statement(functions, stack, stmt).await,
        OpenStatement::ReturnStmt(stmt) => execute_return_statement(functions, stack, stmt).await,
        OpenStatement::ClearStmt(stmt) => execute_clear_statement(functions, stack, stmt).await,
        OpenStatement::CommandStmt(stmt) => execute_command_statement(functions, stack, stmt).await,
    }
}

async fn execute_set_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &SetStatement,
) -> Next {
    match execute_value(functions, stack, &stmt.value).await {
        Next::Append(output) => {
            stack.set_var(stmt.variable.as_str(), output);
            Next::Proceed
        }
        other => other,
    }
}

async fn execute_command_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &CommandStatement,
) -> Next {
    let func_list = functions.get_list(&stmt.name);
    let func_list = match func_list {
        Some(value) => value,
        None => return Next::Abort(ExecutionError::UndefinedFunction(stmt.name.to_owned())),
    };
    let count = stmt.parameters.len();
    let func = func_list
        .iter()
        .find(|x| x.max_args >= count && x.min_args <= count);
    let func = match func {
        Some(func) => func,
        None => {
            return Next::Abort(ExecutionError::UndefinedOverload(
                stmt.name.to_owned(),
                count,
            ))
        }
    };
    tokio::task::yield_now().await;
    let mut arg_values = vec![];
    for arg in &stmt.parameters {
        let output = evaluate!(execute_value(functions, stack, &arg.value).await);
        arg_values.push(output);
    }
    match func.runnable.as_ref() {
        Runnable::Block(block) => {
            let mut func_stack = ScopeStack::call_frame(stack);
            for arg in func.args.iter() {
                if arg.vector {
                    let joined_values = arg_values
                        .iter()
                        .map(|output| output.value())
                        .collect::<Vec<&str>>()
                        .join(" ");
                    let last_code = arg_values.last().map(|o| o.code()).unwrap_or(0);
                    func_stack.declare_var(&arg.name, Output::new(joined_values.into(), last_code));
                    break;
                } else {
                    func_stack.declare_var(&arg.name, arg_values.remove(0))
                }
            }
            let res = execute_block(functions, &mut func_stack, &block).await;
            let res = match res {
                Next::Return(out) => Next::Append(out),
                other => other,
            };
            return res;
        }
        Runnable::BuiltIn(builtin) => {
            return builtin.call(functions, stack, arg_values).await.into()
        }
    }
}

async fn execute_clear_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &ClearStatement,
) -> Next {
    Next::Clear(match &stmt.value {
        Some(value) => match execute_value(functions, stack, &value).await {
            Next::Append(output) => output,
            Next::Proceed => unreachable!(),
            other => return other,
        },
        None => Output::new_truthy(),
    })
}

async fn execute_return_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &ReturnStatement,
) -> Next {
    Next::Return(match &stmt.value {
        Some(value) => match execute_value(functions, stack, &value).await {
            Next::Append(output) => output,
            Next::Proceed => unreachable!(),
            other => return other,
        },
        None => Output::new_truthy(),
    })
}

async fn execute_for_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &ForStatement,
) -> Next {
    scoped!(stack, {
        let mut output = Output::new_truthy();
        let list = evaluate!(execute_value(functions, stack, &stmt.list).await);
        let split = match &stmt.split {
            None => None,
            Some(split) => Some(evaluate!(execute_value(functions, stack, split).await)),
        };
        for value in list.split_iter(split.as_ref()) {
            tokio::task::yield_now().await;
            stack.set_var(&stmt.variable, Output::new(value.to_owned().into(), 0));
            proceed!(scoped!(stack, {
                output.append(&evaluate!(
                    execute_value(functions, stack, &stmt.output).await
                ));
                Next::Proceed
            }));
        }
        Next::Append(output)
    })
}

async fn execute_if_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &IfStatement,
) -> Next {
    let condition = evaluate!(execute_value(functions, stack, &stmt.condition).await);
    if condition.is_truthy() {
        execute_value(functions, stack, &stmt.output).await
    } else {
        Next::Append(Output::new_falsy())
    }
}

async fn execute_if_else_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &IfElseStatement,
) -> Next {
    let condition = evaluate!(execute_value(functions, stack, &stmt.condition).await);
    if condition.is_truthy() {
        execute_value(functions, stack, &stmt.output_true).await
    } else {
        execute_value(functions, stack, &stmt.output_false).await
    }
}

async fn execute_while_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    stmt: &WhileStatement,
) -> Next {
    let mut output = Output::new_truthy();
    let mut condition = evaluate!(execute_value(functions, stack, &stmt.condition).await);
    while condition.is_truthy() {
        tokio::task::yield_now().await;
        proceed!(scoped!(stack, {
            output.append(&evaluate!(
                execute_value(functions, stack, &stmt.output).await
            ));
            Next::Proceed
        }));
        condition = evaluate!(execute_value(functions, stack, &stmt.condition).await);
    }
    Next::Append(output)
}

async fn execute_control_statement<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    control: &ControlStatement,
) -> Next {
    match control {
        ControlStatement::ForStatement(stmt) => execute_for_statement(functions, stack, stmt).await,
        ControlStatement::IfStatement(stmt) => execute_if_statement(functions, stack, stmt).await,
        ControlStatement::IfElseStatement(stmt) => {
            execute_if_else_statement(functions, stack, stmt).await
        }
        ControlStatement::WhileStatement(stmt) => {
            execute_while_statement(functions, stack, stmt).await
        }
    }
}

async fn execute_statements<'env, 'stack>(
    functions: &FunctionLibrary,
    stack: &'stack mut ScopeStack<'env>,
    execs: &[Execution],
) -> Next {
    let mut output = Output::new_truthy();
    for exec in execs.iter() {
        let next = match exec {
            Execution::Block(block) => execute_block(functions, stack, block).await.supress(),
            Execution::ControlStatement(control) => {
                execute_control_statement(functions, stack, control)
                    .await
                    .supress()
            }
            Execution::OpenStatement(open) => execute_open_statement(functions, stack, open).await,
        };
        match next {
            Next::Proceed => continue,
            Next::Append(out) => output.append(&out),
            Next::Clear(out) => output = out,
            other => return other,
        }
    }
    Next::Append(output)
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            library: FunctionLibrary::new(),
            globals: HashMap::new(),
        }
    }
    pub async fn execute(&mut self, execs: &[Execution]) -> Result<Output, ExecutionError> {
        let mut glob = ScopeStack::new(&mut self.globals);
        let res = execute_statements(&self.library, &mut glob, &execs).await;
        match res {
            Next::Append(output) => Ok(output),
            Next::Return(output) => Ok(output),
            Next::Abort(err) => Err(err),
            _ => Err(ExecutionError::InternalError),
        }
    }
    pub async fn execute_scoped<'env, 'stack>(
        &self,
        stack: &'stack mut ScopeStack<'env>,
        execs: &[Execution],
    ) -> Result<Output, ExecutionError> {
        let res = execute_statements(&self.library, stack, &execs).await;
        match res {
            Next::Append(output) => Ok(output),
            Next::Return(output) => Ok(output),
            Next::Abort(err) => Err(err),
            _ => Err(ExecutionError::InternalError),
        }
    }
}
