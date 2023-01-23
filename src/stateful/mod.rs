pub mod elements;
pub mod state_machine;

use elements::*;
use state_machine::*;

fn send<
    'sm_life,
    const STATE_COUNT: usize,
    const EVENT_COUNT: usize,
    const TRANSITION_COUNT: usize,
>(
    mut state_machine: BasicStateMachine<'sm_life, STATE_COUNT, EVENT_COUNT, TRANSITION_COUNT>,
    event: &Event,
) -> BasicStateMachine<'sm_life, STATE_COUNT, EVENT_COUNT, TRANSITION_COUNT> {
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
    fn reaches_the_correct_state_when_an_event_is_received() {
        fn test_action() {}

        const IDLE: State = State {
            id: 0,
            name: "IDLE",
        };
        const ACTIVE: State = State {
            id: 1,
            name: "ACTIVE",
        };

        const ACTIVATE: Event = Event {
            id: 0,
            name: "activate",
        };
        const DEACTIVATE: Event = Event {
            id: 1,
            name: "deactivate",
        };

        let mut sm = BasicStateMachine {
            states: [&IDLE, &ACTIVE],
            events: [&ACTIVATE, &DEACTIVATE],
            transitions: [
                Transition {
                    start: &IDLE,
                    event: &ACTIVATE,
                    action: test_action,
                    end: &ACTIVE,
                },
                Transition {
                    start: &ACTIVE,
                    event: &DEACTIVATE,
                    action: test_action,
                    end: &IDLE,
                },
            ],
            state: &IDLE,
        };

        assert_eq!(&IDLE, sm.state);

        sm = send(sm, &ACTIVATE);

        assert_eq!(&ACTIVE, sm.state);

        sm = send(sm, &DEACTIVATE);

        assert_eq!(&IDLE, sm.state);
    }
}
