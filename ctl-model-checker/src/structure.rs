use std::collections::HashSet;
use std::collections::HashMap;

use ctl::Proposition;

pub type State = i32;
pub type Label = HashSet<Proposition>;

pub struct Model {
	initial_state: State,
	states: HashSet<State>,
	labeling: HashMap<State, Label>,
	successors_map: HashMap<State, Vec<State>>,
	predecessors_map: HashMap<State, Vec<State>>
}

impl Model {
	pub fn new(initial_state: State, initial_label: Label) -> Model {
		let mut model = Model {
			initial_state: initial_state, 
			states: HashSet::new(),
			labeling: HashMap::new(),
			successors_map: HashMap::new(),
			predecessors_map: HashMap::new()
		};
		
		model.add_state(initial_state, initial_label);
		
		model
	}
	
	pub fn add_state(&mut self, state: State, label: Label) {
		self.states.insert(state);
		self.labeling.insert(state, label);
		self.successors_map.insert(state, Vec::new());
		self.predecessors_map.insert(state, Vec::new());
	}
	
	pub fn add_transition(&mut self, origin: State, destination: State) {	
		assert!(self.states.contains(&origin));
		assert!(self.states.contains(&destination));
		
		self.successors_map.get_mut(&origin).unwrap().push(destination);
		self.predecessors_map.get_mut(&destination).unwrap().push(origin);
	}
	
	pub fn label(&self, state: &State) -> &Label {
		&self.labeling.get(state).unwrap()
	}
	
	pub fn initial_state(&self) -> &State {
		&self.initial_state
	}
	
	pub fn states(&self) -> &HashSet<State> {
		&self.states
	}
	
	pub fn successors(&self, state: &State) -> &Vec<State>{
		&self.successors_map.get(state).unwrap()
	}
	
	pub fn degree(&self, state: &State) -> i32 {
		self.successors_map.get(state).unwrap().len() as i32
	}
	
	pub fn predecessors(&self, state: &State) -> &Vec<State>{
		&self.predecessors_map.get(state).unwrap()
	}
}