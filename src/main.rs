const EVENT_AMOUNT: usize = 5;
const STATE_AMOUNT: usize = 5;

type FsmStorage = [[usize; STATE_AMOUNT]; EVENT_AMOUNT];

struct FSM {
    storage: FsmStorage,
}

impl FSM {
    fn new() -> Self {
        FSM {
            storage: [[0; STATE_AMOUNT]; EVENT_AMOUNT],
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

fn add_transition(fsm: &mut FSM, from_state: usize, to_state: usize, when_event: usize) {
    fsm.storage[when_event][from_state] = to_state;
}

fn main() {
    let mut fsm_table = FSM::new();
    add_transition(&mut fsm_table, 1, 4, 3);

    fsm_table.dump();

    let next_state = next_state(&fsm_table, 1, 3);
    println!("");
    println!("Transition 1 => {next_state} when event: 3");
}
