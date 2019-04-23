pub enum Formula {
	Atomic(i32),
	And(Box<Formula>, Box<Formula>),
	Not(Box<Formula>, Box<Formula>),
	E(Box<TemporalFormula>),
	A(Box<TemporalFormula>)
}

pub enum TemporalFormula {
	X(Box<Formula>),
	U(Box<Formula>, Box<Formula>)
}