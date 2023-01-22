mod stateful;

const FOO: stateful::State = stateful::State{id: 0, name: "foo"};

fn main() {
    
    println!("Hello, state {}", FOO.name);

    let sm = stateful::StateMachine{states: [&FOO], actions: []};
    println!("State machine with {} states and {} actions", sm.states.len(), sm.actions.len());
}
