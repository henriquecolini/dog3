use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::time::Instant;

use crate::format_string::FormatString;

#[derive(Parser)]
#[grammar = "grammar.pest"]
#[derive(Debug)]
pub struct Dog3Parser;

#[derive(Debug)]
pub struct Program {
	pub(crate) functions: Vec<Function>,
	pub(crate) executions: Vec<Execution>,
}

#[derive(Debug)]
pub struct Function {
	pub(crate) name: String,
	pub(crate) args: Vec<FormalParameter>,
	pub(crate) block: Block,
}

#[derive(Debug)]
pub struct FormalParameter {
	pub(crate) name: String,
	pub(crate) vector: bool,
}

#[derive(Debug)]
pub enum Execution {
	Block(Block),
	ControlStatement(ControlStatement),
	OpenStatement(OpenStatement),
}

#[derive(Debug)]
pub struct Block {
	pub(crate) executions: Vec<Execution>,
}

#[derive(Debug)]
pub enum ControlStatement {
	ForStatement(ForStatement),
	IfStatement(IfStatement),
	IfElseStatement(IfElseStatement),
}

#[derive(Debug)]
pub enum OpenStatement {
	SetStmt(SetStatement),
	ReturnStmt(ReturnStatement),
	ClearStmt(ClearStatement),
	CommandStmt(CommandStatement),
}

#[derive(Debug)]
pub enum Value {
	String(FormatString),
	Block(Box<Block>),
	ControlStatement(Box<ControlStatement>),
}

#[derive(Debug)]
pub struct ForStatement {
	pub(crate) variable: String,
	pub(crate) split: Option<Value>,
	pub(crate) list: Value,
	pub(crate) output: Value,
}

#[derive(Debug)]
pub struct IfStatement {
	pub(crate) condition: Value,
	pub(crate) output: Value,
}

#[derive(Debug)]
pub struct IfElseStatement {
	pub(crate) condition: Value,
	pub(crate) output_true: Value,
	pub(crate) output_false: Value,
}

#[derive(Debug)]
pub struct SetStatement {
	pub(crate) variable: String,
	pub(crate) value: Value,
}

#[derive(Debug)]
pub struct ReturnStatement {
	pub(crate) value: Option<Value>,
}

#[derive(Debug)]
pub struct ClearStatement {
	pub(crate) value: Option<Value>,
}

#[derive(Debug)]
pub struct CommandStatement {
	pub(crate) name: String,
	pub(crate) parameters: Vec<ActualParameter>,
}

#[derive(Debug)]
pub struct ActualParameter {
	pub(crate) value: Value,
}

fn build_identifier(entry: Pair<'_, Rule>) -> String {
	entry.as_span().as_str().to_owned()
}

fn build_string(entry: Pair<'_, Rule>) -> FormatString {
	FormatString::parse(
		entry.as_span().as_str(),
		entry.into_inner().next().unwrap().as_rule() != Rule::SQuoteString,
	)
}

fn build_raw_string(entry: Pair<'_, Rule>) -> String {
	FormatString::parse(entry.as_span().as_str(), false).into()
}

fn build_value(entry: Pair<'_, Rule>) -> Value {
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::String => return Value::String(build_string(pair)),
			Rule::Block => return Value::Block(Box::new(build_block(pair))),
			Rule::ControlStmt => {
				return Value::ControlStatement(Box::new(build_control_stmt(pair)))
			}
			_ => unreachable!(),
		}
	}
	unreachable!()
}

fn build_for_stmt(entry: Pair<'_, Rule>) -> ForStatement {
	let mut variable = String::new();
	let mut list = Value::String(FormatString::raw(""));
	let mut output = Value::String(FormatString::raw(""));
	let mut value_index = 0;
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Identifier => variable = build_identifier(pair),
			Rule::Value => {
				match value_index {
					0 => list = build_value(pair),
					1 => output = build_value(pair),
					_ => unreachable!(),
				}
				value_index += 1
			}
			Rule::For | Rule::In | Rule::Split => continue,
			_ => unreachable!(),
		}
	}
	ForStatement {
		variable,
		list,
		output,
		split: None,
	}
}

fn build_for_split_stmt(entry: Pair<'_, Rule>) -> ForStatement {
	let mut variable = String::new();
	let mut list = Value::String(FormatString::raw(""));
	let mut output = Value::String(FormatString::raw(""));
	let mut split = Value::String(FormatString::raw(" "));
	let mut value_index = 0;
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Identifier => variable = build_identifier(pair),
			Rule::Value => {
				match value_index {
					0 => list = build_value(pair),
					1 => split = build_value(pair),
					2 => output = build_value(pair),
					_ => unreachable!(),
				}
				value_index += 1
			}
			Rule::For | Rule::In | Rule::Split => continue,
			_ => unreachable!(),
		}
	}
	ForStatement {
		variable,
		list,
		output,
		split: Some(split),
	}
}

fn build_if_stmt(entry: Pair<'_, Rule>) -> IfStatement {
	let mut condition = Value::String(FormatString::raw(""));
	let mut output = Value::String(FormatString::raw(""));
	let mut value_index = 0;
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Value => {
				match value_index {
					0 => condition = build_value(pair),
					1 => output = build_value(pair),
					_ => unreachable!(),
				}
				value_index += 1
			}
			Rule::If => continue,
			_ => unreachable!(),
		}
	}
	IfStatement { condition, output }
}

fn build_if_else_stmt(entry: Pair<'_, Rule>) -> IfElseStatement {
	let mut condition = Value::String(FormatString::raw(""));
	let mut output_true = Value::String(FormatString::raw(""));
	let mut output_false = Value::String(FormatString::raw(""));
	let mut value_index = 0;
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Value => {
				match value_index {
					0 => condition = build_value(pair),
					1 => output_true = build_value(pair),
					2 => output_false = build_value(pair),
					_ => unreachable!(),
				}
				value_index += 1
			}
			Rule::If => continue,
			_ => unreachable!(),
		}
	}
	IfElseStatement {
		condition,
		output_true,
		output_false,
	}
}

