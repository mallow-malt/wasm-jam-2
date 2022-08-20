use crate::planning::PlanningState;

pub struct Player {
    active: bool,
    planning_state: PlanningState,
}

impl Player {
    pub fn new() -> Self {
        Self {
            active: false,
            planning_state: PlanningState::new(),
        }
    }
}
