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

struct FSM {
    storage: FsmStorage,
}

impl FSM {
    fn new() -> Self {
        FSM {
            storage: [core::array::from_fn(|i| i); EVENT_AMOUNT],
        }
    }

    fn compile(transitions: &[Transition]) -> Self {
        let mut fsm = FSM::new();
        fsm.add_transitions(transitions);
        fsm
    }

    fn add_transition(&mut self, transition: &Transition) {
        self.storage[transition.when_event][transition.from_state] = transition.to_state;
    }

    fn add_transitions(&mut self, transitions: &[Transition]) {
        for transition in transitions {
            self.add_transition(&transition);
        }
    }

    fn get_next_state(&self, current_state: usize, thrown_event: usize) -> usize {
        self.storage[thrown_event][current_state]
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

fn main() {
    let fsm = FSM::compile(&[
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
    ]);

    let next = fsm.get_next_state(state::OFF, event::START_STOP_BTN);
    assert_eq!(next, state::OFF);

    let next = fsm.get_next_state(next, event::ON_OFF_BTN);
    assert_eq!(next, state::IDLE);

    let next = fsm.get_next_state(next, event::SELECT_COFFEE_BTN);
    assert_eq!(next, state::SELECTED);

    let next = fsm.get_next_state(next, event::SELECT_COFFEE_BTN);
    assert_eq!(next, state::IDLE);

    let next = fsm.get_next_state(next, event::SELECT_COFFEE_BTN);
    assert_eq!(next, state::SELECTED);

    let next = fsm.get_next_state(next, event::START_STOP_BTN);
    assert_eq!(next, state::STARTED);

    let next = fsm.get_next_state(next, event::SELECT_COFFEE_BTN);
    assert_eq!(next, state::STARTED);

    fsm.dump();
}
