use std::fmt::Debug;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TapeCharacter {
    Blank,
    Symbol(char),
}

#[derive(Debug, Clone)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Write(TapeCharacter),
}

#[derive(Debug, Clone)]
pub struct Transition<TState> {
    pub state: TState,
    pub read: TapeCharacter,
    pub action: Action,
    pub next_state: TState,
}

#[derive(Debug, Clone)]
pub struct TuringMachine<TState> {
    pub tape: Vec<TapeCharacter>,
    pub head: usize,
    pub state: TState,
    pub transitions: Vec<Transition<TState>>,
    pub accepting_states: Vec<TState>,
}

impl<TState> TuringMachine<TState> where TState: PartialEq + Copy + Debug + std::fmt::Display {
    fn new(init_state: TState, accepting_states: Vec<TState>) -> Self {
        Self {
            tape: vec![TapeCharacter::Blank],
            head: 0,
            state: init_state,
            transitions: Vec::new(),
            accepting_states,
        }
    }

    pub fn set_tape(&mut self, tape: &str) {
        self.head = 0;
        self.tape = tape.chars().map(|c| TapeCharacter::Symbol(c)).collect();

        self.tape.insert(0, TapeCharacter::Blank);
        self.tape.push(TapeCharacter::Blank);
    }

    pub fn print_tape(&self) {
        print!("'");
        for (i, character) in self.tape.iter().enumerate() {
            if i == self.head {
                print!("|");
            }
            match character {
                TapeCharacter::Blank => print!("Δ"),
                TapeCharacter::Symbol(c) => print!("{}", c),
            }
        }
        println!("'");
    }

    pub fn run(&mut self, verbose: bool) -> bool {
        loop {
            let transition = self.transitions.iter().find(|transition| {
                transition.state == self.state && transition.read == self.tape[self.head]
            });
            if verbose {
                //print!("{}: {:?} ; ", self.state, transition);
                self.print_tape();
            }
            match transition {
                Some(transition) => {
                    match transition.action {
                        Action::MoveLeft => {
                            if self.head == 0 {
                                self.tape.insert(0, TapeCharacter::Blank);
                                continue;
                            }
                            self.head -= 1;
                        }
                        Action::MoveRight => {
                            self.head += 1;
                            if self.head == self.tape.len() {
                                self.tape.push(TapeCharacter::Blank);
                            }
                        }
                        Action::Write(character) => {
                            self.tape[self.head] = character;
                        },
                    };
                    self.state = transition.next_state;

                    if self.accepting_states.contains(&self.state) {
                        return true;
                    }
                }
                None => return false,
            }
        }
    }
}

pub fn compile_turing_machine(code: &str) -> TuringMachine<&str> {
    //split code by lines
    let mut lines: Vec<&str> = code.split("\n").collect();
    //delete all empty lines
    lines.retain(|line| line.trim().len() > 0);

    //first line defines the initial state: <state>
    let init_state = lines[0].trim();
    //second line defines the accepting states: <state1> <state2> ...
    let accepting_states: Vec<&str> = lines[1].split(" ").collect();
    //now create a new TuringMachine
    let mut tm = TuringMachine::new(init_state, accepting_states);

    //delete first two lines as they are already processed
    lines.remove(0);
    lines.remove(0);
    let lines = lines;

    //next lines define the transition functions:
    //ML <state> <next_state> <read> (read can be a symbol or a blank (empty))
    //MR <state> <next_state> <read> (read can be a symbol or a blank (empty))
    //WS <state> <next_state> <read> <write> (read symbol, write symbol)
    //WS <state> <next_state> <write> (read blank, write symbol)
    //WB <state> <next_state> <read> (write blank, read can be a symbol or a blank (empty))
    for line in lines {
        let mut func_def: Vec<&str> = line.trim().split(" ").collect();
        func_def.retain(|item| item.len() > 0);
        let func_def = func_def;

        if func_def.len() < 3 {
            panic!("Invalid instruction: {}", line);
        }

        tm.transitions.push(Transition {
            action: match func_def[0].trim() {
                "ML" => Action::MoveLeft,
                "MR" => Action::MoveRight,
                "WS" => match func_def.len(){
                    5 => Action::Write(TapeCharacter::Symbol(func_def[4].chars().next().unwrap())),
                    4 => Action::Write(TapeCharacter::Symbol(func_def[3].chars().next().unwrap())),
                    _ => panic!("Invalid definition: {:?}", func_def),
                }
                "WB" => Action::Write(TapeCharacter::Blank),
                _ => panic!("Invalid action: {}", func_def[0]),
            },
            state: func_def[1].trim(),
            next_state: func_def[2].trim(),
            read: match func_def.len() {
                5 => TapeCharacter::Symbol(func_def[3].chars().next().unwrap()),
                4 => match func_def[0].trim() {
                    "WS" => TapeCharacter::Blank,
                    _ => TapeCharacter::Symbol(func_def[3].chars().next().unwrap()),
                },
                _ => TapeCharacter::Blank,
            },
        });
    }
    return tm;
}
