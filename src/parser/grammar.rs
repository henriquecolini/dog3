use super::format_string::FormatString;

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
	WhileStatement(WhileStatement),
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
pub struct WhileStatement {
	pub(crate) condition: Value,
	pub(crate) output: Value,
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

impl Program {
	pub fn functions(&mut self) -> &mut Vec<Function> {
		&mut self.functions
	}
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
