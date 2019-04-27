use std::rc::Rc;

pub type Proposition = i32;

pub type Formula = Rc<FormulaEnum>;
pub type TemporalFormula = Rc<TemporalFormulaEnum>;

pub enum FormulaEnum {
	Atomic(Proposition),
	And(Formula, Formula),
	Not(Formula),
	E(TemporalFormula),
	A(TemporalFormula)
}

pub enum TemporalFormulaEnum {
	X(Formula),
	U(Formula, Formula)
}