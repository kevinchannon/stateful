mod stateful;
use stateful::elements::*;

const FOO: State = State{id: 0, name: "foo"};
const BAR: Event = Event{id: 0, name: "bar"};

fn main() {
    
    println!("Hello, state {}", FOO.name);

    let sm = stateful::StateMachine{states: [&FOO], events: [&BAR], transitions: [], state: &FOO};
    println!("State machine with {} states and {} actions", sm.states.len(), sm.transitions.len());
}
