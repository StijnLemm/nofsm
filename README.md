# nofsm
Pure rust Finite state machine. No dynamic memory allocation.

Example:
``` cpp
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
```

Output:
```
                State 0         State 1
Event   0:      Goto: 0         Goto: 0
Event   1:      Goto: 1         Goto: 1
```
