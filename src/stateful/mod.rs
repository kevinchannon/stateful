pub mod elements;
use elements::*;

pub struct StateMachine<'sm_life, const STATE_COUNT: usize, const EVENT_COUNT: usize, const TRANSITION_COUNT: usize> {
    pub states: [&'sm_life State<'sm_life>; STATE_COUNT],
    pub events: [&'sm_life Event<'sm_life>; EVENT_COUNT],
    pub transitions: [Transition<'sm_life>; TRANSITION_COUNT],

    pub state: &'sm_life State<'sm_life>
}

fn send<'sm_life, const STATE_COUNT: usize, const EVENT_COUNT: usize, const TRANSITION_COUNT: usize>(mut state_machine: StateMachine<'sm_life, STATE_COUNT, EVENT_COUNT, TRANSITION_COUNT>, event: &Event) -> StateMachine<'sm_life, STATE_COUNT, EVENT_COUNT, TRANSITION_COUNT> {
    for t in state_machine.transitions.iter() {
        if t.start == state_machine.state && t.event == event {
        (t.action)();
            state_machine.state = t.end;

            return state_machine;
        }
    }

    panic!("No transition for this event from this state!");
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

        let sm = StateMachine{
            states: [&IDLE, &ACTIVE],
            events: [&ACTIVATE, &DEACTIVATE],
            transitions: [Transition{start: &IDLE, event: &ACTIVATE, action: test_action, end: &ACTIVE}],
            state: &IDLE 
        };

        assert_eq!(2, sm.states.len());
        assert_eq!(2, sm.events.len());
        assert_eq!(1, sm.transitions.len());
    }

    #[test]
    fn reaches_the_correct_state_when_an_event_is_received() {
        fn test_action() {}

        const IDLE: State = State{ id: 0, name: "IDLE"};
        const ACTIVE: State = State{ id: 1, name: "ACTIVE"};

        const ACTIVATE: Event = Event{id: 0, name: "activate"};
        const DEACTIVATE: Event = Event{id: 1, name: "deactivate"};
        
        let mut sm = StateMachine{
            states: [
                &IDLE, 
                &ACTIVE
                ],
            events: [
                &ACTIVATE,
                &DEACTIVATE
                ],
            transitions: [
                Transition{start: &IDLE, event: &ACTIVATE, action: test_action, end: &ACTIVE},
                Transition{start: &ACTIVE, event: &DEACTIVATE, action: test_action, end: &IDLE}
                ],
            state: &IDLE
        };

        assert_eq!(&IDLE, sm.state);

        sm = send(sm, &ACTIVATE);

        assert_eq!(&ACTIVE, sm.state);

        sm = send(sm, &DEACTIVATE);

        assert_eq!(&IDLE, sm.state);

    }
}
