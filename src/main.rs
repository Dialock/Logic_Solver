
use std::io::{self, Write};


mod truth_tree;
mod val_inval;
mod truth_table_single;
mod truth_table_multi;


fn main() {
    
	let mut input = String::new();

	println!("Welcome to Logic Solver!");
	println!("########################\n");

	loop {

		input.clear();

		println!("Select a solver:");
		println!("(1) Truth Trees");
		println!("(2) Valid/Invalid");
		println!("(3) Truth Table (Single)");
		println!("(4) Truth Table (Multi)");
		println!("(Q) Exit");
		print!("> ");

		io::stdout().flush()
			.expect("flush failed!");

		io::stdin().read_line(&mut input)
    		.expect("Failed to read line");

		println!(" ");

		let input_trimmed = input.trim();
		//input.clear();
		if input_trimmed == "Q" { break; }

		//let choice = input_trimmed.parse().expect("");

		match input_trimmed.parse::<i32>() {
			Ok(choice) => match choice {
								1 => get_argument("multi", choice),
								2 => get_argument("multi", choice),
								3 => get_argument("single", choice),
								4 => get_argument("multi", choice),
								_ => println!("'{}' is not an option\n", choice)
							},
			Err(..) => println!("Invalid Input: {}", input_trimmed),

		};

	}

	println!("Goodbye!");

}

fn get_argument(s: &str, x: i32) {

	if s == "single" {
		truth_table_single::run_truth_table_single();
	} else {
		val_inval::run_val_inval();
		truth_tree::run_tree();
		truth_table_multi::run_true_multi();
	}


}

fn validate_string(s: &str) {

}
