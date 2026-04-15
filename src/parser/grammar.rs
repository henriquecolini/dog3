use super::format_string::FormatString;

#[derive(Debug)]
pub struct Program {
	pub functions: Vec<Function>,
	pub executions: Vec<Execution>,
}

#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub args: Vec<FormalParameter>,
	pub block: Block,
	pub script: String,
}

#[derive(Debug, Clone)]
pub struct FormalParameter {
	pub name: String,
	pub vector: bool,
}

#[derive(Debug, Clone)]
pub enum Execution {
	Block(Block),
	ControlStatement(ControlStatement),
	OpenStatement(OpenStatement),
}

#[derive(Debug, Clone)]
pub struct Block {
	pub executions: Vec<Execution>,
}

#[derive(Debug, Clone)]
pub enum ControlStatement {
	ForStatement(ForStatement),
	IfStatement(IfStatement),
	IfElseStatement(IfElseStatement),
	WhileStatement(WhileStatement),
}

#[derive(Debug, Clone)]
pub enum OpenStatement {
	SetStmt(SetStatement),
	ReturnStmt(ReturnStatement),
	ClearStmt(ClearStatement),
	CommandStmt(CommandStatement),
}

#[derive(Debug, Clone)]
pub enum Value {
	String(FormatString),
	Block(Box<Block>),
	ControlStatement(Box<ControlStatement>),
}

#[derive(Debug, Clone)]
pub struct ForStatement {
	pub variable: String,
	pub split: Option<Value>,
	pub list: Value,
	pub output: Value,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
	pub condition: Value,
	pub output: Value,
}

#[derive(Debug, Clone)]
pub struct IfElseStatement {
	pub condition: Value,
	pub output_true: Value,
	pub output_false: Value,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
	pub condition: Value,
	pub output: Value,
}

#[derive(Debug, Clone)]
pub struct SetStatement {
	pub variable: String,
	pub value: Value,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
	pub value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ClearStatement {
	pub value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct CommandStatement {
	pub name: String,
	pub parameters: Vec<ActualParameter>,
}

#[derive(Debug, Clone)]
pub struct ActualParameter {
	pub value: Value,
}

impl FormalParameter {
	pub fn new(name: &str) -> FormalParameter {
		FormalParameter {
			name: name.to_owned(),
			vector: false,
		}
	}
	pub fn new_vector(name: &str) -> FormalParameter {
		FormalParameter {
			name: name.to_owned(),
			vector: true,
		}
	}
}
