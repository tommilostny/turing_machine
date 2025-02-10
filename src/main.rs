use std::io;
use turing_machine::compile_turing_machine;

pub mod turing_machine;

fn main() {
    let bin_inv_tm = compile_turing_machine(include_str!("../example_machines/binary_invert.tm"));
    let pow2_a_tm = compile_turing_machine(include_str!("../example_machines/pow2_a.tm"));
    let an_to_anbn_tm = compile_turing_machine(include_str!("../example_machines/an2anbn.tm"));
    let palindrome_tm = compile_turing_machine(include_str!("../example_machines/palindrome.tm"));
    loop {
        println!("Select a Turing machine to run:");
        println!("1. Binary Inverter");
        println!("2. Power of 2 'a's");
        println!("3. a^n to a^n b^n");
        println!("4. Palindrome");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut tm = match input.trim() {
            "1" => {
                println!("Enter binary number to invert:");
                bin_inv_tm.clone()
            },
            "2" => {
                println!("Enter number to check if the number of 'a's is a power of 2:");
                pow2_a_tm.clone()
            },
            "3" => {
                println!("Enter string to convert from a^n to a^n b^n:");
                an_to_anbn_tm.clone()
            },
            "4" => {
                println!("Enter string to check if it is a palindrome:");
                palindrome_tm.clone()
            },
            _ => break,
        };
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        tm.set_tape(input.trim());
        let result = tm.run(false);
        tm.print_tape();
        println!("{}, {}", result, tm.state);
        println!();
    }
}
