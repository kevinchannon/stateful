mod stateful;

const FOO: stateful::State = stateful::State{id: 0, name: "foo"};
const BAR: stateful::Event = stateful::Event{id: 0, name: "bar"};

fn main() {
    
    println!("Hello, state {}", FOO.name);

    let sm = stateful::StateMachine{states: [&FOO], events: [&BAR], transitions: [], state: &FOO};
    println!("State machine with {} states and {} actions", sm.states.len(), sm.transitions.len());
}
