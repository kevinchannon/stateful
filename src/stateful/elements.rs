type Action = Box<dyn Fn() -> ()>;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct State<'sm_life> {
    pub id: u32,
    pub name: &'sm_life str,
    pub on_entry: Action
}

impl<'sm_life> State<'sm_life> {
    pub fn new_with_entry(id: u32, name: &'sm_life str, on_entry: dyn Fn()->()) -> Self {
        State{id: id, name: name, on_entry: Box::new(on_entry)}
    }

    pub fn new(id: u32, name: &'sm_life str) -> Self {
        State{id: id, name: name, on_entry: Box::new(||{})}
    }
}

#[derive(PartialEq)]
pub struct Event<'sm_life> {
    pub id: u32,
    pub name: &'sm_life str,
}

pub struct Transition<'sm_life> {
    pub start: &'sm_life State<'sm_life>,
    pub event: &'sm_life Event<'sm_life>,
    pub action: Action,
    pub end: &'sm_life State<'sm_life>
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
