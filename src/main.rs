mod stateful;
use stateful::elements::*;
use stateful::state_machine::*;

fn main() {
    let foo: State = State::new(0, "foo");
    let bar: Event = Event{id: 0, name: "bar"};

    println!("Hello, state {}", foo.name);

    let sm = BasicStateMachine{states: [&foo], events: [&bar], transitions: [], state: &foo};
    println!("State machine with {} states and {} actions", sm.states.len(), sm.transitions.len());
}
