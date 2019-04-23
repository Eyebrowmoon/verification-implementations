use std::collections::HashSet;
use std::collections::HashMap;

type StateId = i32;
type Proposition = i32;
type TransitionLabel = i32;

pub struct State {
	id: StateId,
	successors: Vec<StateId>,
	predecessors: Vec<StateId>
}

pub struct Transition {
	origin: StateId,
	transition_label: TransitionLabel,
	destination: StateId
}

pub struct Model {
	initial_state: StateId,
	transitions: Vec<Transition>,
	labeling: HashMap<StateId, HashSet<Proposition>>,
	id_state_map: HashMap<StateId, State>
}

impl State {
	pub fn new(id: StateId) -> State {
		State {id: id, successors: Vec::new(), predecessors: Vec::new() }
	}
	
	pub fn id(&self) -> &StateId {
		&self.id
	}
	
	pub fn successors(&self) -> &Vec<StateId> {
		&self.successors
	}
	
	pub fn predecessors(&self) -> &Vec<StateId> {
		&self.predecessors
	}
	
	pub fn add_successor(&mut self, succ: StateId) {
		&self.successors.push(succ);
	}
	
	pub fn add_predecessor(&mut self, pred: StateId) {
		&self.predecessors.push(pred);
	}
	
	pub fn degree(&self) -> usize {
		self.successors.len()
	}
}

impl Model {
	pub fn new(initial_state: StateId, initial_label: HashSet<Proposition>) -> Model {
		let mut model = Model {
			initial_state: initial_state, 
			transitions: Vec::new(), 
			labeling: HashMap::new(),
			id_state_map: HashMap::new()
		};
		
		model.add_state(initial_state, initial_label);
		
		model
	}
	
	pub fn add_state(&mut self, id: StateId, label: HashSet<Proposition>) {
		let state = State::new(id);
		
		self.labeling.insert(id, label);
		self.id_state_map.insert(id, state);
	}
	
	pub fn add_transition(&mut self, trans: Transition) {
		let origin = trans.origin;
		let destination = trans.destination;
		
		self.transitions.push(trans);
		
		match self.id_state_map.get_mut(&origin) {
			Some(state) => state.add_successor(destination),
			None => panic!("Invalid transition: Failed to find origin"),
		}
		
		match self.id_state_map.get_mut(&destination) {
			Some(state) => state.add_predecessor(origin),
			None => panic!("Invalid transition: Failed to find destination"),
		}
	}
}