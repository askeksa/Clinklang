
#[derive(Debug)]
pub struct Program<'input> {
	pub declarations: Vec<Declaration<'input>>,
}

#[derive(Debug)]
pub enum Declaration<'input> {
	Procedure {
		kind: ProcedureKind,
		name: Id<'input>,
		params: Pattern<'input>,
		outputs: Pattern<'input>,
		body: Vec<Statement<'input>>,
	},
}

#[derive(Debug)]
pub enum ProcedureKind {
	Function,
	Instrument,
}

#[derive(Debug)]
pub enum Statement<'input> {
	Assign { node: Pattern<'input>, exp: Expression<'input> },
}

pub type Pattern<'input> = Vec<PatternItem<'input>>;

#[derive(Debug)]
pub struct PatternItem<'input> {
	pub variable: PatternVariable<'input>,
	pub scope: Option<Scope<'input>>,
	pub width: Option<Width<'input>>,
	pub value_type: Option<ValueType>,
}

#[derive(Debug)]
pub enum PatternVariable<'input> {
	Variable { name: Id<'input> },
	Split { left: Id<'input>, right: Id<'input> },
}

#[derive(Debug)]
pub enum Scope<'input> {
	Static,
	Dynamic,
	Generic { name: Id<'input> },
}

#[derive(Debug)]
pub enum Width<'input> {
	Mono,
	Stereo,
	Generic { name: Id<'input> },
}

#[derive(Debug)]
pub enum ValueType {
	Number,
	Bool,
	Buffer,
}

#[derive(Debug)]
pub enum Expression<'input> {
	Number { value: f64 },
	Variable { name: Id<'input> },
	UnOp { op: UnOpKind, exp: Box<Expression<'input>> },
	BinOp { left: Box<Expression<'input>>, op: BinOpKind, right: Box<Expression<'input>> },
	Call { name: Id<'input>, args: Vec<Expression<'input>> },
	Tuple { elements: Vec<Expression<'input>> },
	Merge { left: Box<Expression<'input>>, right: Box<Expression<'input>> },
	Property { exp: Box<Expression<'input>>, name: Id<'input> },
	TupleIndex { exp: Box<Expression<'input>>, index: u64 },
	BufferIndex {exp: Box<Expression<'input>>, index: Box<Expression<'input>> },
}

#[derive(Debug)]
pub enum UnOpKind {
	Neg, Not,
}

#[derive(Debug)]
pub enum BinOpKind {
	Add, Sub, Mul, Div, And, Or, Xor, Eq, Neq, Less, LessEq, Greater, GreaterEq,
}

#[derive(Debug)]
pub struct Id<'input> {
	pub pos: usize,
	pub text: &'input str,
}
