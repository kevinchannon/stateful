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

        let idle: State = State::new(0, "IDLE");
        let active: State = State::new(1, "ACTIVE");

        const ACTIVATE: Event = Event{id: 0, name: "activate"};
        const DEACTIVATE: Event = Event{id: 1, name: "deactivate"};

        let sm = BasicStateMachine{
            states: [&idle, &active],
            events: [&ACTIVATE, &DEACTIVATE],
            transitions: [Transition{start: &idle, event: &ACTIVATE, action: Box::new(test_action), end: &active}],
            state: &idle 
        };

        assert_eq!(2, sm.states.len());
        assert_eq!(2, sm.events.len());
        assert_eq!(1, sm.transitions.len());
    }

    #[test]
    fn entry_function_is_called_on_state_entry(){
        fn test_action() {}

        let mut called = false;
        let mut entry_spy = || called = true;

        let idle: State = State::new(0, "IDLE");
        let active: State = State::new_with_entry(1, "ACTIVE", entry_spy);
 
        const ACTIVATE: Event = Event{id: 0, name: "activate"};
        const DEACTIVATE: Event = Event{id: 1, name: "deactivate"};

        let sm = BasicStateMachine{
            states: [&idle, &active],
            events: [&ACTIVATE, &DEACTIVATE],
            transitions: [Transition{start: &idle, event: &ACTIVATE, action: test_action, end: &active}],
            state: &idle 
        };
    }
}