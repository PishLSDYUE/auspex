pub struct Transition<Value> {
  weight: u64,
  value: Value
}


pub struct State<Value> {
  transitions: Vec<Transition<Value>>
}

pub struct Chain<Value> {
  map: std::collections::HashMap<Value, State<Value>>
}

impl<Value> Transition<Value> {
  pub fn new(value: Value, weight: u64) {
    Transition {
      value,
      weight
    }
  }
}

impl<Value> State<Value> {
  pub fn total_weight(&self) {
    self.transitions.map(|t| t.weight).sum()
  }
  pub fn new() {
    State {
      transitions: Vec::new()
    }
  }
}

impl<Value> Chain<Value> {
  pub fn record(&mut self, from: Value, to: Value) {
    let state = match self.map.get_mut(&from) {
      Some(x) => x,
      None => {
        let new_state = State::new();
        self.map.insert(from, new_state);
        new_state
      }
    };
    match state.transitions.iter().find(|x|x.value == to) {
      Some(t) => {
        t.weight += 1;
      },
      None => {
        state.transitions.push(Transition::new(value, 1));
      }
    }
  }
  pub fn new() {
    Chain {
      map: HashMap::new()
    }
  }
}

/*
Transition: weight and target for state
State: contains some transitions with weights
Chain: map of values to States
*/
