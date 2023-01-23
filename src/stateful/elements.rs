#[derive(PartialEq)]
#[derive(Debug)]
pub struct State<'sm_life> {
    pub id: u32,
    pub name: &'sm_life str,
}

#[derive(PartialEq)]
pub struct Event<'sm_life> {
    pub id: u32,
    pub name: &'sm_life str,
}

pub type Action = fn();

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
}
