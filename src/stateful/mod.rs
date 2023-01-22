pub struct State {
    pub id: u32,
    pub name: &'static str,
}

pub struct Event {
    pub id: u32,
    pub name: &'static str,
}

pub type Action = fn();

pub struct Transition{
    pub start: &'static State,
    pub event: &'static Event,
    pub action: Action,
    pub end: &'static State
}

pub struct StateMachine<const STATE_COUNT: usize, const EVENT_COUNT: usize, const TRANSITION_COUNT: usize> {
    pub states: [&'static State; STATE_COUNT],
    pub events: [&'static Event; EVENT_COUNT],
    pub transitions: [Transition; TRANSITION_COUNT],

    pub state: &'static State
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_has_name_and_id() {
        let s = State{ id: 0, name: "IDLE"};
        assert_eq!(0, s.id);
        assert_eq!("IDLE", s.name);
    }

    #[test]
    fn event_has_name_and_id() {
        let e = Event{ id: 0, name: "button press"};
        assert_eq!(0, e.id);
        assert_eq!("button press", e.name);
    }

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
            transitions: [Transition{start: &IDLE, event: &ACTIVATE, action: test_action, end: &ACTIVE}]
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
        
        let sm = StateMachine{
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
                ]
        };

        assert_eq!(IDLE, sm.state);
    }
}
