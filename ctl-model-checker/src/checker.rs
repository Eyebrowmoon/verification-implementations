use std::collections::{HashMap, HashSet};

use structure::{Model, State};
use ctl::{Proposition, Formula, TemporalFormula};
use ctl::FormulaEnum::*;
use ctl::TemporalFormulaEnum::*;

type Marking = HashSet<State>;

pub struct Checker {
	model: Model,
	property: Formula
}

impl Checker {
	pub fn new(model: Model, property: Formula) -> Checker {
		Checker { model: model, property: property }
	}
	
	fn marking_atomic(&self, id: &Proposition) -> Marking {
		self.model.states().iter().filter(|&state| {
			self.model.label(&state).contains(id)
		}).map(|&state| state.to_owned()).collect()
	}
	
	fn marking_and(&self, psi1: &Formula, psi2: &Formula) -> Marking {
		let marking_psi1 = self.marking(psi1);
		let marking_psi2 = self.marking(psi2);
		
		self.model.states().iter().filter(|&state| {
			let hold_psi1 = marking_psi1.contains(&state);
			let hold_psi2 = marking_psi2.contains(&state);
			
			hold_psi1 & hold_psi2
		}).map(|&state| state.to_owned()).collect()
	}
	
	fn marking_not(&self, psi: &Formula) -> Marking {
		let marking_psi = self.marking(psi);
		
		self.model.states().iter().filter(|&state| {
			 !marking_psi.contains(&state)
		}).map(|&state| state.to_owned()).collect()
	}
	
	fn marking_ex(&self, psi: &Formula) -> Marking {
		let marking_psi = self.marking(psi);
	
		self.model.states().iter().filter(|&state| {
			self.model.successors(&state).iter().any(|&succ| {
				marking_psi.contains(&succ)
			})
		}).map(|&state| state.to_owned()).collect()
	}
	
	fn marking_ax(&self, psi: &Formula) -> Marking {
		let marking_psi = self.marking(psi);
	
		self.model.states().iter().filter(|&state| {
			self.model.successors(&state).iter().all(|&succ| {
				marking_psi.contains(&succ)
			})
		}).map(|&state| state.to_owned()).collect()
	}
	
	fn marking_eu(&self, psi1: &Formula, psi2: &Formula) -> Marking {
		let marking_psi1 = self.marking(psi1);
		let marking_psi2 = self.marking(psi2);
		
		let states = self.model.states();
		
		let mut to_lookup = Vec::new();
		let mut marking = HashSet::new();
		let mut seen_before = HashSet::new();
		
		for state in states {
			let hold_psi2 = marking_psi2.contains(&state);
			
			if hold_psi2 {
				to_lookup.push(state.to_owned());
				seen_before.insert(state.to_owned());
			}
		}
		
		while !to_lookup.is_empty() {
			let state = to_lookup.pop().unwrap();
			let predecessors = self.model.predecessors(&state);
			
			marking.insert(state.to_owned());
			
			for pred in predecessors {
				let hold_psi1 = marking_psi1.contains(&pred);
				let pred_seen_before = seen_before.contains(&pred);
				
				seen_before.insert(pred.to_owned());
				
				if hold_psi1 & pred_seen_before {
					to_lookup.push(pred.to_owned());
				}
			}
		}
		
		marking
	}
	
	fn marking_au(&self, psi1: &Formula, psi2: &Formula) -> Marking {
		let marking_psi1 = self.marking(psi1);
		let marking_psi2 = self.marking(psi2);
		
		let states = self.model.states();
		
		let mut to_lookup = Vec::new();
		let mut marking = HashSet::new();
		let mut count: HashMap<State, i32> = states.iter().map(|&state| {
			(state.to_owned(), self.model.degree(&state))
		}).collect();
		
		state.iter()
			.filter(|&state| marking_psi2.contains(&state))
			.for_each(|&state| to_lookup.push(state.to_owned));
		
		while !to_lookup.is_empty() {
			let state = to_lookup.pop().unwrap();
			let predecessors = self.model.predecessors(&state);
						
			marking.insert(state.to_owned());
			
			for pred in predecessors {
				let hold_psi1 = marking_psi1.contains(&pred);
				
				match count.get_mut(&pred) {
					Some(count_val) => {
						*count_val -= 1;
						
						if (count_val.to_owned() == 0) & hold_psi1 {
							to_lookup.push(pred.to_owned());
						}
					}
					None => {}
				}
			}
		}
		
		marking
	}
		
	fn marking_e(&self, phi: &TemporalFormula) -> Marking {
		match **phi {
			X(ref psi) => self.marking_ex(&psi),
			U(ref psi1, ref psi2) => self.marking_eu(&psi1, &psi2)
		}
	}
	
	fn marking_a(&self, phi: &TemporalFormula) -> Marking {
		match **phi {
			X(ref psi) => self.marking_ax(&psi),
			U(ref psi1, ref psi2) => self.marking_au(&psi1, &psi2)
		}
	}
	
	fn marking(&self, phi: &Formula) -> Marking {
		match **phi {
			Atomic(id) => self.marking_atomic(&id),
			And(ref psi1, ref psi2) => self.marking_and(&psi1, &psi2),
			Not(ref psi) => self.marking_not(&psi),
			E(ref psi) => self.marking_e(&psi),
			A(ref psi) => self.marking_a(&psi)
		}
	}
	
	pub fn check(&self) -> bool {
		let marking = self.marking(&self.property);
		
		marking.contains(self.model.initial_state())
	}
}