use crate::State;
use crate::Event;
use crate::Transition;

pub struct BasicStateMachine<'sm_life, const STATE_COUNT: usize, const EVENT_COUNT: usize, const TRANSITION_COUNT: usize> {
    pub states: [&'sm_life State<'sm_life>; STATE_COUNT],
    pub events: [&'sm_life Event<'sm_life>; EVENT_COUNT],
    pub transitions: [Transition<'sm_life>; TRANSITION_COUNT],

    pub state: &'sm_life State<'sm_life>
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_state_machine(){
        fn test_action() {}

        const IDLE: State = State{ id: 0, name: "IDLE"};
        const ACTIVE: State = State{ id: 1, name: "ACTIVE"};

        const ACTIVATE: Event = Event{id: 0, name: "activate"};
        const DEACTIVATE: Event = Event{id: 1, name: "deactivate"};

        let sm = BasicStateMachine{
            states: [&IDLE, &ACTIVE],
            events: [&ACTIVATE, &DEACTIVATE],
            transitions: [Transition{start: &IDLE, event: &ACTIVATE, action: test_action, end: &ACTIVE}],
            state: &IDLE 
        };

        assert_eq!(2, sm.states.len());
        assert_eq!(2, sm.events.len());
        assert_eq!(1, sm.transitions.len());
    }
}