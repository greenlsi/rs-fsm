pub type Precondition<S> = fn(&S) -> bool;

pub type Activation<S> = fn(&mut S);

pub type Transition<S> = (Precondition<S>, Activation<S>);

pub struct Fsm<'a, S> {
    state: S,
    transition_table: &'a [Transition<S>],
}

impl<'a, S> Fsm<'a, S> {
    pub fn new(state: S, transition_table: &'a [Transition<S>]) -> Self {
        Fsm {
            state,
            transition_table,
        }
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn fire(&mut self) {
        for (condition, transition) in self.transition_table {
            if condition(&self.state) {
                transition(&mut self.state);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct State {
        min_val: i32,
        max_val: i32,
        count: i32,
        increasing: bool,
    }

    impl State {
        fn new(min_val: i32, max_val: i32, increasing: bool) -> Self {
            if max_val < min_val {
                panic!("invalid state")
            }
            match increasing {
                true => Self{min_val, max_val, count: min_val, increasing: true},
                false => Self{min_val, max_val, count: max_val, increasing: false}
            }
        }
    }

    fn needs_decrease(state: &State) -> bool {
        state.count >= state.max_val || !state.increasing && state.count > state.min_val
    }

    fn needs_increase(state: &State) -> bool {
        state.count <= state.min_val || state.increasing && state.count < state.max_val
    }

    fn decrement(state: &mut State) {
        state.increasing = false;
        state.count -= 1;
    }

    fn increment(state: &mut State) {
        state.increasing = true;
        state.count += 1;
    }

    #[test]
    fn test_fsm_increasing() {
        let (min_val, max_val, increasing) = (0, 5, true);
        let initial_state = State::new(min_val, max_val, increasing);
        let tt: Vec<Transition<State>> = vec![(needs_decrease, decrement), (needs_increase, increment)];
        let mut fsm = Fsm::new(initial_state, &tt);

        assert_eq!(min_val, fsm.get_state().min_val);
        assert_eq!(max_val, fsm.get_state().max_val);
        assert!(fsm.get_state().increasing);
        assert_eq!(min_val, fsm.get_state().count);

        for _ in 0..5 {
            for j in 1..max_val + 1 - min_val {
                fsm.fire();
                assert_eq!(min_val, fsm.get_state().min_val);
                assert_eq!(max_val, fsm.get_state().max_val);
                assert!(fsm.get_state().increasing);
                assert_eq!(min_val + j, fsm.get_state().count);
            }
            for j in 1..max_val + 1 - min_val {
                fsm.fire();
                assert_eq!(min_val, fsm.get_state().min_val);
                assert_eq!(max_val, fsm.get_state().max_val);
                assert!(!fsm.get_state().increasing);
                assert_eq!(max_val - j, fsm.get_state().count);
            }
        }
    }
}
