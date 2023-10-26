pub mod format_string;
pub mod grammar;
mod str_range;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::parser::grammar::*;

use format_string::FormatString;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
#[derive(Debug)]
pub struct Dog3Parser;

trait AST {
	fn build(entry: Pair<'_, Rule>) -> Self;
}

impl AST for String {
	fn build(entry: Pair<'_, Rule>) -> Self {
		entry.as_span().as_str().to_owned()
	}
}

impl AST for FormatString {
	fn build(entry: Pair<'_, Rule>) -> Self {
		FormatString::parse(
			entry.as_span().as_str(),
			entry.into_inner().next().unwrap().as_rule() != Rule::SQuoteString,
		)
	}
}

impl AST for Value {
	fn build(entry: Pair<'_, Rule>) -> Self {
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::String => return Value::String(AST::build(pair)),
				Rule::Block => return Value::Block(Box::new(AST::build(pair))),
				Rule::ControlStmt => return Value::ControlStatement(Box::new(AST::build(pair))),
				_ => unreachable!(),
			}
		}
		unreachable!()
	}
}

impl AST for ForStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		match entry.as_rule() {
			Rule::ForStmt => {
				let mut variable = String::new();
				let mut list = Value::String(FormatString::raw(""));
				let mut output = Value::String(FormatString::raw(""));
				let mut value_index = 0;
				for pair in entry.into_inner() {
					match pair.as_rule() {
						Rule::Identifier => variable = AST::build(pair),
						Rule::Value => {
							match value_index {
								0 => list = AST::build(pair),
								1 => output = AST::build(pair),
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
			Rule::ForSplitStmt => {
				let mut variable = String::new();
				let mut list = Value::String(FormatString::raw(""));
				let mut output = Value::String(FormatString::raw(""));
				let mut split = Value::String(FormatString::raw(" "));
				let mut value_index = 0;
				for pair in entry.into_inner() {
					match pair.as_rule() {
						Rule::Identifier => variable = AST::build(pair),
						Rule::Value => {
							match value_index {
								0 => list = AST::build(pair),
								1 => split = AST::build(pair),
								2 => output = AST::build(pair),
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
			_ => unreachable!(),
		}
	}
}

impl AST for IfStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut condition = Value::String(FormatString::raw(""));
		let mut output = Value::String(FormatString::raw(""));
		let mut value_index = 0;
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Value => {
					match value_index {
						0 => condition = AST::build(pair),
						1 => output = AST::build(pair),
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
}

impl AST for IfElseStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut condition = Value::String(FormatString::raw(""));
		let mut output_true = Value::String(FormatString::raw(""));
		let mut output_false = Value::String(FormatString::raw(""));
		let mut value_index = 0;
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Value => {
					match value_index {
						0 => condition = AST::build(pair),
						1 => output_true = AST::build(pair),
						2 => output_false = AST::build(pair),
						_ => unreachable!(),
					}
					value_index += 1
				}
				Rule::If | Rule::Else => continue,
				_ => unreachable!(),
			}
		}
		IfElseStatement {
			condition,
			output_true,
			output_false,
		}
	}
}

impl AST for WhileStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut condition = Value::String(FormatString::raw(""));
		let mut output = Value::String(FormatString::raw(""));
		let mut value_index = 0;
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Value => {
					match value_index {
						0 => condition = AST::build(pair),
						1 => output = AST::build(pair),
						_ => unreachable!(),
					}
					value_index += 1
				}
				Rule::While => continue,
				_ => unreachable!(),
			}
		}
		WhileStatement { condition, output }
	}
}

impl AST for ControlStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::ForStmt => return ControlStatement::ForStatement(AST::build(pair)),
				Rule::ForSplitStmt => return ControlStatement::ForStatement(AST::build(pair)),
				Rule::IfStmt => return ControlStatement::IfStatement(AST::build(pair)),
				Rule::IfElseStmt => return ControlStatement::IfElseStatement(AST::build(pair)),
				Rule::WhileStmt => return ControlStatement::WhileStatement(AST::build(pair)),
				_ => unreachable!(),
			}
		}
		unreachable!()
	}
}

