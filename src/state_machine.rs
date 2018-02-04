#[derive(Debug)]
struct StateMachine {
    st: String,
}

// a tuple struct with one (implicit) field: a function that takes a mutable
// reference to a StateMachine and returns another StateFn.
struct StateFn(fn(&mut StateMachine) -> StateFn);

impl StateMachine {
    // Returns the next state from the start state: foo
    fn start(&mut self) -> StateFn {
        self.st = String::from("start");
        StateFn(Self::foo)
    }

    fn foo(&mut self) -> StateFn {
        self.st = String::from("foo");
        StateFn(Self::end)
    }

    fn end(&mut self) -> StateFn {
        self.st = String::from("end");
        StateFn(Self::end)
    }
}

pub fn sample() {
    let mut state_machine = StateMachine {
        st: String::from(""),
    };

    let mut state = StateFn(StateMachine::start);
    println!("{:?}", state_machine);

    state = state.step(&mut state_machine);
    println!("{:?}", state_machine);

    state.step(&mut state_machine);
    println!("{:?}", state_machine);
}

impl StateFn {
    fn step(&self, sm: &mut StateMachine) -> StateFn {
        self.0(sm)
    }
}
