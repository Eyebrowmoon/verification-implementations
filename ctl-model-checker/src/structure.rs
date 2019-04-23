use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::{Keys, Values};

pub type StateId = i32;
pub type Proposition = i32;
pub type Label = HashSet<Proposition>;

pub struct State {
	id: StateId,
	successors: Vec<StateId>,
	predecessors: Vec<StateId>
}

pub struct Model {
	initial_state: StateId,
	labeling: HashMap<StateId, Label>,
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
	pub fn new(initial_state: StateId, initial_label: Label) -> Model {
		let mut model = Model {
			initial_state: initial_state, 
			labeling: HashMap::new(),
			id_state_map: HashMap::new()
		};
		
		model.add_state(initial_state, initial_label);
		
		model
	}
	
	pub fn add_state(&mut self, id: StateId, label: Label) {
		let state = State::new(id);
		
		self.labeling.insert(id, label);
		self.id_state_map.insert(id, state);
	}
	
	pub fn add_transition(&mut self, origin: StateId, destination: StateId) {	
		match self.id_state_map.get_mut(&origin) {
			Some(state) => state.add_successor(destination),
			None => panic!("Invalid transition: Failed to find origin"),
		}
		
		match self.id_state_map.get_mut(&destination) {
			Some(state) => state.add_predecessor(origin),
			None => panic!("Invalid transition: Failed to find destination"),
		}
	}
	
	pub fn label(&self, state_id: StateId) -> &Label {
		&self.labeling.get(&state_id).unwrap()
	}
	
	pub fn initial_state(&self) -> &StateId {
		&self.initial_state
	}
	
	pub fn state_ids(&self) -> Keys<StateId, State> {
		self.id_state_map.keys()
	}
	
	pub fn states(&self) -> Values<StateId, State> {
		self.id_state_map.values()
	}
	
	pub fn successors(&self, state_id: &StateId) -> &Vec<StateId>{
		&self.id_state_map.get(state_id).unwrap().successors()
	}
	
	pub fn predecessors(&self, state_id: &StateId) -> &Vec<StateId>{
		&self.id_state_map.get(state_id).unwrap().predecessors()
	}
}