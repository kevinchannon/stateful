pub struct State {
    pub id: u32,
    pub name: &'static str,
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
}
