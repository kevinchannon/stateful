mod stateless;

fn main() {
    let s = stateless::State{id: 0, name: "foo"};
    println!("Hello, state {}", s.name);

    let sm = stateless::StateMachine{states: [s]};
    println!("State machine with {} states", sm.states.len());
}