impl AST for CommandStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut name = String::new();
		let mut parameters: Vec<ActualParameter> = vec![];
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Identifier => name = AST::build(pair),
				Rule::CommandArgs => {
					for pair in pair.into_inner() {
						match pair.as_rule() {
							Rule::Value => parameters.push(ActualParameter {
								value: AST::build(pair),
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
}

impl AST for SetStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut variable = String::new();
		let mut value = Value::String(FormatString::raw(""));
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Identifier => variable = AST::build(pair),
				Rule::Value => value = AST::build(pair),
				Rule::Equals => continue,
				_ => unreachable!(),
			}
		}
		SetStatement { variable, value }
	}
}

impl AST for ClearStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Value => {
					return ClearStatement {
						value: Some(AST::build(pair)),
					}
				}
				Rule::Clear => continue,
				_ => unreachable!(),
			}
		}
		ClearStatement { value: None }
	}
}

impl AST for ReturnStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Value => {
					return ReturnStatement {
						value: Some(AST::build(pair)),
					}
				}
				Rule::Return => continue,
				_ => unreachable!(),
			}
		}
		ReturnStatement { value: None }
	}
}

impl AST for OpenStatement {
	fn build(entry: Pair<'_, Rule>) -> Self {
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::CommandStmt => return OpenStatement::CommandStmt(AST::build(pair)),
				Rule::SetStmt => return OpenStatement::SetStmt(AST::build(pair)),
				Rule::ClearStmt => return OpenStatement::ClearStmt(AST::build(pair)),
				Rule::ReturnStmt => return OpenStatement::ReturnStmt(AST::build(pair)),
				_ => unreachable!(),
			}
		}
		unreachable!()
	}
}

impl AST for Function {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut name = String::new();
		let mut args: Vec<FormalParameter> = vec![];
		let mut block: Block = Block { executions: vec![] };
		let def = entry.as_str().to_owned();
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Identifier => name = AST::build(pair),
				Rule::FormalArgs => {
					let mut destroy = false;
					for pair in pair.into_inner() {
						match pair.as_rule() {
							Rule::Identifier => args.push(FormalParameter {
								name: AST::build(pair),
								vector: destroy,
							}),
							Rule::Destroy => destroy = true,
							Rule::Comma => continue,
							_ => unreachable!(),
						}
					}
				}
				Rule::Block => block = AST::build(pair),
				Rule::Fn | Rule::RPar | Rule::LPar => continue,
				_ => unreachable!(),
			}
		}
		Function {
			name,
			args,
			block,
			script: def,
		}
	}
}

impl AST for Block {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut executions: Vec<Execution> = vec![];
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::Executions => {
					for pair in pair.into_inner() {
						match pair.as_rule() {
							Rule::ControlStmt => {
								executions.push(Execution::ControlStatement(AST::build(pair)))
							}
							Rule::OpenStmt => {
								executions.push(Execution::OpenStatement(AST::build(pair)))
							}
							Rule::Block => executions.push(Execution::Block(AST::build(pair))),
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
}

impl AST for Program {
	fn build(entry: Pair<'_, Rule>) -> Self {
		let mut functions: Vec<Function> = vec![];
		let mut executions: Vec<Execution> = vec![];
		for pair in entry.into_inner() {
			match pair.as_rule() {
				Rule::ControlStmt => executions.push(Execution::ControlStatement(AST::build(pair))),
				Rule::OpenStmt => executions.push(Execution::OpenStatement(AST::build(pair))),
				Rule::Block => executions.push(Execution::Block(AST::build(pair))),
				Rule::Function => functions.push(AST::build(pair)),
				Rule::Semi | Rule::EOI => continue,
				_ => unreachable!(),
			}
		}
		Program {
			functions,
			executions,
		}
	}
}

pub fn parse(input: &str) -> Result<Program, Error<Rule>> {
	let mut pairs = Dog3Parser::parse(Rule::Program, input)?;
	let root = pairs.next().unwrap();
	Ok(AST::build(root))
}
