pub struct State {
    pub id: u32,
    pub name: &'static str,
}

pub struct StateMachine<const STATE_COUNT: usize> {
    pub states: [State; STATE_COUNT]
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
        let sm = StateMachine{states: [State{id: 0, name: "IDLE"}, State{id: 1, name: "ACTIVE"}]};

        assert_eq!(2, sm.states.len());
    }
}
