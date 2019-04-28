pub mod structure;
pub mod ctl;
pub mod checker;

use std::rc::Rc;

use checker::Checker;
use structure::Model;
use ctl::FormulaEnum::*;
use ctl::TemporalFormulaEnum::*;

fn main() {
	let state0_label = vec![0, 1].iter().cloned().collect();
	let state1_label = vec![1, 2].iter().cloned().collect();
	let state2_label = vec![2].iter().cloned().collect();
	
	let mut model = Model::new(0, state0_label);
	
	model.add_state(1, state1_label);
	model.add_state(2, state2_label);
	
	model.add_transition(0, 1);
	model.add_transition(1, 2);
	model.add_transition(2, 0);
	model.add_transition(1, 0);
	model.add_transition(2, 2);
	
	let checker = Checker::new(model);
	let true_formula = Rc::new(Not(
		Rc::new(And(
				Rc::new(Atomic(0)),
				Rc::new(Not(Rc::new(Atomic(0))))
			))
		));
	let property = Rc::new(E(
		Rc::new(U(
			Rc::new(E(
				Rc::new(X(
					Rc::new(Not(
						Rc::new(Atomic(0))
					))
				))
			)),
			Rc::new(A(
				Rc::new(U(
					true_formula, // Modeling of true by ~ (p0 /\ ~p0) 
					Rc::new(And(
						Rc::new(Atomic(1)),
						Rc::new(Atomic(2))
					))
				))
			)),
		)))); // E(EX ~p0 U AF (p1 /\ p2))
	
	let property2 = Rc::new(A(
		Rc::new(X(
			Rc::new(A(
				Rc::new(U(
					Rc::new(Atomic(1)),
					Rc::new(Atomic(0))
				))
			))
		))
	)); // AX A(p1 U p0)
	
	assert!(checker.check(&property) == true);
	assert!(checker.check(&property2) == false);
}