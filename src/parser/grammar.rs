use super::format_string::FormatString;

#[derive(Debug)]
pub struct Program {
	pub functions: Vec<Function>,
	pub executions: Vec<Execution>,
}

#[derive(Debug)]
pub struct Function {
	pub(crate) name: String,
	pub(crate) args: Vec<FormalParameter>,
	pub(crate) block: Block,
	pub(crate) script: String,
}

#[derive(Debug, Clone)]
pub struct FormalParameter {
	pub(crate) name: String,
	pub(crate) vector: bool,
}

#[derive(Debug, Clone)]
pub enum Execution {
	Block(Block),
	ControlStatement(ControlStatement),
	OpenStatement(OpenStatement),
}

#[derive(Debug, Clone)]
pub struct Block {
	pub(crate) executions: Vec<Execution>,
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
	pub(crate) variable: String,
	pub(crate) split: Option<Value>,
	pub(crate) list: Value,
	pub(crate) output: Value,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
	pub(crate) condition: Value,
	pub(crate) output: Value,
}

#[derive(Debug, Clone)]
pub struct IfElseStatement {
	pub(crate) condition: Value,
	pub(crate) output_true: Value,
	pub(crate) output_false: Value,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
	pub(crate) condition: Value,
	pub(crate) output: Value,
}

#[derive(Debug, Clone)]
pub struct SetStatement {
	pub(crate) variable: String,
	pub(crate) value: Value,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
	pub(crate) value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ClearStatement {
	pub(crate) value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct CommandStatement {
	pub(crate) name: String,
	pub(crate) parameters: Vec<ActualParameter>,
}

#[derive(Debug, Clone)]
pub struct ActualParameter {
	pub(crate) value: Value,
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
