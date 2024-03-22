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

struct FSM<const STATE_COUNT: usize, const EVENT_COUNT: usize> {
    storage: [[usize; STATE_COUNT]; EVENT_COUNT],
}

impl<const STATE_COUNT: usize, const EVENT_COUNT: usize> FSM<STATE_COUNT, EVENT_COUNT> {
    fn new() -> Self {
        FSM {
            storage: [core::array::from_fn(|i| i); EVENT_COUNT],
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
        for i in 0..STATE_COUNT {
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

#[test]
fn test_turnstile() {
    const STATE_LOCKED: usize = 0;
    const STATE_UNLOCKED: usize = 1;

    const EVENT_PUSH: usize = 0;
    const EVENT_COIN: usize = 1;

    let fsm = FSM::<2, 2>::compile(&[
        Transition::new(STATE_LOCKED, STATE_UNLOCKED, EVENT_COIN),
        Transition::new(STATE_LOCKED, STATE_LOCKED, EVENT_PUSH),
        Transition::new(STATE_UNLOCKED, STATE_UNLOCKED, EVENT_COIN),
        Transition::new(STATE_UNLOCKED, STATE_LOCKED, EVENT_PUSH),
    ]);

    let next = fsm.get_next_state(STATE_LOCKED, EVENT_COIN);
    assert_eq!(next, STATE_UNLOCKED);

    let next = fsm.get_next_state(next, EVENT_COIN);
    assert_eq!(next, STATE_UNLOCKED);

    let next = fsm.get_next_state(next, EVENT_PUSH);
    assert_eq!(next, STATE_LOCKED);

    let next = fsm.get_next_state(next, EVENT_PUSH);
    assert_eq!(next, STATE_LOCKED);

    fsm.dump();
}

#[test]
fn test_coffee_machine() {
    const STATE_OFF: usize = 0;
    const STATE_IDLE: usize = 1;
    const STATE_SELECTED: usize = 2;
    const STATE_STARTED: usize = 3;

    const EVENT_ON_OFF_BTN: usize = 0;
    const EVENT_SELECT_COFFEE_BTN: usize = 1;
    const EVENT_START_STOP_BTN: usize = 2;
    const EVENT_COFFEE_DONE: usize = 3;
    const EVENT_OFF_TIMEOUT: usize = 4;

    let fsm = FSM::<4, 5>::compile(&[
        // off
        Transition::new(STATE_OFF, STATE_IDLE, EVENT_ON_OFF_BTN),
        // idle
        Transition::new(STATE_IDLE, STATE_SELECTED, EVENT_SELECT_COFFEE_BTN),
        Transition::new(STATE_IDLE, STATE_OFF, EVENT_ON_OFF_BTN),
        Transition::new(STATE_IDLE, STATE_OFF, EVENT_OFF_TIMEOUT),
        // selected
        Transition::new(STATE_SELECTED, STATE_OFF, EVENT_ON_OFF_BTN),
        Transition::new(STATE_SELECTED, STATE_OFF, EVENT_OFF_TIMEOUT),
        Transition::new(STATE_SELECTED, STATE_IDLE, EVENT_SELECT_COFFEE_BTN),
        Transition::new(STATE_SELECTED, STATE_STARTED, EVENT_START_STOP_BTN),
        // started
        Transition::new(STATE_STARTED, STATE_OFF, EVENT_ON_OFF_BTN),
        Transition::new(STATE_STARTED, STATE_OFF, EVENT_OFF_TIMEOUT),
        Transition::new(STATE_STARTED, STATE_SELECTED, EVENT_START_STOP_BTN),
        Transition::new(STATE_STARTED, STATE_SELECTED, EVENT_COFFEE_DONE),
    ]);

    let next = fsm.get_next_state(STATE_OFF, EVENT_START_STOP_BTN);
    assert_eq!(next, STATE_OFF);

    let next = fsm.get_next_state(next, EVENT_ON_OFF_BTN);
    assert_eq!(next, STATE_IDLE);

    let next = fsm.get_next_state(next, EVENT_SELECT_COFFEE_BTN);
    assert_eq!(next, STATE_SELECTED);

    let next = fsm.get_next_state(next, EVENT_SELECT_COFFEE_BTN);
    assert_eq!(next, STATE_IDLE);

    let next = fsm.get_next_state(next, EVENT_SELECT_COFFEE_BTN);
    assert_eq!(next, STATE_SELECTED);

    let next = fsm.get_next_state(next, EVENT_START_STOP_BTN);
    assert_eq!(next, STATE_STARTED);

    let next = fsm.get_next_state(next, EVENT_SELECT_COFFEE_BTN);
    assert_eq!(next, STATE_STARTED);
}

fn main() {
    const STATE_ONE: usize = 0;
    const STATE_TWO: usize = 1;

    const EVENT_ONE: usize = 0;
    const EVENT_TWO: usize = 1;

    let fsm = FSM::<2, 2>::compile(&[
        Transition::new(STATE_ONE, STATE_TWO, EVENT_TWO),
        Transition::new(STATE_TWO, STATE_ONE, EVENT_ONE),
    ]);

    let next = fsm.get_next_state(STATE_ONE, EVENT_ONE);
    assert_eq!(next, STATE_ONE);

    let next = fsm.get_next_state(next, EVENT_TWO);
    assert_eq!(next, STATE_TWO);

    let next = fsm.get_next_state(next, EVENT_TWO);
    assert_eq!(next, STATE_TWO);

    let next = fsm.get_next_state(next, EVENT_ONE);
    assert_eq!(next, STATE_ONE);

    fsm.dump();
}
