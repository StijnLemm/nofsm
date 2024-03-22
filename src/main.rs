pub mod state {
    pub const OFF: usize = 0;
    pub const IDLE: usize = 1;
    pub const SELECTED: usize = 2;
    pub const STARTED: usize = 3;
}

pub mod event {
    pub const ON_OFF_BTN: usize = 0;
    pub const SELECT_COFFEE_BTN: usize = 1;
    pub const START_STOP_BTN: usize = 2;
    pub const COFFEE_DONE: usize = 3;
    pub const OFF_TIMEOUT: usize = 4;
}

const EVENT_AMOUNT: usize = 5;
const STATE_AMOUNT: usize = 4;

type FsmStorage = [[usize; STATE_AMOUNT]; EVENT_AMOUNT];

struct FSM {
    storage: FsmStorage,
}

impl FSM {
    fn new() -> Self {
        FSM {
            storage: [core::array::from_fn(|i| i); EVENT_AMOUNT],
        }
    }

    fn dump(&self) {
        print!("\t\t");
        for i in 0..(STATE_AMOUNT) {
            print!("State {i}\t\t");
        }
        print!("\n");

        for (i, row) in self.storage.iter().enumerate() {
            print!("Event {i:3}:\t");
            for state in row {
                print!("Goto: {state}\t\t");
            }
            print!("\n");
        }
    }
}

fn next_state(fsm: &FSM, current_state: usize, thrown_event: usize) -> usize {
    return fsm.storage[thrown_event][current_state];
}

struct Transition {
    from_state: usize,
    to_state: usize,
    when_event: usize,
}

impl Transition {
    fn new(from_state: usize, to_state: usize, when_event: usize) -> Self {
        Transition {
            from_state,
            to_state,
            when_event,
        }
    }
}

fn add_transition(fsm: &mut FSM, transition: &Transition) {
    fsm.storage[transition.when_event][transition.from_state] = transition.to_state;
}

fn add_transitions(fsm: &mut FSM, transitions: &[Transition]) {
    for transition in transitions {
        add_transition(fsm, &transition);
    }
}

fn main() {
    let mut fsm_table = FSM::new();
    let transitions = [
        // off
        Transition::new(state::OFF, state::IDLE, event::ON_OFF_BTN),
        // idle
        Transition::new(state::IDLE, state::SELECTED, event::SELECT_COFFEE_BTN),
        Transition::new(state::IDLE, state::OFF, event::ON_OFF_BTN),
        Transition::new(state::IDLE, state::OFF, event::OFF_TIMEOUT),
        // selected
        Transition::new(state::SELECTED, state::OFF, event::ON_OFF_BTN),
        Transition::new(state::SELECTED, state::OFF, event::OFF_TIMEOUT),
        Transition::new(state::SELECTED, state::IDLE, event::SELECT_COFFEE_BTN),
        Transition::new(state::SELECTED, state::STARTED, event::START_STOP_BTN),
        // started
        Transition::new(state::STARTED, state::OFF, event::ON_OFF_BTN),
        Transition::new(state::STARTED, state::OFF, event::OFF_TIMEOUT),
        Transition::new(state::STARTED, state::SELECTED, event::START_STOP_BTN),
        Transition::new(state::STARTED, state::SELECTED, event::COFFEE_DONE),
    ];

    add_transitions(&mut fsm_table, &transitions);
    fsm_table.dump();
}
