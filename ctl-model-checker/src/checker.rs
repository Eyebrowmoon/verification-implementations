use std::collections::HashMap;
use structure::{Model, Proposition, StateId};
use ctl::{Formula, TemporalFormula};

type Marking = HashMap<i32, bool>;

pub struct Checker {
	model: Model,
	property: Formula
}

impl Checker {
	pub fn new(model: Model, property: Formula) -> Checker {
		Checker { model: model, property: property }
	}
	
	fn marking_atomic(&self, id: StateId) -> Marking {
		let mut marking = HashMap::new();
		
		for state_id in self.model.state_ids() {
			let owned_state_id = state_id.to_owned();
			let label = self.model.label(owned_state_id);
			
			marking.insert(owned_state_id, label.contains(state_id));
		}
		
		marking
	}
	
	fn marking_and(&self, psi1: Formula, psi2: Formula) -> Marking {
		let marking_psi1 = self.marking(psi1);
		let marking_psi2 = self.marking(psi2);
		
		let mut marking = HashMap::new();
		
		for state_id in self.model.state_ids() {
			let owned_state_id = state_id.to_owned();
			
			let hold_psi1 = marking_psi1.get(state_id).unwrap();
			let hold_psi2 = marking_psi2.get(state_id).unwrap();
			
			marking.insert(owned_state_id, hold_psi1 & hold_psi2);
		}
		
		marking
	}
	
	fn marking_not(&self, psi: Formula) -> Marking {
		let marking_psi = self.marking(psi);
		
		let mut marking = HashMap::new();
		
		for state_id in self.model.state_ids() {
			let owned_state_id = state_id.to_owned();	
			let hold_psi = marking_psi.get(state_id).unwrap();
			
			marking.insert(owned_state_id, !hold_psi);
		}
		
		marking
	}
	
	fn marking_ex(self, psi: Formula) -> Marking {
		let marking_psi = self.marking(psi);
		
		let mut marking = HashMap::new();
	
		for state_id in self.model.state_ids() {
			let owned_state_id = state_id.to_owned();
			let hold_phi = self.model.successors(state_id).iter().any(
				|&succ_id| marking_psi.get(&succ_id).unwrap().to_owned()
			);
			
			marking.insert(owned_state_id, hold_phi);
		}
		
		marking
	}
	
	fn marking_ax(self, psi: Formula) -> Marking {
		let marking_psi = self.marking(psi);
		
		let mut marking = HashMap::new();
	
		for state_id in self.model.state_ids() {
			let owned_state_id = state_id.to_owned();
			let hold_phi = self.model.successors(state_id).iter().all(
				|&succ_id| marking_psi.get(&succ_id).unwrap().to_owned()
			);
			
			marking.insert(owned_state_id, hold_phi);
		}
		
		marking
	}
	
	pub fn marking(&self, phi: Formula) -> Marking {
		HashMap::new()
	}
}