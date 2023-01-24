

#[derive(PartialEq)]
#[derive(Debug)]
pub struct State<'sm_life, Action_T> {
    pub id: u32,
    pub name: &'sm_life str,
    pub on_entry: Action_T
}

impl<'sm_life, Action_T> State<'sm_life, Action_T> {
    pub fn new_with_entry(id: u32, name: &'sm_life str, on_entry: Action_T) -> Self {
        State{id: id, name: name, on_entry: on_entry}
    }

    pub fn new(id: u32, name: &'sm_life str) -> Self {
        State{id: id, name: name, on_entry: ||{}}
    }
}

#[derive(PartialEq)]
pub struct Event<'sm_life> {
    pub id: u32,
    pub name: &'sm_life str,
}

pub struct Transition<'sm_life, Action_T> {
    pub start: &'sm_life State<'sm_life, Action_T>,
    pub event: &'sm_life Event<'sm_life>,
    pub action: Action_T,
    pub end: &'sm_life State<'sm_life, Action_T>
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state_has_name_and_id() {
        fn entry_fn(){}

        let s = State::new(0, "IDLE");
        assert_eq!(0, s.id);
        assert_eq!("IDLE", s.name);
    }

    #[test]
    fn event_has_name_and_id() {
        let e = Event{ id: 0, name: "button press"};
        assert_eq!(0, e.id);
        assert_eq!("button press", e.name);
    }
}
