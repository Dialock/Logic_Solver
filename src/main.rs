
use std::io::{self, Write};
use std::collections::LinkedList;

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
		println!("*(1) Truth Trees");
		println!("*(2) Valid/Invalid");
		println!(" (3) Truth Table (Single)");
		println!("*(4) Truth Table (Multi)");
		println!("(Q) Exit");
		print!("> ");

		io::stdout().flush()
			.expect("flush failed!");

		io::stdin().read_line(&mut input)
    		.expect("Failed to read line");

		println!(" ");

		let input_trimmed = input.trim();

		if input_trimmed == "Q" { break; }


		match input_trimmed.parse::<i32>() {
			Ok(choice) => match choice {
								1 => get_arguments("multi", choice),
								2 => get_arguments("multi", choice),
								3 => get_arguments("single", choice),
								4 => get_arguments("multi", choice),
								_ => println!("'{}' is not an option\n", choice)
							},
			Err(..) => println!("Invalid Input: {}", input_trimmed),

		};

	}

	println!("Goodbye!");

}

fn get_arguments(s: &str, x: i32) {

	let mut line_num = 1;
	let mut input = String::new();

	let _premise_fore 		= "Input Premise (";
	let _premise_end		= ") 'C' for Conclusion: ";
	let _conc_fore			= "Input Conclusion(";
	let _conc_end			= "): ";

	let mut fore 			= _premise_fore;
	let mut end 			= _premise_end;

	let mut is_condition 	= false;

	let mut list: LinkedList<String> = LinkedList::new();

	println!("'ZZZ' to quit modules.");

	loop {

		if s == "multi" {
    		println!("-------------------------\n");
    		print!("{}({}){}", fore, line_num, end);
    	} 
    	else {
    		println!("-------------------------\n");
    		print!("Input Argument: ");
    	}

    	// this line is for when print! is used.
    	io::stdout().flush()
    	 .expect("flush failed!");

    	io::stdin().read_line(&mut input)
    		.expect("Failed to read line");

    	// exit 
    	if input.trim() == "ZZZ" { break; } 

    	// work with conclusion
    	if input.trim() == "C" {

    		// if there is more than one premise already
    		if line_num > 1 && !is_condition {
    			is_condition = true;
    			fore 	= _conc_fore;
    			end 	= _conc_end;
    		}

    	} else {

    		let _v = validate_string(&input);

			if _v {
				line_num += 1;
				if x == 3 { is_condition = true; }
			}

			if is_condition {
				if x == 1 {  }
				else if x == 2 {  }
				else if x == 3 { 
					truth_table_single::run_truth_table_single(&input); 
					break;
				}
				else if x == 4 {  }
				else {  }
    			//evalute_argument(&input);
    		}

    		input.clear();
    	}
	}

}

fn validate_string(s: &str) -> bool {

	let mut para_check: LinkedList<char> = LinkedList::new();

	if ()


	for c in s.chars() {
		if c == '(' {
			para_check.push_back(c);
		} else if c == ')' {
			if para_check.len() == 0 {
				println!("Parentheses not balanced.");
				return false;
			}
			para_check.pop_back();
		}

	}

	if para_check.len() > 0 {
		println!("Parentheses not balanced.");
		return false;
	} else {
		println!("Parentheses balanced.");
		return true;
	}

}
