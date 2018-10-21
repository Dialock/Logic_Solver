
use std::io::{self, Write};

mod truth_tree;

fn main() {
    
	let mut input = String::new();

	println!("Welcome to Logic Solver!");
	println!("########################\n");

	loop {

		println!("Select a solver:");
		println!("(1) Truth Trees");
		println!("(Q) Exit");
		print!("> ");

		io::stdout().flush()
			.expect("flush failed!");


		io::stdin().read_line(&mut input)
    		.expect("Failed to read line");

		println!(" ");

		if input.trim() == "Q" { break; }

		if input.trim() == "1" { truth_tree::run_tree(); }

		input.clear();
	}

	println!("Goodbye!");


}
