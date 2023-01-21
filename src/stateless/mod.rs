pub struct State {
    pub id: u32,
    pub name: &'static str,
}

pub type Action = fn(State) -> State;

pub struct StateMachine<const STATE_COUNT: usize, const ACTION_COUNT: usize> {
    pub states: [State; STATE_COUNT],
    pub actions: [Action; ACTION_COUNT]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_name_and_id() {
        let s = State{ id: 0, name: "IDLE"};
        assert_eq!(0, s.id);
        assert_eq!("IDLE", s.name);
    }

    #[test]
    fn create_state_machine(){
        fn test_action(start: State) -> State {
            start
        }

        let sm = StateMachine{
            states: [State{id: 0, name: "IDLE"}, State{id: 1, name: "ACTIVE"}],
            actions: [test_action]
        };

        assert_eq!(2, sm.states.len());
        assert_eq!(1, sm.actions.len());
    }
    }
}
