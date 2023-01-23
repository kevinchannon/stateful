mod stateful;
use stateful::elements::*;
use stateful::state_machine::*;

const FOO: State = State{id: 0, name: "foo"};
const BAR: Event = Event{id: 0, name: "bar"};

fn main() {
    
    println!("Hello, state {}", FOO.name);

    let sm = StateMachine{states: [&FOO], events: [&BAR], transitions: [], state: &FOO};
    println!("State machine with {} states and {} actions", sm.states.len(), sm.transitions.len());
}