fn build_control_stmt(entry: Pair<'_, Rule>) -> ControlStatement {
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::ForStmt => return ControlStatement::ForStatement(build_for_stmt(pair)),
			Rule::ForSplitStmt => return ControlStatement::ForStatement(build_for_split_stmt(pair)),
			Rule::IfStmt => return ControlStatement::IfStatement(build_if_stmt(pair)),
			Rule::IfElseStmt => return ControlStatement::IfElseStatement(build_if_else_stmt(pair)),
			_ => unreachable!(),
		}
	}
	unreachable!()
}

fn build_command_stmt(entry: Pair<'_, Rule>) -> CommandStatement {
	let mut name = String::new();
	let mut parameters: Vec<ActualParameter> = vec![];
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Identifier => name = build_identifier(pair),
			Rule::CommandArgs => {
				for pair in pair.into_inner() {
					match pair.as_rule() {
						Rule::Value => parameters.push(ActualParameter {
							value: build_value(pair),
						}),
						_ => unreachable!(),
					}
				}
			}
			_ => unreachable!(),
		}
	}
	CommandStatement { name, parameters }
}

fn build_set_stmt(entry: Pair<'_, Rule>) -> SetStatement {
	let mut variable = String::new();
	let mut value = Value::String(FormatString::raw(""));
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Identifier => variable = build_identifier(pair),
			Rule::Value => value = build_value(pair),
			Rule::Equals => continue,
			_ => unreachable!(),
		}
	}
	SetStatement { variable, value }
}

fn build_clear_stmt(entry: Pair<'_, Rule>) -> ClearStatement {
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Value => {
				return ClearStatement {
					value: Some(build_value(pair)),
				}
			}
			Rule::Clear => continue,
			_ => unreachable!(),
		}
	}
	ClearStatement { value: None }
}

fn build_return_stmt(entry: Pair<'_, Rule>) -> ReturnStatement {
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Value => {
				return ReturnStatement {
					value: Some(build_value(pair)),
				}
			}
			Rule::Return => continue,
			_ => unreachable!(),
		}
	}
	ReturnStatement { value: None }
}

fn build_open_stmt(entry: Pair<'_, Rule>) -> OpenStatement {
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::CommandStmt => return OpenStatement::CommandStmt(build_command_stmt(pair)),
			Rule::SetStmt => return OpenStatement::SetStmt(build_set_stmt(pair)),
			Rule::ClearStmt => return OpenStatement::ClearStmt(build_clear_stmt(pair)),
			Rule::ReturnStmt => return OpenStatement::ReturnStmt(build_return_stmt(pair)),
			_ => unreachable!(),
		}
	}
	unreachable!()
}

fn build_function(entry: Pair<'_, Rule>) -> Function {
	let mut name = String::new();
	let mut args: Vec<FormalParameter> = vec![];
	let mut block: Block = Block { executions: vec![] };
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Identifier => name = build_identifier(pair),
			Rule::FormalArgs => {
				let mut destroy = false;
				for pair in pair.into_inner() {
					match pair.as_rule() {
						Rule::Identifier => args.push(FormalParameter {
							name: build_identifier(pair),
							vector: destroy,
						}),
						Rule::Destroy => destroy = true,
						Rule::Comma => continue,
						_ => unreachable!(),
					}
				}
			}
			Rule::Block => block = build_block(pair),
			Rule::Fn | Rule::RPar | Rule::LPar => continue,
			_ => unreachable!(),
		}
	}
	Function { name, args, block }
}

fn build_block(entry: Pair<'_, Rule>) -> Block {
	let mut executions: Vec<Execution> = vec![];
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::Executions => {
				for pair in pair.into_inner() {
					match pair.as_rule() {
						Rule::ControlStmt => {
							executions.push(Execution::ControlStatement(build_control_stmt(pair)))
						}
						Rule::OpenStmt => {
							executions.push(Execution::OpenStatement(build_open_stmt(pair)))
						}
						Rule::Block => executions.push(Execution::Block(build_block(pair))),
						Rule::Semi => continue,
						_ => unreachable!(),
					}
				}
			}
			Rule::RCurly | Rule::LCurly => continue,
			_ => unreachable!(),
		}
	}
	Block { executions }
}

fn build_program(entry: Pair<'_, Rule>) -> Program {
	let mut functions: Vec<Function> = vec![];
	let mut executions: Vec<Execution> = vec![];
	for pair in entry.into_inner() {
		match pair.as_rule() {
			Rule::ControlStmt => {
				executions.push(Execution::ControlStatement(build_control_stmt(pair)))
			}
			Rule::OpenStmt => executions.push(Execution::OpenStatement(build_open_stmt(pair))),
			Rule::Block => executions.push(Execution::Block(build_block(pair))),
			Rule::Function => functions.push(build_function(pair)),
			Rule::Semi | Rule::EOI => continue,
			_ => unreachable!(),
		}
	}
	Program {
		functions,
		executions,
	}
}

pub fn parse(input: &str) -> Result<Program, Error<Rule>> {
	let start = Instant::now();
	let mut pairs = Dog3Parser::parse(Rule::Program, input)?;
	let duration = start.elapsed();

	println!("[parser] Parse tree generated in {:?}", duration);

	let start = Instant::now();
	let root = pairs.next().unwrap();
	let program = build_program(root);
	let duration = start.elapsed();

	println!("[parser] Syntax tree generated in {:?}", duration);

	Ok(program)
}
