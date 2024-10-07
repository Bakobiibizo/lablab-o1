pub enum AgentState {
    Ingesting,
    Parsing,
    Generating,
    Validating,
    Completed,
}

pub struct StateMachine {
    current_state: AgentState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: AgentState::Ingesting,
        }
    }

    pub fn transition(&mut self, new_state: AgentState) {
        self.current_state = new_state;
        // TODO: Handle state-specific actions
    }
}